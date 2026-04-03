<script setup lang="ts">
import { useRouter, useRoute } from 'vue-router'
import { useI18n } from 'vue-i18n'
import { ref, computed, onMounted } from 'vue'
import {
  NLayout, NLayoutSider, NMenu, NLayoutContent,
  NMessageProvider, NConfigProvider, zhCN, dateZhCN, enUS, dateEnUS, GlobalThemeOverrides, NButton, NSpace,
  NAlert, NIcon, NModal
} from 'naive-ui'
import { Download, Radar, Settings, Clapperboard, AlertCircle, CheckCircle } from '@lucide/vue'
import { h } from 'vue'
import { useConfigStore } from './stores/configStore'
import { initDatabase } from './services/database'

const router = useRouter()
const route = useRoute()
const { locale } = useI18n()
const { t } = useI18n()
const configStore = useConfigStore()

// 环境检查状态
const showEnvironmentCheck = ref(true)
const environmentChecking = ref(false)

// 语言配置
const locales = ref<{ label: string; value: 'zh' | 'en' }[]>([
  { label: '中文', value: 'zh' },
  { label: 'English', value: 'en' }
])

// 切换语言
const handleLocaleChange = (newLocale: 'zh' | 'en') => {
  locale.value = newLocale
  localStorage.setItem('locale', newLocale)
}

// 自定义主题色 (现代科技蓝)
const themeOverrides: GlobalThemeOverrides = {
  common: {
    primaryColor: '#3b82f6',
    primaryColorHover: '#60a5fa',
    borderRadius: '8px'
  }
}

const menuOptions = computed(() => [
  { label: t('menu.download'), key: '/', icon: () => h(Download, { size: 18 }) },
  { label: t('menu.channelExtraction'), key: '/channel-extraction', icon: () => h(Clapperboard, { size: 18 }) },
  { label: t('menu.sniffer'), key: '/sniffer', icon: () => h(Radar, { size: 18 }) },
  { label: t('menu.settings'), key: '/settings', icon: () => h(Settings, { size: 18 }) }
])

const naiveLocale = computed(() => locale.value === 'zh' ? zhCN : enUS)
const naiveDateLocale = computed(() => locale.value === 'zh' ? dateZhCN : dateEnUS)

// 检查是否有工具未安装
const hasToolsNotInstalled = computed(() => {
  return Object.values(configStore.versions).some(v => v.includes('未安装') || v.includes('not installed'))
})

// 所有工具版本都已检测
const hasVersionsDetected = computed(() => {
  return Object.values(configStore.versions).some(v => v && v.length > 0)
})

async function performEnvironmentCheck() {
  environmentChecking.value = true
  await configStore.checkVersions()
  environmentChecking.value = false
}

// 初始化时检查环境
onMounted(async () => {
  await initDatabase()
  await configStore.loadConfig()
  await performEnvironmentCheck()
  setTimeout(() => {
    showEnvironmentCheck.value = hasToolsNotInstalled.value
  }, 500)
})

function handleMenuSelect(key: string) { router.push(key) }

function closeEnvironmentCheck() {
  showEnvironmentCheck.value = false
}
</script>

<template>
  <n-config-provider :theme-overrides="themeOverrides" :locale="naiveLocale" :date-locale="naiveDateLocale">
    <n-message-provider>
      <!-- 环境检查弹窗 -->
      <n-modal
        v-model:show="showEnvironmentCheck"
        :title="t('app.environmentCheck')"
        preset="dialog"
        type="warning"
        @positive-click="closeEnvironmentCheck"
        :mask-closable="false"
      >
        <n-alert v-if="!hasVersionsDetected" type="info" class="mb-4">
          {{ t('app.checkingEnvironment') }}
        </n-alert>

        <n-alert
          v-if="hasVersionsDetected && hasToolsNotInstalled"
          type="warning"
          class="mb-4"
        >
          {{ t('app.toolsNotFound') }}
        </n-alert>

        <n-alert
          v-if="hasVersionsDetected && !hasToolsNotInstalled"
          type="success"
          class="mb-4"
        >
          {{ t('app.allToolsReady') }}
        </n-alert>

        <n-space vertical :size="8" class="mt-4">
          <div v-if="configStore.versions.ytdlp" class="flex items-center gap-2">
            <n-icon :component="configStore.versions.ytdlp.includes('未安装') || configStore.versions.ytdlp.includes('not installed') ? AlertCircle : CheckCircle" :color="configStore.versions.ytdlp.includes('未安装') || configStore.versions.ytdlp.includes('not installed') ? '#ef4444' : '#10b981'" size="18" />
            <strong class="w-24 text-center flex-shrink-0">yt-dlp:</strong>
            <span class="font-mono text-gray-600">{{ configStore.versions.ytdlp }}</span>
          </div>
          <div v-if="configStore.versions.deno" class="flex items-center gap-2">
            <n-icon :component="configStore.versions.deno.includes('未安装') || configStore.versions.deno.includes('not installed') ? AlertCircle : CheckCircle" :color="configStore.versions.deno.includes('未安装') || configStore.versions.deno.includes('not installed') ? '#ef4444' : '#10b981'" size="18" />
            <strong class="w-24 text-center flex-shrink-0">deno:</strong>
            <span class="font-mono text-gray-600">{{ configStore.versions.deno }}</span>
          </div>
          <div v-if="configStore.versions.ffmpeg" class="flex items-center gap-2">
            <n-icon :component="configStore.versions.ffmpeg.includes('未安装') || configStore.versions.ffmpeg.includes('not installed') ? AlertCircle : CheckCircle" :color="configStore.versions.ffmpeg.includes('未安装') || configStore.versions.ffmpeg.includes('not installed') ? '#ef4444' : '#10b981'" size="18" />
            <strong class="w-24 text-center flex-shrink-0">ffmpeg:</strong>
            <span class="font-mono text-gray-600">{{ configStore.versions.ffmpeg }}</span>
          </div>
          <div v-if="configStore.versions.ffprobe" class="flex items-center gap-2">
            <n-icon :component="configStore.versions.ffprobe.includes('未安装') || configStore.versions.ffprobe.includes('not installed') ? AlertCircle : CheckCircle" :color="configStore.versions.ffprobe.includes('未安装') || configStore.versions.ffprobe.includes('not installed') ? '#ef4444' : '#10b981'" size="18" />
            <strong class="w-24 text-center flex-shrink-0">ffprobe:</strong>
            <span class="font-mono text-gray-600">{{ configStore.versions.ffprobe }}</span>
          </div>
        </n-space>
      </n-modal>

      <n-layout has-sider class="h-screen bg-gray-50">
        <n-layout-sider
          bordered
          collapse-mode="width"
          :collapsed-width="64"
          :width="220"
          class="bg-white"
        >
          <div class="flex items-center gap-3 px-4 py-6 border-b border-gray-200">
            <img src="/app-icon.png" alt="Video-DLP" class="w-8 h-8 rounded-lg" />
            <span class="text-lg font-bold text-gray-900">Video-DLP GUI</span>
          </div>
          <n-menu
            :value="route.path"
            :options="menuOptions"
            @update:value="handleMenuSelect"
          />

          <!-- 语言切换 -->
          <div class="absolute bottom-4 left-4 right-4 p-4 border-t border-gray-200">
            <n-space vertical fill>
              <div v-for="loc in locales" :key="loc.value" class="flex items-center">
                <n-button
                  :type="locale === loc.value ? 'primary' : 'default'"
                  size="small"
                  block
                  @click="handleLocaleChange(loc.value)"
                >
                  {{ loc.label }}
                </n-button>
              </div>
            </n-space>
          </div>
        </n-layout-sider>
        <n-layout class="bg-gray-100">
          <n-layout-content class="p-0 h-screen overflow-x-scroll">
            <router-view />
          </n-layout-content>
        </n-layout>
      </n-layout>
    </n-message-provider>
  </n-config-provider>
</template>

<style scoped>
/* Tailwind CSS handles all styling via utility classes above */
</style>