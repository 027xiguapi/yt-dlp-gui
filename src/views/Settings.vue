<script setup lang="ts">
import { onMounted, ref, computed } from "vue";
import {
  NCard, NSpace, NButton, NInput,
  NForm, NFormItem, NIcon, NTooltip, NDivider, NSelect, useMessage
} from "naive-ui";
import {
  FolderOpen, FileText, Save, RotateCcw,
  Info, ShieldCheck, DownloadCloud, Terminal
} from "@lucide/vue";
import { open } from "@tauri-apps/plugin-dialog";
import { useConfigStore } from "../stores/configStore";
import { useI18n } from "../composables/useI18n";

const configStore = useConfigStore();
const message = useMessage();
const { t } = useI18n();

const checking = ref(false);

const browserOptions = computed(() => [
  { label: t("settings.browserOptions.chrome"), value: "chrome" },
  { label: t("settings.browserOptions.firefox"), value: "firefox" },
  { label: t("settings.browserOptions.edge"), value: "edge" },
  { label: t("settings.browserOptions.safari"), value: "safari" },
  { label: t("settings.browserOptions.opera"), value: "opera" },
  { label: t("settings.browserOptions.custom"), value: "custom" }
]);

onMounted(async () => {
  await configStore.loadConfig();
});

async function selectFolder() {
  try {
    const selected = await open({
      directory: true,
      multiple: false,
      title: t("settings.selectDownloadFolder")
    });
    if (selected && typeof selected === "string") {
      configStore.setDownloadPath(selected);
    }
  } catch (error) {
    console.error("Failed to select folder:", error);
  }
}

async function selectCookieFile() {
  try {
    const selected = await open({
      multiple: false,
      title: t("settings.selectCookieFile"),
      filters: [
        { name: "Text Files", extensions: ["txt"] },
        { name: "All Files", extensions: ["*"] }
      ]
    });
    if (selected && typeof selected === "string") {
      configStore.setCookiePath(selected);
    }
  } catch (error) {
    console.error("Failed to select file:", error);
  }
}

async function selectYtdlpFile() {
  try {
    const selected = await open({
      multiple: false,
      title: t("settings.selectYtdlpFile"),
      filters: [
        { name: "Executable Files", extensions: ["exe"] },
        { name: "All Files", extensions: ["*"] }
      ]
    });
    if (selected && typeof selected === "string") {
      configStore.setYtdlpPath(selected);
    }
  } catch (error) {
    console.error("Failed to select file:", error);
  }
}

async function saveSettings() {
  try {
    await configStore.saveConfig();
    message.success(t("settings.saveSuccess"));
  } catch (error) {
    message.error(t("settings.saveFailed", { error: String(error) }));
  }
}

function resetSettings() {
  if (configStore.config) {
    configStore.setDownloadPath(configStore.config.general.path);
    configStore.setCookiePath(configStore.config.general.cookie_path);
    configStore.setYtdlpPath(configStore.config.general.ytdlp_path || './win/yt-dlp.exe');
  }
}

async function checkVersions() {
  checking.value = true;
  await configStore.checkVersions();
  checking.value = false;
}
</script>

<template>
  <div class="flex flex-col min-h-screen bg-gray-50">
    <n-space vertical :size="20" class="p-6">
      <header class="bg-white px-10 py-8 border-b border-gray-200 -mx-6">
        <div class="flex items-center gap-4 max-w-4xl">
          <n-icon size="28" :component="DownloadCloud" />
          <div>
            <h1 class="m-0 text-2xl font-semibold text-gray-900">{{ t('settings.title') }}</h1>
            <span class="text-sm text-gray-600">{{ t('settings.subtitle') }}</span>
          </div>
        </div>
      </header>

      <main class="flex-1 w-full">
        <n-space vertical :size="20">
          <n-form label-placement="top">
            <n-card :title="t('settings.downloadConfig')" hoverable :segmented="{ content: true }">
              <template #header-extra>
                <n-icon size="20" color="#3b82f6" :component="ShieldCheck" />
              </template>

              <n-form-item :label="t('settings.downloadPath')">
                <n-input-group>
                  <n-input
                    v-model:value="configStore.downloadPath"
                    :placeholder="t('settings.downloadPathPlaceholder')"
                    readonly
                  />
                  <n-button type="primary" secondary @click="selectFolder">
                    <template #icon><n-icon :component="FolderOpen" /></template>
                    {{ t('settings.browse') }}
                  </n-button>
                </n-input-group>
              </n-form-item>

              <n-divider />

              <n-form-item :label="t('settings.ytdlpPath')">
                <n-input-group>
                  <n-input
                    v-model:value="configStore.ytdlpPath"
                    :placeholder="t('settings.ytdlpPathPlaceholder')"
                  />
                  <n-button @click="selectYtdlpFile">
                    <template #icon><n-icon :component="FileText" /></template>
                    {{ t('settings.selectFile') }}
                  </n-button>
                </n-input-group>
              </n-form-item>

              <n-divider />

              <n-form-item :label="t('settings.cookieSource')">
                <n-select
                  v-model:value="configStore.cookieBrowser"
                  :options="browserOptions"
                  @update:value="configStore.setCookieBrowser"
                />
              </n-form-item>

              <n-form-item v-if="configStore.cookieBrowser === 'custom'">
                <template #label>
                  <n-space :size="4" align="center">
                    <span>{{ t('settings.cookiePath') }}</span>
                    <n-tooltip trigger="hover">
                      <template #trigger>
                        <n-icon :component="Info" color="#999" />
                      </template>
                      {{ t('settings.cookieHint') }}
                    </n-tooltip>
                  </n-space>
                </template>

                <n-input-group>
                  <n-input
                    v-model:value="configStore.cookiePath"
                    :placeholder="t('settings.cookiePathPlaceholder')"
                  />
                  <n-button @click="selectCookieFile">
                    <template #icon><n-icon :component="FileText" /></template>
                    {{ t('settings.selectFile') }}
                  </n-button>
                  <n-button v-if="configStore.cookiePath" type="error" ghost @click="configStore.clearCookiePath">
                    {{ t('settings.clear') }}
                  </n-button>
                </n-input-group>
              </n-form-item>
              <p class="hint">
                {{ configStore.cookieBrowser === 'custom'
                  ? t('settings.customCookie')
                  : t('settings.fromBrowser', { browser: browserOptions.find(b => b.value === configStore.cookieBrowser)?.label || '' })
                }}
              </p>
            </n-card>

            <n-card :title="t('settings.envTest')" hoverable :segmented="{ content: true }" class="mt-5">
              <template #header-extra>
                <n-icon size="20" color="#10b981" :component="Terminal" />
              </template>

              <n-space vertical :size="12">
                <p class="hint">{{ t('settings.versionHint') }}</p>

                <n-button type="primary" :loading="checking" @click="checkVersions">
                  <template #icon><n-icon :component="Terminal" /></template>
                  {{ t('settings.checkVersion') }}
                </n-button>

                <n-divider v-if="configStore.versions.ytdlp || configStore.versions.deno || configStore.versions.ffmpeg || configStore.versions.ffprobe" />

                <n-space v-if="configStore.versions.ytdlp || configStore.versions.deno || configStore.versions.ffmpeg || configStore.versions.ffprobe" vertical :size="8">
                  <div v-if="configStore.versions.ytdlp">
                    <strong>yt-dlp:</strong> <span class="font-mono text-gray-600">{{ configStore.versions.ytdlp }}</span>
                  </div>
                  <div v-if="configStore.versions.deno">
                    <strong>deno:</strong> <span class="font-mono text-gray-600">{{ configStore.versions.deno }}</span>
                  </div>
                  <div v-if="configStore.versions.ffmpeg">
                    <strong>ffmpeg:</strong> <span class="font-mono text-gray-600">{{ configStore.versions.ffmpeg }}</span>
                  </div>
                  <div v-if="configStore.versions.ffprobe">
                    <strong>ffprobe:</strong> <span class="font-mono text-gray-600">{{ configStore.versions.ffprobe }}</span>
                  </div>
                </n-space>
              </n-space>
            </n-card>
          </n-form>

          <div class="mt-6 pt-5 border-t border-dashed border-gray-300">
            <n-space justify="end" :size="12">
              <n-button strong secondary @click="resetSettings">
                <template #icon><n-icon :component="RotateCcw" /></template>
                {{ t('settings.reset') }}
              </n-button>
              <n-button strong type="primary" size="large" @click="saveSettings">
                <template #icon><n-icon :component="Save" /></template>
                {{ t('settings.saveChanges') }}
              </n-button>
            </n-space>
          </div>
        </n-space>
      </main>
    </n-space>
  </div>
</template>

<style scoped>
/* Tailwind CSS handles all styling via utility classes above */
</style>