<script setup lang="ts">
import { onMounted, ref } from "vue";
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

const configStore = useConfigStore();
const message = useMessage();

const checking = ref(false);

const browserOptions = [
  { label: "Chrome", value: "chrome" },
  { label: "Firefox", value: "firefox" },
  { label: "Edge", value: "edge" },
  { label: "Safari", value: "safari" },
  { label: "Opera", value: "opera" },
  { label: "自定义文件", value: "custom" }
];

onMounted(async () => {
  await configStore.loadConfig();
});

async function selectFolder() {
  try {
    const selected = await open({
      directory: true,
      multiple: false,
      title: "选择下载保存目录"
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
      title: "选择 Cookie 文件",
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
      title: "选择 yt-dlp 可执行文件",
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
    message.success("配置已成功保存到本地！");
  } catch (error) {
    message.error(`保存失败: ${error}`);
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
  <div class="settings-container">
    <n-space vertical :size="20" style="padding: 24px">
      <header class="settings-header">
        <div class="header-content">
          <n-icon size="28" :component="DownloadCloud" />
          <div class="header-text">
            <h1>软件设置</h1>
            <span>管理下载偏好、核心引擎及身份验证</span>
          </div>
        </div>
      </header>

      <main class="settings-content">
        <n-space vertical :size="20">
          <n-form label-placement="top">
            <n-card title="下载配置" hoverable :segmented="{ content: true }">
              <template #header-extra>
                <n-icon size="20" color="#3b82f6" :component="ShieldCheck" />
              </template>
              
              <n-form-item label="默认下载路径">
                <n-input-group>
                  <n-input 
                    v-model:value="configStore.downloadPath" 
                    placeholder="点击右侧按钮选择路径" 
                    readonly 
                  />
                  <n-button type="primary" secondary @click="selectFolder">
                    <template #icon><n-icon :component="FolderOpen" /></template>
                    浏览
                  </n-button>
                </n-input-group>
              </n-form-item>

              <n-divider />

              <n-form-item label="yt-dlp 可执行文件路径">
                <n-input-group>
                  <n-input
                    v-model:value="configStore.ytdlpPath"
                    placeholder="yt-dlp.exe 绝对路径或相对路径"
                  />
                  <n-button @click="selectYtdlpFile">
                    <template #icon><n-icon :component="FileText" /></template>
                    选择
                  </n-button>
                </n-input-group>
              </n-form-item>

              <n-divider />

              <n-form-item label="Cookie 来源">
                <n-select
                  v-model:value="configStore.cookieBrowser"
                  :options="browserOptions"
                  @update:value="configStore.setCookieBrowser"
                />
              </n-form-item>

              <n-form-item v-if="configStore.cookieBrowser === 'custom'">
                <template #label>
                  <n-space :size="4" align="center">
                    <span>Cookie 文件路径</span>
                    <n-tooltip trigger="hover">
                      <template #trigger>
                        <n-icon :component="Info" color="#999" />
                      </template>
                      使用 Cookie 可以下载会员视频或绕过限制
                    </n-tooltip>
                  </n-space>
                </template>

                <n-input-group>
                  <n-input
                    v-model:value="configStore.cookiePath"
                    placeholder="cookies.txt 绝对路径"
                  />
                  <n-button @click="selectCookieFile">
                    <template #icon><n-icon :component="FileText" /></template>
                    选择
                  </n-button>
                  <n-button v-if="configStore.cookiePath" type="error" ghost @click="configStore.clearCookiePath">
                    清除
                  </n-button>
                </n-input-group>
              </n-form-item>
              <p class="hint">{{ configStore.cookieBrowser === 'custom' ? '选择自定义 Cookie 文件' : '将从 ' + browserOptions.find(b => b.value === configStore.cookieBrowser)?.label + ' 浏览器获取 Cookie' }}</p>
            </n-card>

            <n-card title="环境变量测试" hoverable :segmented="{ content: true }">
              <template #header-extra>
                <n-icon size="20" color="#10b981" :component="Terminal" />
              </template>

              <n-space vertical :size="12">
                <p class="hint">检查系统环境中的依赖工具版本</p>

                <n-button type="primary" :loading="checking" @click="checkVersions">
                  <template #icon><n-icon :component="Terminal" /></template>
                  检测版本
                </n-button>

                <n-divider v-if="configStore.versions.ytdlp || configStore.versions.deno || configStore.versions.ffmpeg || configStore.versions.ffprobe" />

                <n-space v-if="configStore.versions.ytdlp || configStore.versions.deno || configStore.versions.ffmpeg || configStore.versions.ffprobe" vertical :size="8">
                  <div v-if="configStore.versions.ytdlp">
                    <strong>yt-dlp:</strong> <span style="font-family: monospace; color: #666;">{{ configStore.versions.ytdlp }}</span>
                  </div>
                  <div v-if="configStore.versions.deno">
                    <strong>deno:</strong> <span style="font-family: monospace; color: #666;">{{ configStore.versions.deno }}</span>
                  </div>
                  <div v-if="configStore.versions.ffmpeg">
                    <strong>ffmpeg:</strong> <span style="font-family: monospace; color: #666;">{{ configStore.versions.ffmpeg }}</span>
                  </div>
                  <div v-if="configStore.versions.ffprobe">
                    <strong>ffprobe:</strong> <span style="font-family: monospace; color: #666;">{{ configStore.versions.ffprobe }}</span>
                  </div>
                </n-space>
              </n-space>
            </n-card>
          </n-form>

          <div class="actions-bar">
            <n-space justify="end" :size="12">
              <n-button strong secondary @click="resetSettings">
                <template #icon><n-icon :component="RotateCcw" /></template>
                重置
              </n-button>
              <n-button strong type="primary" size="large" @click="saveSettings">
                <template #icon><n-icon :component="Save" /></template>
                保存更改
              </n-button>
            </n-space>
          </div>
        </n-space>
      </main>
    </n-space>
  </div>
</template>

<style scoped>
.settings-container {
  display: flex;
  flex-direction: column;
  min-height: 100vh;
  background-color: #f6f8fa;
}

/* Header 优化 */
.settings-header {
  background: #fff;
  padding: 32px 40px;
  border-bottom: 1px solid #e5e7eb;
}

.header-content {
  display: flex;
  align-items: center;
  gap: 16px;
  max-width: 900px;
  margin: 0 auto;
}

.header-text h1 {
  margin: 0;
  font-size: 24px;
  font-weight: 600;
  color: #111827;
}

.header-text span {
  font-size: 14px;
  color: #6b7280;
}

/* 主内容区 */
.settings-content {
  flex: 1;
  max-width: 100%;
  margin: 0 auto;
}

/* 行布局优化 */
.setting-row {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 8px 0;
}

.row-info {
  display: flex;
  flex-direction: column;
}

.row-label {
  font-weight: 500;
  font-size: 15px;
  color: #1f2937;
}

.row-desc {
  font-size: 13px;
  color: #6b7280;
}

.hint {
  font-size: 12px;
  color: #9ca3af;
  margin-top: 8px;
}

/* 底部栏 */
.actions-bar {
  margin-top: 24px;
  padding: 20px 0;
  border-top: 1px dashed #d1d5db;
}

/* 动画 */
.fade-enter-active, .fade-leave-active {
  transition: opacity 0.3s;
}
.fade-enter-from, .fade-leave-to {
  opacity: 0;
}
</style>