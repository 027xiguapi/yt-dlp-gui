<script setup lang="ts">
import { ref, onMounted, h, computed } from "vue";
import { useI18n } from "vue-i18n";
import { invoke } from "@tauri-apps/api/core";
import { getCurrentWindow } from "@tauri-apps/api/window";
import {
  NButton, NInput, NCard, NSpace, NProgress,
  NIcon, NTag, NDataTable, NPopconfirm, NTooltip, useMessage
} from "naive-ui";
import { Trash2, Copy, Play, CheckCircle2, FolderOpen, Download } from "@lucide/vue";
import { useConfigStore } from "../stores/configStore";
import { useDownloadStore } from "../stores/downloadStore";

interface VideoInfoResult {
  title: string;
  thumbnail: string;
  thumbnails: { url: string; width?: number; height?: number }[];
}

const { t } = useI18n();
const configStore = useConfigStore();
const downloadStore = useDownloadStore();
const message = useMessage();

const urlInput = ref("");
const checkedRowKeys = ref<string[]>([]);
const fetchingInfo = ref(false);
const searchKeyword = ref("");

// 搜索过滤 + 分页
const filteredTaskList = computed(() => {
  const keyword = searchKeyword.value.trim().toLowerCase();
  if (!keyword) return downloadStore.taskList;
  return downloadStore.taskList.filter(
    task => task.title.toLowerCase().includes(keyword) || task.url.toLowerCase().includes(keyword)
  );
});

const pagination = ref({
  page: 1,
  pageSize: 10,
  showSizePicker: true,
  pageSizes: [10, 20, 50],
  prefix: ({ itemCount }: { itemCount: number | undefined }) => `共 ${itemCount ?? 0} 条`,
});

// 格式化速度，保留一位小数
function formatSpeed(speed: string): string {
  if (!speed) return '';

  // 匹配数字部分
  const match = speed.match(/[\d.]+/);
  if (match) {
    const num = parseFloat(match[0]);
    const unit = speed.replace(/[\d.]/g, '').trim();
    return `${num.toFixed(1)}${unit}`;
  }

  return speed;
}

onMounted(async () => {
  await configStore.loadConfig();
  await downloadStore.initializeFromDatabase();

  const appWindow = await getCurrentWindow();
  await appWindow.listen<any>("download_progress", (event) => {
    const data = event.payload;
    downloadStore.updateTask(data.id, {
      status: data.status,
      progress: data.progress ? Number(parseFloat(data.progress).toFixed(2)) : undefined,
      speed: data.speed ? formatSpeed(data.speed) : undefined,
      eta: data.eta,
      size: data.size,
      title: data.title,
      error: data.error,
    });
  });
});

// --- 逻辑处理 ---

async function fetchVideoInfo(url: string): Promise<VideoInfoResult | null> {
  try {
    const info = await invoke<VideoInfoResult>("get_video_info", {
      url,
      ytdlpPath: configStore.ytdlpPath || null,
    });
    return info;
  } catch (err) {
    console.warn(`获取视频信息失败 ${url}:`, err);
    return null;
  }
}

async function addUrls() {
  if (!urlInput.value.trim() || !configStore.downloadPath) {
    message.warning(t("main.noUrlError"));
    return;
  }

  const urls = urlInput.value
    .split("\n")
    .map(u => u.trim())
    .filter(u => {
      if (!u) return false;
      try {
        new URL(u);
        return true;
      } catch {
        return false;
      }
    });

  if (urls.length === 0) {
    message.error(t("main.noValidUrl"));
    return;
  }

  fetchingInfo.value = true;

  // 先获取每个视频的详情
  const infoMap = new Map<string, VideoInfoResult>();
  for (const url of urls) {
    const info = await fetchVideoInfo(url);
    if (info) {
      infoMap.set(url, info);
    }
  }

  // 带详情信息添加任务
  for (const url of urls) {
    const info = infoMap.get(url);
    await downloadStore.addTask(
      url,
      configStore.selectedPreset,
      configStore.downloadPath,
      info ? { title: info.title, thumbnail: info.thumbnail } : undefined,
    );
  }

  fetchingInfo.value = false;
  urlInput.value = "";
  message.success(t("main.addSuccess", { count: urls.length }));
  console.log(urls, infoMap)

  // // 自动批量下载新添加的任务
  // const newTasks = downloadStore.taskList.filter(t => t.status === "Queued");
  // for (const task of newTasks) {
  //   await runDownloadTask(task);
  // }
}

// 执行单个下载任务
async function runDownloadTask(task: any) {
  if (task.status !== "Queued" && task.status !== "ERROR") return;

  downloadStore.updateTask(task.id, { status: "Processing" });
  try {
    await invoke("start_download", {
      task,
      cookiePath: configStore.cookiePath || null,
      ytdlpPath: configStore.ytdlpPath || null,
      cookiesFromBrowser: configStore.cookieBrowser || null,
    });
  } catch (error) {
    console.error(`下载启动失败 ${task.id}:`, error);
    downloadStore.updateTask(task.id, { status: "ERROR" });
  }
}

// 批量下载选中
async function startSelectedDownloads() {
  const selectedTasks = downloadStore.taskList.filter(
    t => checkedRowKeys.value.includes(t.id) && (t.status === "Queued" || t.status === "ERROR")
  );
  
  for (const task of selectedTasks) {
    runDownloadTask(task);
  }
}

// 批量删除选中
function deleteSelectedRows() {
  checkedRowKeys.value.forEach(id => downloadStore.removeTask(id));
  checkedRowKeys.value = [];
  message.info("已删除选中任务");
}

async function copyUrl(url: string) {
  try {
    await navigator.clipboard.writeText(url);
    message.info("链接已复制");
  } catch (err) {
    message.error("复制失败");
  }
}

async function openDownloadFolder() {
  try {
    await invoke("open_download_folder", { path: configStore.downloadPath });
    message.success("已打开下载文件夹");
  } catch (err) {
    message.error("打开文件夹失败");
  }
}

function getStatusType(status: string) {
  switch (status) {
    case "Finished": return "success";
    case "Processing":
    case "Downloading": return "info";
    case "Converting": return "warning";
    case "ERROR": return "error";
    default: return "default";
  }
}

// --- 表格列定义 ---

const columns = [
  {
    type: 'selection' as const,
  },
  {
    title: '标题',
    key: 'title',
    width: 220,
    ellipsis: { tooltip: true },
    render(row: any) {
      const displayText = row.title || row.url;
      const children: any[] = [];

      if (row.thumbnail) {
        children.push(
          h('img', {
            src: row.thumbnail,
            style: 'width: 64px; height: 36px; object-fit: cover; border-radius: 4px; flex-shrink: 0; margin-right: 8px;',
          })
        );
      }

      children.push(
        h('span', {
          style: row.thumbnail ? 'overflow: hidden; text-overflow: ellipsis; white-space: nowrap;' : '',
          title: displayText,
        }, displayText)
      );

      return h('div', {
        style: 'display: flex; align-items: center;',
      }, children);
    }
  },
  {
    title: '状态',
    key: 'status',
    width: 120,
    render(row: any) {
      const tag = h(NTag, { type: getStatusType(row.status), bordered: false }, { default: () => row.status });
      if (row.status === 'ERROR' && row.error) {
        return h(NTooltip, { trigger: 'hover' }, {
          trigger: () => tag,
          default: () => row.error,
        });
      }
      return tag;
    }
  },
  {
    title: '进度',
    key: 'progress',
    width: 100,
    render(row: any) {
      if (["Downloading", "Processing", "Converting"].includes(row.status)) {
        return h(NProgress, { 
          type: 'line', 
          percentage: row.progress || 0, 
          status: row.status === 'ERROR' ? 'error' : 'default',
          indicatorPlacement: 'inside' 
        });
      }
      return row.status === 'Finished' ? h(NIcon, { color: '#18a058', size: 20 }, { default: () => h(CheckCircle2) }) : '-';
    }
  },
  {
    title: '速度/大小',
    key: 'info',
    width: 150,
    render(row: any) {
      return h('div', { style: 'font-size: 12px' }, [
        h('div', row.speed ? `速度: ${row.speed}` : ''),
        h('div', row.size ? `大小: ${row.size}` : '')
      ]);
    }
  },
  {
    title: '操作',
    key: 'actions',
    width: 220,
    align: 'center' as const,
    fixed: 'right' as const,
    render(row: any) {
      return h(NSpace, { justify: 'center' }, {
        default: () => [
          // 开始按钮
          h(NButton, {
            quaternary: true,
            circle: true,
            type: 'primary',
            disabled: !["Queued", "ERROR"].includes(row.status),
            onClick: () => runDownloadTask(row)
          }, { default: () => h(NIcon, null, { default: () => h(Download) }) }),

          // 复制按钮
          h(NButton, {
            quaternary: true,
            circle: true,
            onClick: () => copyUrl(row.url)
          }, { default: () => h(NIcon, null, { default: () => h(Copy) }) }),

          // 打开文件夹按钮
          h(NButton, {
            quaternary: true,
            circle: true,
            type: 'info',
            onClick: () => openDownloadFolder()
          }, { default: () => h(NIcon, null, { default: () => h(FolderOpen) }) }),

          // 删除按钮
          h(NPopconfirm, {
            onPositiveClick: () => {
              downloadStore.removeTask(row.id);
              checkedRowKeys.value = checkedRowKeys.value.filter(k => k !== row.id);
            }
          }, {
            trigger: () => h(NButton, { quaternary: true, circle: true, type: 'error' }, {
              default: () => h(NIcon, null, { default: () => h(Trash2) })
            }),
            default: () => '确定删除此任务吗？'
          })
        ]
      });
    }
  }
];
</script>

<template>
  <div class="h-screen w-full bg-gray-50 overflow-y-auto overflow-x-auto" style="min-width: 500px">
    <n-space vertical :size="20" class="p-6">
      <div class="mb-2">
        <h1 class="text-3xl font-bold bg-gradient-to-r from-blue-500 to-purple-500 bg-clip-text text-transparent">{{ t('main.title') }}</h1>
        <p class="text-gray-600 text-sm mt-1">{{ t('main.subtitle') }}</p>
      </div>

      <n-card hoverable>
        <n-space vertical :size="12">
          <n-input
            v-model:value="urlInput"
            type="textarea"
            :placeholder="t('main.pasteUrl')"
            :autosize="{ minRows: 3, maxRows: 6 }"
          />
          <n-button type="primary" size="large" @click="addUrls" block :loading="fetchingInfo">
            {{ fetchingInfo ? t('channelExtraction.extracting') : t('main.addQueue') }}
          </n-button>
        </n-space>
      </n-card>

      <n-card :title="t('main.downloadQueue')" :segmented="{ content: true }">
        <template #header-extra>
          <n-space>
            <n-input
              v-model:value="searchKeyword"
              :placeholder="t('main.searchPlaceholder')"
              clearable
              style="width: 200px"
            />
            <n-button
              secondary
              type="primary"
              @click="startSelectedDownloads"
              :disabled="checkedRowKeys.length === 0"
            >
              {{ t('main.startSelected') }} ({{ checkedRowKeys.length }})
            </n-button>
            <n-button
              secondary
              type="error"
              @click="deleteSelectedRows"
              :disabled="checkedRowKeys.length === 0"
            >
              {{ t('main.deleteSelected') }}
            </n-button>
            <n-button quaternary @click="downloadStore.clearCompleted">
              {{ t('main.clearCompleted') }}
            </n-button>
          </n-space>
        </template>

        <n-data-table
          ref="table"
          :columns="columns"
          :data="filteredTaskList"
          :row-key="(row) => row.id"
          v-model:checked-row-keys="checkedRowKeys"
          :pagination="pagination"
          :max-height="500"
        />
      </n-card>
    </n-space>
  </div>
</template>

<style scoped>
:deep(.n-data-table-table) {
  font-variant-numeric: tabular-nums;
}

:deep(.n-data-table-wrapper) {
  overflow-x: auto;
}

::-webkit-scrollbar {
  height: 8px;
  width: 8px;
}

::-webkit-scrollbar-track {
  background: transparent;
}

::-webkit-scrollbar-thumb {
  background: #ccc;
  border-radius: 4px;
}

::-webkit-scrollbar-thumb:hover {
  background: #999;
}
</style>