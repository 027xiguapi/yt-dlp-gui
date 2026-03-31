<script setup lang="ts">
import { ref, computed, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { NButton, NInput, NSelect, NCard, NSpace, NProgress, NEmpty, NIcon, NTag } from "naive-ui";
import { Folder, Delete } from "@lucide/vue";
import { useConfigStore } from "../stores/configStore";
import { useDownloadStore } from "../stores/downloadStore";
import { useChannelStore } from "../stores/channelStore";

const configStore = useConfigStore();
const downloadStore = useDownloadStore();
const channelStore = useChannelStore();

const urlInput = ref("");
const presetOptions = computed(() =>
  configStore.config
    ? Object.keys(configStore.config.presets).map(name => ({ label: name, value: name }))
    : []
);

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

  await appWindow.listen<any>("extraction_progress", (event) => {
    const data = event.payload;
    channelStore.setExtractionProgress(data.progress || 0);
  });
});

async function addUrls() {
  if (!urlInput.value.trim() || !configStore.downloadPath) {
    alert("Please enter URLs and select a download path");
    return;
  }

  const urls = urlInput.value
    .split("\n")
    .map(u => u.trim())
    .filter(u => u && (u.includes("youtube.com") || u.includes("youtu.be")));

  if (urls.length === 0) {
    alert("No valid YouTube URLs found");
    return;
  }

  await downloadStore.addMultipleTasks(urls, configStore.selectedPreset, configStore.downloadPath);
  urlInput.value = "";
}

async function startDownloads() {
  const queuedTasks = downloadStore.taskList.filter(t => t.status === "Queued");
  if (queuedTasks.length === 0) {
    alert("No queued downloads");
    return;
  }

  for (const task of queuedTasks) {
    downloadStore.updateTask(task.id, { status: "Processing" });
    try {
      await invoke("start_download", { task, cookiePath: configStore.cookiePath || null });
    } catch (error) {
      console.error(`Failed to start download ${task.id}:`, error);
      downloadStore.updateTask(task.id, { status: "ERROR" });
    }
  }
}

function getStatusType(status: string): "success" | "warning" | "error" | "default" {
  switch (status) {
    case "Finished":
      return "success";
    case "Converting":
      return "warning";
    case "ERROR":
      return "error";
    default:
      return "default";
  }
}

async function addExtractedUrlsToQueue() {
  if (channelStore.extractedUrls.length === 0) {
    alert("No URLs to add");
    return;
  }

  urlInput.value = channelStore.extractedUrls.join("\n");
  await addUrls();
  channelStore.clearExtraction();
}
</script>

<template>
  <div class="main">
    <n-space vertical :size="16" style="padding: 20px">
      <!-- Header -->
      <div class="header">
        <h1>YouTube Batch Downloader</h1>
      </div>

      <!-- Settings Panel -->
      <n-card title="Settings" :segmented="{ content: true }">
        <n-space vertical :size="12">
          <div>
            <label class="label">Download Path:</label>
            <n-space :size="8">
              <n-input v-model:value="configStore.downloadPath" type="text" readonly style="flex: 1" />
              <n-button @click="() => configStore.setDownloadPath(prompt('Enter download path:', configStore.downloadPath) || configStore.downloadPath)" :icon-placement="'left'">
                <template #icon>
                  <n-icon><Folder /></n-icon>
                </template>
                Browse
              </n-button>
            </n-space>
          </div>

          <div>
            <label class="label">Cookie File (Optional):</label>
            <n-space :size="8">
              <n-input v-model:value="configStore.cookiePath" type="text" readonly style="flex: 1" placeholder="No cookie file selected" />
              <n-button @click="() => configStore.setCookiePath(prompt('Enter cookie file path:', configStore.cookiePath) || configStore.cookiePath)">Select</n-button>
              <n-button v-if="configStore.cookiePath" @click="configStore.clearCookiePath" type="error" :icon-placement="'left'">
                <template #icon>
                  <n-icon><Delete /></n-icon>
                </template>
              </n-button>
            </n-space>
          </div>

          <div>
            <label class="label">Preset:</label>
            <n-select v-model:value="configStore.selectedPreset" :options="presetOptions" />
          </div>
        </n-space>
      </n-card>

      <!-- URL Input Panel -->
      <n-card title="Add URLs" :segmented="{ content: true }">
        <n-space vertical :size="12">
          <n-input
            v-model:value="urlInput"
            type="textarea"
            placeholder="Paste YouTube URLs here (one per line)&#10;https://www.youtube.com/watch?v=..."
            :rows="6"
          />
          <n-button type="primary" @click="addUrls" block>Add to Queue</n-button>
        </n-space>
      </n-card>

      <!-- Channel Extraction Panel -->
      <n-card title="Extract from Channel" :segmented="{ content: true }">
        <n-space vertical :size="12">
          <div>
            <label class="label">Channel URL:</label>
            <n-input
              v-model:value="channelStore.channelUrl"
              type="text"
              placeholder="https://www.youtube.com/@ChannelName/videos"
              :disabled="channelStore.isExtracting"
            />
          </div>

          <n-button
            type="primary"
            @click="() => channelStore.extractChannelUrls(channelStore.channelUrl)"
            :loading="channelStore.isExtracting"
            block
          >
            {{ channelStore.isExtracting ? "Extracting..." : "Extract Videos" }}
          </n-button>

          <div v-if="channelStore.isExtracting">
            <n-progress :percentage="channelStore.extractionProgress" :show-indicator="true" />
          </div>

          <div v-if="channelStore.extractedUrls.length > 0">
            <n-space vertical :size="8">
              <div class="extraction-info">
                <span>Channel: <strong>{{ channelStore.extractedChannelName }}</strong></span>
                <span>Videos: <strong>{{ channelStore.extractedUrls.length }}</strong></span>
              </div>
              <n-button type="success" @click="addExtractedUrlsToQueue" block>
                Add {{ channelStore.extractedUrls.length }} Videos to Queue
              </n-button>
            </n-space>
          </div>
        </n-space>
      </n-card>

      <!-- Download Queue Panel -->
      <n-card title="Download Queue" :segmented="{ content: true }">
        <template #header-extra>
          <n-space :size="8">
            <n-button type="primary" @click="startDownloads" :disabled="downloadStore.isDownloading">
              Start Downloads
            </n-button>
            <n-button @click="downloadStore.clearCompleted">Clear Completed</n-button>
          </n-space>
        </template>

        <n-space vertical :size="12" style="max-height: 400px; overflow-y: auto">
          <n-empty v-if="downloadStore.taskList.length === 0" description="No downloads yet. Add URLs above to get started." />

          <div v-for="task in downloadStore.taskList" :key="task.id">
            <n-card :title="task.title || task.url" :segmented="{ content: true }" size="small">
              <template #header-extra>
                <n-tag :type="getStatusType(task.status)">{{ task.status }}</n-tag>
              </template>

              <n-space vertical :size="8">
                <div class="task-detail">
                  <span class="detail-label">URL:</span>
                  <span class="detail-value">{{ task.url }}</span>
                </div>
                <div class="task-detail">
                  <span class="detail-label">Preset:</span>
                  <span class="detail-value">{{ task.preset }}</span>
                </div>
                <div class="task-detail">
                  <span class="detail-label">Size:</span>
                  <span class="detail-value">{{ task.size }}</span>
                </div>

                <div v-if="task.status === 'Downloading' || task.status === 'Processing'">
                  <n-progress :percentage="task.progress" :show-indicator="true" />
                  <div class="progress-info">
                    <span>{{ task.progress.toFixed(1) }}%</span>
                    <span>Speed: {{ task.speed }}</span>
                    <span>ETA: {{ task.eta }}</span>
                  </div>
                </div>

                <div v-if="task.status === 'Queued' || task.status === 'Finished' || task.status === 'ERROR'">
                  <n-button
                    @click="downloadStore.removeTask(task.id)"
                    type="error"
                    size="small"
                    :icon-placement="'left'"
                  >
                    <template #icon>
                      <n-icon><Delete /></n-icon>
                    </template>
                    Remove
                  </n-button>
                </div>
              </n-space>
            </n-card>
          </div>
        </n-space>
      </n-card>
    </n-space>
  </div>
</template>

<style scoped>
.main {
  display: flex;
  flex-direction: column;
  height: 100%;
  overflow-y: auto;
}

.header {
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
  padding: 20px;
  border-radius: 0;
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

.task-detail {
  display: flex;
  gap: 10px;
  font-size: 13px;
  color: #666;
}

.detail-label {
  font-weight: 500;
  min-width: 60px;
}

.detail-value {
  flex: 1;
  word-break: break-all;
}

.progress-info {
  display: flex;
  justify-content: space-between;
  font-size: 12px;
  color: #666;
  margin-top: 8px;
}

.extraction-info {
  display: flex;
  gap: 20px;
  font-size: 14px;
  padding: 8px 0;
}
</style>
