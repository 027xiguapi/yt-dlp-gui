<script setup lang="ts">
import { ref, onMounted, h } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { getCurrentWindow } from "@tauri-apps/api/window";
import {
  NButton, NInput, NCard, NSpace, NProgress,
  NIcon, NTag, NDataTable, NPopconfirm, useMessage
} from "naive-ui";
import { Trash2, Copy, Play, CheckCircle2, FolderOpen } from "@lucide/vue";
import { useConfigStore } from "../stores/configStore";
import { useDownloadStore } from "../stores/downloadStore";

const configStore = useConfigStore();
const downloadStore = useDownloadStore();
const message = useMessage();

const urlInput = ref("");
const checkedRowKeys = ref<string[]>([]);

onMounted(async () => {
  await configStore.loadConfig();

  const appWindow = await getCurrentWindow();
  await appWindow.listen<any>("download_progress", (event) => {
    const data = event.payload;
    downloadStore.updateTask(data.id, {
      status: data.status,
      progress: data.progress ? parseFloat(data.progress) : undefined,
      speed: data.speed,
      eta: data.eta,
      size: data.size,
      title: data.title,
    });
  });
});

// --- 逻辑处理 ---

async function addUrls() {
  if (!urlInput.value.trim() || !configStore.downloadPath) {
    message.warning("请确认输入了 URL 并设置了下载路径");
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
    message.error("未发现有效的链接");
    return;
  }

  await downloadStore.addMultipleTasks(urls, configStore.selectedPreset, configStore.downloadPath);
  urlInput.value = "";
  message.success(`成功添加 ${urls.length} 个任务，开始下载...`);

  // 自动批量下载新添加的任务
  const newTasks = downloadStore.taskList.filter(t => t.status === "Queued");
  for (const task of newTasks) {
    await runDownloadTask(task);
  }
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
      cookiesFromBrowser: "chrome"
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
    await invoke("plugin:opener|open", { path: configStore.downloadPath });
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
    width: 160,
    ellipsis: { tooltip: true },
    render(row: any) {
      const displayText = row.title || row.url;
      return h('span', { title: displayText }, displayText);
    }
  },
  {
    title: '状态',
    key: 'status',
    width: 120,
    render(row: any) {
      return h(NTag, { type: getStatusType(row.status), bordered: false }, { default: () => row.status });
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
          }, { default: () => h(NIcon, null, { default: () => h(Play) }) }),

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
  <div class="main-container">
    <n-space vertical :size="20" style="padding: 24px">
      <div class="app-header">
        <h1 class="title">YouTube 下载管理器</h1>
        <p class="subtitle">高性能下载引擎</p>
      </div>

      <n-card hoverable>
        <n-space vertical :size="12">
          <n-input
            v-model:value="urlInput"
            type="textarea"
            placeholder="在此粘贴 YouTube 链接（支持多行粘贴）"
            :autosize="{ minRows: 3, maxRows: 6 }"
          />
          <n-button type="primary" size="large" @click="addUrls" block>
            添加到下载队列
          </n-button>
        </n-space>
      </n-card>

      <n-card title="下载任务队列" :segmented="{ content: true }">
        <template #header-extra>
          <n-space>
            <n-button 
              secondary 
              type="primary" 
              @click="startSelectedDownloads" 
              :disabled="checkedRowKeys.length === 0"
            >
              开始选中 ({{ checkedRowKeys.length }})
            </n-button>
            <n-button 
              secondary 
              type="error" 
              @click="deleteSelectedRows" 
              :disabled="checkedRowKeys.length === 0"
            >
              删除选中
            </n-button>
            <n-button quaternary @click="downloadStore.clearCompleted">
              清空已完成
            </n-button>
          </n-space>
        </template>

        <n-data-table
          remote
          ref="table"
          :columns="columns"
          :data="downloadStore.taskList"
          :row-key="(row) => row.id"
          v-model:checked-row-keys="checkedRowKeys"
          :pagination="false"
          :max-height="500"
        />
      </n-card>
    </n-space>
  </div>
</template>

<style scoped>
.main-container {
  height: 100vh;
  background-color: #f9f9f9;
  overflow-y: auto;
}

.app-header {
  margin-bottom: 8px;
}

.title {
  margin: 0;
  font-size: 28px;
  font-weight: 700;
  background: linear-gradient(135deg, #3b82f6, #8b5cf6);
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
}

.subtitle {
  margin: 4px 0 0;
  color: #666;
  font-size: 14px;
}

:deep(.n-data-table-table) {
  font-variant-numeric: tabular-nums;
}
</style>