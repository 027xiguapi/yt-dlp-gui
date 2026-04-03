<script setup lang="ts">
import { onMounted } from "vue";
import { useRouter } from "vue-router";
import { useI18n } from "vue-i18n";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { NButton, NInput, NCard, NSpace, NProgress, NEmpty, useMessage } from "naive-ui";
import { useChannelStore } from "../stores/channelStore";
import { useDownloadStore } from "../stores/downloadStore";
import { useConfigStore } from "../stores/configStore";

const router = useRouter();
const { t } = useI18n();
const channelStore = useChannelStore();
const downloadStore = useDownloadStore();
const configStore = useConfigStore();
const message = useMessage();

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
    message.warning(t("channelExtraction.noUrlsWarning"));
    return;
  }

  if (!configStore.downloadPath) {
    message.warning(t("channelExtraction.noDownloadPathWarning"));
    return;
  }

  const count = channelStore.extractedUrls.length;

  await downloadStore.addMultipleTasks(
    channelStore.extractedUrls,
    configStore.selectedPreset,
    configStore.downloadPath
  );
  channelStore.clearExtraction();
  message.success(t("channelExtraction.addSuccess", { count }));
  setTimeout(() => {
    router.push("/");
  }, 1000);
}
</script>

<template>
  <div class="h-screen w-full bg-gray-50 overflow-y-auto overflow-x-auto" style="min-width: 500px">
    <n-space vertical :size="16" class="p-5">
      <!-- Header -->
      <div class="bg-gradient-to-r from-purple-500 to-purple-600 text-white p-5 rounded-none shadow-sm">
        <h1 class="m-0 text-2xl font-bold">{{ t('channelExtraction.title') }}</h1>
      </div>

      <!-- Channel Extraction Panel -->
      <n-card :title="t('channelExtraction.channelUrl')" :segmented="{ content: true }">
        <n-space vertical :size="12">
          <n-input
            v-model:value="channelStore.channelUrl"
            type="text"
            :placeholder="t('channelExtraction.placeholder')"
            :disabled="channelStore.isExtracting"
          />
          <n-button
            type="primary"
            @click="() => channelStore.extractChannelUrls(channelStore.channelUrl, configStore.ytdlpPath || undefined)"
            :loading="channelStore.isExtracting"
            block
          >
            {{ channelStore.isExtracting ? t('channelExtraction.extracting') : t('channelExtraction.extract') }}
          </n-button>

          <div v-if="channelStore.isExtracting">
            <n-progress :percentage="channelStore.extractionProgress" :show-indicator="true" />
          </div>
        </n-space>
      </n-card>

      <!-- Extraction Results -->
      <n-card v-if="channelStore.extractedUrls.length > 0" :title="t('channelExtraction.results')" :segmented="{ content: true }">
        <n-space vertical :size="12">
          <div class="flex gap-5 text-sm py-2">
            <span>{{ t('channelExtraction.channel') }}: <strong>{{ channelStore.extractedChannelName }}</strong></span>
            <span>{{ t('channelExtraction.videosCount') }}: <strong>{{ channelStore.extractedUrls.length }}</strong></span>
          </div>

          <div class="border border-gray-300 rounded">
            <div class="bg-gray-100 px-3 py-2 font-medium text-sm">{{ t('channelExtraction.extractedUrls') }}</div>
            <div class="max-h-72 overflow-y-auto p-2">
              <div v-for="(url, index) in channelStore.extractedUrls" :key="index" class="px-2 py-1.5 text-xs text-gray-600 word-break break-all border-b border-gray-100 last:border-b-0">
                {{ index + 1 }}. {{ url }}
              </div>
            </div>
          </div>

          <n-button type="success" @click="addExtractedUrlsToQueue" block>
            {{ t('channelExtraction.addToQueue', { count: channelStore.extractedUrls.length }) }}
          </n-button>

          <n-button @click="channelStore.clearExtraction" block>{{ t('channelExtraction.clearResults') }}</n-button>
        </n-space>
      </n-card>

      <n-empty v-else :description="t('channelExtraction.noResults')" />
    </n-space>
  </div>
</template>

<style scoped>
/* Tailwind CSS handles all styling via utility classes above */
</style>
