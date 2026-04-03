<script setup lang="ts">
import { onMounted } from "vue";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { NButton, NInput, NCard, NSpace, NProgress, NEmpty } from "naive-ui";
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
  <div class="h-screen w-full bg-gray-50 overflow-y-auto overflow-x-auto" style="min-width: 500px">
    <n-space vertical :size="16" class="p-5">
      <!-- Header -->
      <div class="bg-gradient-to-r from-purple-500 to-purple-600 text-white p-5 rounded-none shadow-sm">
        <h1 class="m-0 text-2xl font-bold">频道提取</h1>
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
            @click="() => channelStore.extractChannelUrls(channelStore.channelUrl, configStore.ytdlpPath || undefined)"
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
      <n-card v-if="channelStore.extractedUrls.length > 0" title="提取结果" :segmented="{ content: true }">
        <n-space vertical :size="12">
          <div class="flex gap-5 text-sm py-2">
            <span>频道: <strong>{{ channelStore.extractedChannelName }}</strong></span>
            <span>视频数: <strong>{{ channelStore.extractedUrls.length }}</strong></span>
          </div>

          <div class="border border-gray-300 rounded">
            <div class="bg-gray-100 px-3 py-2 font-medium text-sm">提取的URL:</div>
            <div class="max-h-72 overflow-y-auto p-2">
              <div v-for="(url, index) in channelStore.extractedUrls" :key="index" class="px-2 py-1.5 text-xs text-gray-600 word-break break-all border-b border-gray-100 last:border-b-0">
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
/* Tailwind CSS handles all styling via utility classes above */
</style>
