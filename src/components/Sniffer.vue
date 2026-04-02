<script setup lang="ts">
import { ref, h } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { getCurrentWindow } from "@tauri-apps/api/window";
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

const appWindow = getCurrentWindow();

appWindow.listen<any>("sniff_progress", (_event) => {
  sniffProgress.value = Math.min(sniffProgress.value + 10, 90);
});

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
  <div class="sniffer-container">
    <n-space vertical :size="16" style="padding: 20px">
      <!-- Header -->
      <div class="header">
        <h1>YouTube Resource Sniffer</h1>
      </div>

      <!-- Input Panel -->
      <n-card title="Sniff Resources" :segmented="{ content: true }">
        <n-space vertical :size="12">
          <div>
            <label class="label">YouTube Video URL:</label>
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
            {{ isSniffing ? "Sniffing..." : "Start Sniffing" }}
          </n-button>

          <div v-if="isSniffing">
            <n-progress :percentage="sniffProgress" :show-indicator="true" />
          </div>
        </n-space>
      </n-card>

      <!-- Results Panel -->
      <n-card v-if="videos.length > 0 || images.length > 0" title="Captured Resources" :segmented="{ content: true }">
        <n-tabs type="bar">
          <n-tab-pane name="Videos" :tab="`Videos (${videos.length})`">
            <n-space vertical :size="12">
              <div v-if="videos.length === 0" class="empty-state">
                No videos captured
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

          <n-tab-pane name="Images" :tab="`Images (${images.length})`">
            <n-space vertical :size="12">
              <div v-if="images.length === 0" class="empty-state">
                No images captured
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

          <n-tab-pane name="Statistics" tab="Statistics">
            <n-space vertical :size="12">
              <div class="stats-grid">
                <div class="stat-item">
                  <div class="stat-label">Total Videos</div>
                  <div class="stat-value">{{ videos.length }}</div>
                </div>
                <div class="stat-item">
                  <div class="stat-label">Total Images</div>
                  <div class="stat-value">{{ images.length }}</div>
                </div>
                <div class="stat-item">
                  <div class="stat-label">Total Resources</div>
                  <div class="stat-value">{{ videos.length + images.length }}</div>
                </div>
                <div class="stat-item">
                  <div class="stat-label">Videos Size</div>
                  <div class="stat-value">{{ formatSize(videos.reduce((sum, v) => sum + v.size, 0)) }}</div>
                </div>
              </div>
            </n-space>
          </n-tab-pane>
        </n-tabs>
      </n-card>

      <n-empty v-else-if="!isSniffing" description="Enter a YouTube URL and click 'Start Sniffing' to capture resources" />
    </n-space>
  </div>
</template>

<style scoped>
.sniffer-container {
  display: flex;
  flex-direction: column;
  height: 100%;
  overflow-y: auto;
}

.header {
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
  padding: 20px;
  border-radius: 8px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
}

.header h1 {
  margin: 0;
  font-size: 24px;
}

.label {
  display: block;
  margin-bottom: 8px;
  font-weight: 500;
  color: #555;
}

.empty-state {
  text-align: center;
  padding: 40px 20px;
  color: #999;
}

.stats-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
  gap: 16px;
}

.stat-item {
  padding: 16px;
  background: #f5f5f5;
  border-radius: 8px;
  text-align: center;
}

.stat-label {
  font-size: 12px;
  color: #999;
  margin-bottom: 8px;
}

.stat-value {
  font-size: 24px;
  font-weight: bold;
  color: #333;
}
</style>
