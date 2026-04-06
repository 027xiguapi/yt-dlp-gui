<script setup lang="ts">
import { ref, onMounted, computed } from "vue";
import { useI18n } from "vue-i18n";
import { invoke } from "@tauri-apps/api/core";
import { getCurrentWindow } from "@tauri-apps/api/window";
import {
  NButton, NInput, NCard, NSpace, NProgress,
  NIcon, NTag, NPopconfirm, NTooltip, NCheckbox, NImage, NEmpty, useMessage
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
const checkedIds = ref<Set<string>>(new Set());
const fetchingInfo = ref(false);
const searchKeyword = ref("");
const currentPage = ref(1);
const pageSize = ref(20);

// 搜索过滤
const filteredTaskList = computed(() => {
  const keyword = searchKeyword.value.trim().toLowerCase();
  if (!keyword) return downloadStore.taskList;
  return downloadStore.taskList.filter(
    task => task.title.toLowerCase().includes(keyword) || task.url.toLowerCase().includes(keyword)
  );
});

// 分页
const pagedTaskList = computed(() => {
  const start = (currentPage.value - 1) * pageSize.value;
  return filteredTaskList.value.slice(start, start + pageSize.value);
});

const totalPages = computed(() => Math.ceil(filteredTaskList.value.length / pageSize.value) || 1);
const isAllChecked = computed(() =>
  pagedTaskList.value.length > 0 && pagedTaskList.value.every(t => checkedIds.value.has(t.id))
);

function toggleCheckAll() {
  if (isAllChecked.value) {
    pagedTaskList.value.forEach(t => checkedIds.value.delete(t.id));
  } else {
    pagedTaskList.value.forEach(t => checkedIds.value.add(t.id));
  }
  checkedIds.value = new Set(checkedIds.value);
}

function toggleCheck(id: string) {
  if (checkedIds.value.has(id)) {
    checkedIds.value.delete(id);
  } else {
    checkedIds.value.add(id);
  }
  checkedIds.value = new Set(checkedIds.value);
}

// 格式化速度
function formatSpeed(speed: string): string {
  if (!speed) return '';
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
      try { new URL(u); return true; }
      catch { return false; }
    });

  if (urls.length === 0) {
    message.error(t("main.noValidUrl"));
    return;
  }

  fetchingInfo.value = true;
  const infoMap = new Map<string, VideoInfoResult>();
  for (const url of urls) {
    const info = await fetchVideoInfo(url);
    if (info) infoMap.set(url, info);
  }

  for (const url of urls) {
    const info = infoMap.get(url);
    await downloadStore.addTask(url, configStore.selectedPreset, configStore.downloadPath,
      info ? { title: info.title, thumbnail: info.thumbnail } : undefined,
    );
  }

  fetchingInfo.value = false;
  urlInput.value = "";
  message.success(t("main.addSuccess", { count: urls.length }));
}

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

async function startSelectedDownloads() {
  const selectedTasks = downloadStore.taskList.filter(
    t => checkedIds.value.has(t.id) && (t.status === "Queued" || t.status === "ERROR")
  );
  for (const task of selectedTasks) {
    runDownloadTask(task);
  }
}

function deleteSelectedRows() {
  checkedIds.value.forEach(id => downloadStore.removeTask(id));
  checkedIds.value = new Set();
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

function getStatusLabel(status: string) {
  switch (status) {
    case "Finished": return "已完成";
    case "Processing": return "处理中";
    case "Downloading": return "下载中";
    case "Converting": return "转换中";
    case "Queued": return "排队中";
    case "ERROR": return "错误";
    default: return status;
  }
}
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
              :disabled="checkedIds.size === 0"
            >
              {{ t('main.startSelected') }} ({{ checkedIds.size }})
            </n-button>
            <n-button
              secondary
              type="error"
              @click="deleteSelectedRows"
              :disabled="checkedIds.size === 0"
            >
              {{ t('main.deleteSelected') }}
            </n-button>
            <n-button quaternary @click="downloadStore.clearCompleted">
              {{ t('main.clearCompleted') }}
            </n-button>
          </n-space>
        </template>

        <div v-if="pagedTaskList.length === 0">
          <n-empty :description="t('main.noValidUrl')" class="py-12" />
        </div>

        <div v-else class="space-y-3">
          <!-- Select All -->
          <div class="flex items-center gap-3 pb-2 border-b border-gray-200">
            <n-checkbox :checked="isAllChecked" @update:checked="toggleCheckAll" />
            <span class="text-sm text-gray-500">
              {{ checkedIds.size }} / {{ pagedTaskList.length }}
            </span>
          </div>

          <!-- Task Cards -->
          <div
            v-for="task in pagedTaskList"
            :key="task.id"
            class="flex items-stretch gap-4 p-3 rounded-xl border transition-colors"
            :class="{
              'bg-blue-50/50 border-blue-200': checkedIds.has(task.id),
              'bg-white border-gray-200 hover:border-gray-300 hover:shadow-sm': !checkedIds.has(task.id),
              'border-l-4': true,
              'border-l-green-400': task.status === 'Finished',
              'border-l-blue-400': task.status === 'Downloading' || task.status === 'Processing',
              'border-l-yellow-400': task.status === 'Converting',
              'border-l-gray-300': task.status === 'Queued',
              'border-l-red-400': task.status === 'ERROR',
            }"
          >
            <!-- Checkbox -->
            <div class="flex items-center pt-1">
              <n-checkbox
                :checked="checkedIds.has(task.id)"
                @update:checked="toggleCheck(task.id)"
              />
            </div>

            <!-- Thumbnail -->
            <div class="flex-shrink-0 w-[160px] h-[90px] rounded-lg overflow-hidden bg-gray-100">
              <n-image
                v-if="task.thumbnail"
                :src="task.thumbnail"
                :width="160"
                :height="90"
                object-fit="cover"
                preview-disabled
                :fallback-src="''"
                class="w-full h-full"
              />
              <div v-else class="w-full h-full flex items-center justify-center">
                <n-icon :size="32" class="text-gray-300"><Play /></n-icon>
              </div>
            </div>

            <!-- Info Area -->
            <div class="flex-1 min-w-0 flex flex-col justify-between py-0.5">
              <!-- Title + Status -->
              <div class="flex items-start gap-2">
                <h3 class="text-sm font-medium text-gray-800 truncate flex-1" :title="task.title || task.url">
                  {{ task.title || task.url }}
                </h3>
                <n-tag
                  :type="getStatusType(task.status)"
                  size="tiny"
                  :bordered="false"
                  class="flex-shrink-0"
                >
                  {{ getStatusLabel(task.status) }}
                </n-tag>
              </div>

              <!-- Progress / Status Info -->
              <div class="mt-auto">
                <!-- Downloading/Processing: progress bar + speed -->
                <template v-if="['Downloading', 'Processing', 'Converting'].includes(task.status)">
                  <n-progress
                    type="line"
                    :percentage="task.progress || 0"
                    :show-indicator="true"
                    indicator-placement="inside"
                    :height="16"
                    :border-radius="4"
                    :color="task.status === 'Converting' ? '#f0a020' : '#3b82f6'"
                  />
                  <div class="flex items-center gap-4 mt-1.5 text-xs text-gray-500">
                    <span v-if="task.speed" class="flex items-center gap-1">
                      <svg class="w-3 h-3" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M12 2v4m0 12v4m-7.07-3.93l2.83-2.83m8.48-8.48l2.83-2.83M2 12h4m12 0h4M4.93 4.93l2.83 2.83m8.48 8.48l2.83 2.83"/></svg>
                      {{ task.speed }}
                    </span>
                    <span v-if="task.eta">ETA {{ task.eta }}</span>
                    <span v-if="task.size">{{ task.size }}</span>
                  </div>
                </template>

                <!-- Finished -->
                <template v-else-if="task.status === 'Finished'">
                  <div class="flex items-center gap-1.5 text-xs text-green-600">
                    <n-icon :size="14"><CheckCircle2 /></n-icon>
                    <span v-if="task.size">{{ task.size }}</span>
                    <span v-else>下载完成</span>
                  </div>
                </template>

                <!-- ERROR -->
                <template v-else-if="task.status === 'ERROR'">
                  <n-tooltip trigger="hover">
                    <template #trigger>
                      <span class="text-xs text-red-500 truncate block cursor-pointer max-w-[300px]">
                        {{ task.error || '下载失败' }}
                      </span>
                    </template>
                    {{ task.error || '下载失败' }}
                  </n-tooltip>
                </template>

                <!-- Queued -->
                <template v-else-if="task.status === 'Queued'">
                  <span class="text-xs text-gray-400">等待下载...</span>
                </template>
              </div>
            </div>

            <!-- Action Buttons -->
            <div class="flex flex-col items-center justify-center gap-1 flex-shrink-0 pl-2">
              <n-tooltip trigger="hover">
                <template #trigger>
                  <n-button
                    quaternary
                    circle
                    size="small"
                    type="primary"
                    :disabled="!['Queued', 'ERROR'].includes(task.status)"
                    @click="runDownloadTask(task)"
                  >
                    <template #icon><n-icon :size="16"><Download /></n-icon></template>
                  </n-button>
                </template>
                {{ t('main.startDownload') }}
              </n-tooltip>

              <n-tooltip trigger="hover">
                <template #trigger>
                  <n-button quaternary circle size="small" @click="copyUrl(task.url)">
                    <template #icon><n-icon :size="16"><Copy /></n-icon></template>
                  </n-button>
                </template>
                {{ t('main.copyUrl') }}
              </n-tooltip>

              <n-tooltip trigger="hover">
                <template #trigger>
                  <n-button quaternary circle size="small" @click="openDownloadFolder">
                    <template #icon><n-icon :size="16"><FolderOpen /></n-icon></template>
                  </n-button>
                </template>
                {{ t('main.openFolder') }}
              </n-tooltip>

              <n-popconfirm @positive-click="downloadStore.removeTask(task.id); checkedIds.delete(task.id); checkedIds = new Set(checkedIds)">
                <template #trigger>
                  <n-button quaternary circle size="small" type="error">
                    <template #icon><n-icon :size="16"><Trash2 /></n-icon></template>
                  </n-button>
                </template>
                {{ t('main.deleteConfirm') }}
              </n-popconfirm>
            </div>
          </div>

          <!-- Pagination -->
          <div class="flex items-center justify-between pt-3 border-t border-gray-100">
            <span class="text-sm text-gray-500">
              {{ t('main.downloadQueue') }}：{{ filteredTaskList.length }} 条
            </span>
            <n-space :size="8" align="center">
              <n-button
                size="tiny"
                :disabled="currentPage <= 1"
                @click="currentPage--"
              >
                上一页
              </n-button>
              <span class="text-sm text-gray-600">
                {{ currentPage }} / {{ totalPages }}
              </span>
              <n-button
                size="tiny"
                :disabled="currentPage >= totalPages"
                @click="currentPage++"
              >
                下一页
              </n-button>
            </n-space>
          </div>
        </div>
      </n-card>
    </n-space>
  </div>
</template>

<style scoped>
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
