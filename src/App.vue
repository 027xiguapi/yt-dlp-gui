<script setup lang="ts">
import { useRouter, useRoute } from 'vue-router'
import { useI18n } from 'vue-i18n'
import { ref, computed } from 'vue'
import {
  NLayout, NLayoutSider, NMenu, NLayoutContent,
  NMessageProvider, NConfigProvider, zhCN, dateZhCN, enUS, dateEnUS, GlobalThemeOverrides, NButton, NSpace
} from 'naive-ui'
import { Download, Radar, Settings, Clapperboard } from '@lucide/vue'
import { h } from 'vue'

const router = useRouter()
const route = useRoute()
const { locale } = useI18n()

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

// 使用 i18n 的菜单选项
const { t } = useI18n()

const menuOptions = computed(() => [
  { label: t('menu.download'), key: '/', icon: () => h(Download, { size: 18 }) },
  { label: t('menu.channelExtraction'), key: '/channel-extraction', icon: () => h(Clapperboard, { size: 18 }) },
  { label: t('menu.sniffer'), key: '/sniffer', icon: () => h(Radar, { size: 18 }) },
  { label: t('menu.settings'), key: '/settings', icon: () => h(Settings, { size: 18 }) }
])

const naiveLocale = computed(() => locale.value === 'zh' ? zhCN : enUS)
const naiveDateLocale = computed(() => locale.value === 'zh' ? dateZhCN : dateEnUS)

function handleMenuSelect(key: string) { router.push(key) }
</script>

<template>
  <n-config-provider :theme-overrides="themeOverrides" :locale="naiveLocale" :date-locale="naiveDateLocale">
    <n-message-provider>
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
          <n-layout-content class="p-0 h-screen overflow-auto">
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