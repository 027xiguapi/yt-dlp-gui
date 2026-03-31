<script setup lang="ts">
import { useRouter, useRoute } from 'vue-router'
import { NLayout, NLayoutSider, NMenu, NLayoutContent } from 'naive-ui'
import { Download, Radar, Settings } from '@lucide/vue'
import { h } from 'vue'

const router = useRouter()
const route = useRoute()

const menuOptions = [
  {
    label: 'Downloader',
    key: '/',
    icon: () => h(Download, { size: 18 })
  },
  {
    label: 'Resource Sniffer',
    key: '/sniffer',
    icon: () => h(Radar, { size: 18 })
  },
  {
    label: 'Settings',
    key: '/settings',
    icon: () => h(Settings, { size: 18 })
  }
]

function handleMenuSelect(key: string) {
  router.push(key)
}
</script>

<template>
  <n-layout has-sider style="height: 100vh">
    <n-layout-sider
      collapse-mode="width"
      :collapsed-width="64"
      :width="200"
      :native-scrollbar="false"
      style="background-color: #001529"
    >
      <div style="padding: 16px; color: white; font-weight: bold; text-align: center">
        YT-DLP
      </div>
      <n-menu
        :value="route.path"
        :options="menuOptions"
        @update:value="handleMenuSelect"
        :collapsed="false"
        :collapsed-width="64"
        :collapsed-icon-size="22"
      />
    </n-layout-sider>
    <n-layout>
      <n-layout-content style="padding: 0; height: 100vh; overflow-y: auto">
        <router-view />
      </n-layout-content>
    </n-layout>
  </n-layout>
</template>

<style scoped>
</style>
