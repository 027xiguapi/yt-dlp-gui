<script setup lang="ts">
import { useRouter, useRoute } from 'vue-router'
import {
  NLayout, NLayoutSider, NMenu, NLayoutContent,
  NMessageProvider, NConfigProvider, zhCN, dateZhCN, GlobalThemeOverrides
} from 'naive-ui'
import { Download, Radar, Settings, Clapperboard } from '@lucide/vue'
import { h } from 'vue'

const router = useRouter()
const route = useRoute()

// 自定义主题色 (现代科技蓝)
const themeOverrides: GlobalThemeOverrides = {
  common: {
    primaryColor: '#3b82f6',
    primaryColorHover: '#60a5fa',
    borderRadius: '8px'
  }
}

const menuOptions = [
  { label: '下载管理', key: '/', icon: () => h(Download, { size: 18 }) },
  { label: '频道提取', key: '/channel-extraction', icon: () => h(Clapperboard, { size: 18 }) },
  { label: '资源嗅探', key: '/sniffer', icon: () => h(Radar, { size: 18 }) },
  { label: '软件设置', key: '/settings', icon: () => h(Settings, { size: 18 }) }
]

function handleMenuSelect(key: string) { router.push(key) }
</script>

<template>
  <n-config-provider :theme-overrides="themeOverrides" :locale="zhCN" :date-locale="dateZhCN">
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