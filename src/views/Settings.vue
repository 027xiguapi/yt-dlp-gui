<script setup lang="ts">
import { ref, onMounted } from "vue";
import { NCard, NSpace, NButton, NInput, NSelect, NAlert } from "naive-ui";
import { useConfigStore } from "../stores/configStore";

const configStore = useConfigStore();
const saveMessage = ref("");

onMounted(async () => {
  await configStore.loadConfig();
});

async function selectFolder() {
  const newPath = prompt("Enter download path:", configStore.downloadPath);
  if (newPath) {
    configStore.setDownloadPath(newPath);
  }
}

async function selectCookieFile() {
  const newPath = prompt("Enter cookie file path:", configStore.cookiePath);
  if (newPath) {
    configStore.setCookiePath(newPath);
  }
}

async function saveSettings() {
  try {
    await configStore.saveConfig();
    saveMessage.value = "Settings saved successfully!";
    setTimeout(() => {
      saveMessage.value = "";
    }, 3000);
  } catch (error) {
    saveMessage.value = `Error: ${error}`;
  }
}

function resetSettings() {
  if (configStore.config) {
    configStore.setDownloadPath(configStore.config.general.path);
    configStore.setCookiePath(configStore.config.general.cookie_path);
    configStore.setGlobalArgs(configStore.config.general.global_args);
    configStore.setUpdateYtdlp(configStore.config.general.update_ytdlp);
  }
}
</script>

<template>
  <div class="settings">
    <n-space vertical :size="16" style="padding: 20px">
      <!-- Header -->
      <div class="header">
        <h1>Settings</h1>
      </div>

      <!-- Message -->
      <n-alert v-if="saveMessage" type="success" closable>
        {{ saveMessage }}
      </n-alert>

      <!-- Download Settings -->
      <n-card title="Download Settings" :segmented="{ content: true }">
        <n-space vertical :size="12">
          <div>
            <label class="label">Download Path:</label>
            <n-space :size="8">
              <n-input v-model:value="configStore.downloadPath" type="text" style="flex: 1" />
              <n-button @click="selectFolder">Browse</n-button>
            </n-space>
          </div>

          <div>
            <label class="label">Global Arguments:</label>
            <n-input
              v-model:value="configStore.globalArgs"
              type="textarea"
              placeholder="Enter yt-dlp global arguments"
              :rows="4"
            />
          </div>

          <div>
            <label class="label">Auto Update yt-dlp:</label>
            <n-select
              v-model:value="configStore.updateYtdlp"
              :options="[
                { label: 'Enabled', value: true },
                { label: 'Disabled', value: false }
              ]"
            />
          </div>
        </n-space>
      </n-card>

      <!-- Cookie Settings -->
      <n-card title="Cookie Settings" :segmented="{ content: true }">
        <n-space vertical :size="12">
          <div>
            <label class="label">Cookie File Path:</label>
            <n-space :size="8">
              <n-input
                v-model:value="configStore.cookiePath"
                type="text"
                placeholder="Path to cookies.txt file"
                style="flex: 1"
              />
              <n-button @click="selectCookieFile">Select</n-button>
              <n-button v-if="configStore.cookiePath" @click="configStore.clearCookiePath" type="error">
                Clear
              </n-button>
            </n-space>
          </div>
          <p style="font-size: 12px; color: #999; margin: 0">
            Cookie file is used for authenticated downloads. Leave empty to use browser cookies.
          </p>
        </n-space>
      </n-card>

      <!-- Actions -->
      <n-space :size="8">
        <n-button type="primary" @click="saveSettings">Save Settings</n-button>
        <n-button @click="resetSettings">Reset</n-button>
      </n-space>
    </n-space>
  </div>
</template>

<style scoped>
.settings {
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
</style>
