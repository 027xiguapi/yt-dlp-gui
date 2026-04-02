<script setup lang="ts">
import { ref, h } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { NButton, NInput, NCard, NSpace, NProgress, NEmpty, NTable, NTabs, NTabPane } from "naive-ui";

interface CapturedResource {
  url: string;
  resource_type: string;
  mime_type: string;
  size: number;
}

const videoUrl = ref("");
const isSniffing = ref(false);
const sniffProgress = ref(0);
const videos = ref<CapturedResource[]>([]);
const images = ref<CapturedResource[]>([]);

// const appWindow = getCurrentWindow();

// appWindow.then(win => {
//   win.listen<any>("sniff_progress", (event) => {
//     sniffProgress.value = Math.min(sniffProgress.value + 10, 90);
//   });
// });

async function startSniffing() {
  if (!videoUrl.value.trim()) {
    alert("Please enter a YouTube video URL");
    return;
  }

  isSniffing.value = true;
  sniffProgress.value = 0;
  videos.value = [];
  images.value = [];

  try {
    const [videoResources, imageResources] = await invoke<[CapturedResource[], CapturedResource[]]>(
      "sniff_youtube_resources",
      { videoUrl: videoUrl.value }
    );
    videos.value = videoResources;
    images.value = imageResources;
    sniffProgress.value = 100;
  } catch (error) {
    console.error("Failed to sniff resources:", error);
    alert(`Error: ${error}`);
  } finally {
    isSniffing.value = false;
  }
}

function copyToClipboard(text: string) {
  navigator.clipboard.writeText(text);
  alert("Copied to clipboard!");
}

function formatSize(bytes: number): string {
  if (bytes === 0) return "0 B";
  const k = 1024;
  const sizes = ["B", "KB", "MB", "GB"];
  const i = Math.floor(Math.log(bytes) / Math.log(k));
  return Math.round((bytes / Math.pow(k, i)) * 100) / 100 + " " + sizes[i];
}

const videoColumns = [
  {
    title: "URL",
    key: "url",
    width: 300,
    ellipsis: true,
  },
  {
    title: "MIME Type",
    key: "mime_type",
    width: 150,
  },
  {
    title: "Size",
    key: "size",
    width: 100,
    render: (row: CapturedResource) => formatSize(row.size),
  },
  {
    title: "Action",
    key: "action",
    width: 150,
    align: "center" as const,
    render: (row: CapturedResource) =>
      h(
        NSpace,
        { size: 8 },
        {
          default: () => [
            h(
              NButton,
              {
                size: "small",
                type: "primary",
                onClick: () => copyToClipboard(row.url),
              },
              { default: () => "Copy" }
            ),
          ],
        }
      ),
  },
];

const imageColumns = [
  {
    title: "URL",
    key: "url",
    width: 300,
    ellipsis: true,
  },
  {
    title: "MIME Type",
    key: "mime_type",
    width: 150,
  },
  {
    title: "Size",
    key: "size",
    width: 100,
    render: (row: CapturedResource) => formatSize(row.size),
  },
  {
    title: "Preview",
    key: "preview",
    width: 100,
    align: "center" as const,
    render: (row: CapturedResource) =>
      h(
        NButton,
        {
          size: "small",
          text: true,
          onClick: () => window.open(row.url, "_blank"),
        },
        { default: () => "View" }
      ),
  },
  {
    title: "Action",
    key: "action",
    width: 150,
    align: "center" as const,
    render: (row: CapturedResource) =>
      h(
        NSpace,
        { size: 8 },
        {
          default: () => [
            h(
              NButton,
              {
                size: "small",
                type: "primary",
                onClick: () => copyToClipboard(row.url),
              },
              { default: () => "Copy" }
            ),
          ],
        }
      ),
  },
];
</script>

<template>
  <div class="flex flex-col h-full overflow-y-auto">
    <n-space vertical :size="16" class="p-5">
      <!-- Header -->
      <div class="bg-gradient-to-r from-purple-500 to-purple-600 text-white p-5 rounded-none shadow-sm">
        <h1 class="m-0 text-2xl font-bold">YouTube 资源嗅探</h1>
      </div>

      <!-- Input Panel -->
      <n-card title="嗅探资源" :segmented="{ content: true }">
        <n-space vertical :size="12">
          <div>
            <label class="block mb-2 font-medium text-gray-700">YouTube 视频 URL:</label>
            <n-input
              v-model:value="videoUrl"
              type="text"
              placeholder="https://www.youtube.com/watch?v=..."
              :disabled="isSniffing"
            />
          </div>

          <n-button
            type="primary"
            @click="startSniffing"
            :loading="isSniffing"
            block
          >
            {{ isSniffing ? "嗅探中..." : "开始嗅探" }}
          </n-button>

          <div v-if="isSniffing">
            <n-progress :percentage="sniffProgress" :show-indicator="true" />
          </div>
        </n-space>
      </n-card>

      <!-- Results Panel -->
      <n-card v-if="videos.length > 0 || images.length > 0" title="捕获的资源" :segmented="{ content: true }">
        <n-tabs type="bar">
          <n-tab-pane name="Videos" :tab="`视频 (${videos.length})`">
            <n-space vertical :size="12">
              <div v-if="videos.length === 0" class="text-center py-10 text-gray-500">
                未捕获视频
              </div>
              <n-table
                v-else
                :columns="videoColumns"
                :data="videos"
                :bordered="false"
                :single-line="false"
                size="small"
              />
            </n-space>
          </n-tab-pane>

          <n-tab-pane name="Images" :tab="`图片 (${images.length})`">
            <n-space vertical :size="12">
              <div v-if="images.length === 0" class="text-center py-10 text-gray-500">
                未捕获图片
              </div>
              <n-table
                v-else
                :columns="imageColumns"
                :data="images"
                :bordered="false"
                :single-line="false"
                size="small"
              />
            </n-space>
          </n-tab-pane>

          <n-tab-pane name="Statistics" tab="统计信息">
            <n-space vertical :size="12">
              <div class="grid grid-cols-auto-fit gap-4">
                <div class="p-4 bg-gray-100 rounded text-center">
                  <div class="text-xs text-gray-600 mb-2">总视频数</div>
                  <div class="text-2xl font-bold text-gray-900">{{ videos.length }}</div>
                </div>
                <div class="p-4 bg-gray-100 rounded text-center">
                  <div class="text-xs text-gray-600 mb-2">总图片数</div>
                  <div class="text-2xl font-bold text-gray-900">{{ images.length }}</div>
                </div>
                <div class="p-4 bg-gray-100 rounded text-center">
                  <div class="text-xs text-gray-600 mb-2">总资源数</div>
                  <div class="text-2xl font-bold text-gray-900">{{ videos.length + images.length }}</div>
                </div>
                <div class="p-4 bg-gray-100 rounded text-center">
                  <div class="text-xs text-gray-600 mb-2">视频总大小</div>
                  <div class="text-2xl font-bold text-gray-900">{{ formatSize(videos.reduce((sum, v) => sum + v.size, 0)) }}</div>
                </div>
              </div>
            </n-space>
          </n-tab-pane>
        </n-tabs>
      </n-card>

      <n-empty v-else-if="!isSniffing" description="输入 YouTube URL 并点击'开始嗅探'来捕获资源" />
    </n-space>
  </div>
</template>

<style scoped>
/* Tailwind CSS handles all styling via utility classes above */
</style>
