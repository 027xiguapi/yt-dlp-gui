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
      <n-layout has-sider class="app-wrapper">
        <n-layout-sider
          bordered
          collapse-mode="width"
          :collapsed-width="64"
          :width="220"
          class="app-sider"
        >
          <div class="logo-container">
            <div class="logo-icon">YT</div>
            <span class="logo-text">YT-DLP GUI</span>
          </div>
          <n-menu
            :value="route.path"
            :options="menuOptions"
            @update:value="handleMenuSelect"
          />
        </n-layout-sider>
        <n-layout class="main-layout">
          <n-layout-content class="content-scroll">
            <router-view />
          </n-layout-content>
        </n-layout>
      </n-layout>
    </n-message-provider>
  </n-config-provider>
</template>

<style scoped>
.app-wrapper { height: 100vh; background-color: #f9fafb; }
.app-sider { background-color: #ffffff; }
.logo-container {
  padding: 24px 16px;
  display: flex;
  align-items: center;
  gap: 12px;
}
.logo-icon {
  background: linear-gradient(135deg, #3b82f6, #8b5cf6);
  color: white;
  width: 32px;
  height: 32px;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 8px;
  font-weight: bold;
}
.logo-text { font-size: 18px; font-weight: 700; color: #111827; }
.main-layout { background-color: #f3f4f6; }
.content-scroll { padding: 0; height: 100vh; }
</style>