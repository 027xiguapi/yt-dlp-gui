<script setup lang="ts">
import { onMounted } from "vue";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { NButton, NInput, NCard, NSpace, NProgress, NEmpty, NIcon } from "naive-ui";
import { useChannelStore } from "../stores/channelStore";
import { useDownloadStore } from "../stores/downloadStore";
import { useConfigStore } from "../stores/configStore";

const channelStore = useChannelStore();
const downloadStore = useDownloadStore();
const configStore = useConfigStore();

onMounted(async () => {
  await configStore.loadConfig();

  const appWindow = await getCurrentWindow();
  await appWindow.listen<any>("extraction_progress", (event) => {
    const data = event.payload;
    channelStore.setExtractionProgress(data.progress || 0);
  });
});

async function addExtractedUrlsToQueue() {
  if (channelStore.extractedUrls.length === 0) {
    alert("No URLs to add");
    return;
  }

  if (!configStore.downloadPath) {
    alert("Please select a download path in settings");
    return;
  }

  await downloadStore.addMultipleTasks(
    channelStore.extractedUrls,
    configStore.selectedPreset,
    configStore.downloadPath
  );
  channelStore.clearExtraction();
  alert(`Added ${channelStore.extractedUrls.length} videos to download queue`);
}
</script>

<template>
  <div class="channel-extraction">
    <n-space vertical :size="16" style="padding: 20px">
      <!-- Header -->
      <div class="header">
        <h1>Extract from Channel</h1>
      </div>

      <!-- Channel Extraction Panel -->
      <n-card title="Channel URL" :segmented="{ content: true }">
        <n-space vertical :size="12">
          <n-input
            v-model:value="channelStore.channelUrl"
            type="text"
            placeholder="https://www.youtube.com/@ChannelName/videos"
            :disabled="channelStore.isExtracting"
          />
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
        </n-space>
      </n-card>

      <!-- Extraction Results -->
      <n-card v-if="channelStore.extractedUrls.length > 0" title="Extraction Results" :segmented="{ content: true }">
        <n-space vertical :size="12">
          <div class="extraction-info">
            <span>Channel: <strong>{{ channelStore.extractedChannelName }}</strong></span>
            <span>Videos: <strong>{{ channelStore.extractedUrls.length }}</strong></span>
          </div>

          <div class="urls-list">
            <div class="urls-header">Extracted URLs:</div>
            <div class="urls-content">
              <div v-for="(url, index) in channelStore.extractedUrls" :key="index" class="url-item">
                {{ index + 1 }}. {{ url }}
              </div>
            </div>
          </div>

          <n-button type="success" @click="addExtractedUrlsToQueue" block>
            Add {{ channelStore.extractedUrls.length }} Videos to Download Queue
          </n-button>

          <n-button @click="channelStore.clearExtraction" block>Clear Results</n-button>
        </n-space>
      </n-card>

      <n-empty v-else description="No extraction results yet. Enter a channel URL and click Extract Videos." />
    </n-space>
  </div>
</template>

<style scoped>
.channel-extraction {
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

.extraction-info {
  display: flex;
  gap: 20px;
  font-size: 14px;
  padding: 8px 0;
}

.urls-list {
  border: 1px solid #e0e0e0;
  border-radius: 4px;
  overflow: hidden;
}

.urls-header {
  background-color: #f5f5f5;
  padding: 8px 12px;
  font-weight: 500;
  font-size: 13px;
}

.urls-content {
  max-height: 300px;
  overflow-y: auto;
  padding: 8px;
}

.url-item {
  padding: 6px 8px;
  font-size: 12px;
  color: #666;
  word-break: break-all;
  border-bottom: 1px solid #f0f0f0;
}

.url-item:last-child {
  border-bottom: none;
}
</style>
