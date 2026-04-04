<script setup lang="ts">
import { ref, reactive, computed, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import {
  NButton, NCard, NSpace, NTag, NProgress,
  NStatistic, NGrid, NGridItem, NCollapse, NCollapseItem, NTooltip
} from 'naive-ui'
import { CheckCircle, XCircle, Loader2, RefreshCw, Play } from '@lucide/vue'
import { useConfigStore } from '../stores/configStore'

const configStore = useConfigStore()

type TestStatus = 'pending' | 'testing' | 'success' | 'error'

interface TestItem {
  name: string
  url: string
  status: TestStatus
  result?: string
  error?: string
}

interface TestCategory {
  label: string
  icon: string
  items: TestItem[]
}

const isTestingAll = ref(false)
const testedCount = ref(0)
const successCount = ref(0)
const errorCount = ref(0)

const categories = reactive<TestCategory[]>([
  {
    label: '全球主流平台',
    icon: '🌍',
    items: [
      { name: 'YouTube（视频）', url: 'https://www.youtube.com/watch?v=dQw4w9WgXcQ', status: 'pending' },
      { name: 'YouTube Shorts', url: 'https://www.youtube.com/shorts/dQw4w9WgXcQ', status: 'pending' },
      { name: 'Vimeo', url: 'https://vimeo.com/824804225', status: 'pending' },
      { name: 'Dailymotion', url: 'https://www.dailymotion.com/video/x8c5j4l', status: 'pending' },
      { name: 'Twitch', url: 'https://www.twitch.tv/videos/1904128456', status: 'pending' },
      { name: 'Facebook 视频', url: 'https://www.facebook.com/reel/1234567890', status: 'pending' },
      { name: 'Twitter / X', url: 'https://x.com/elaboratestuff/status/1746704799002718458', status: 'pending' },
      { name: 'TikTok', url: 'https://www.tiktok.com/@scout2015/video/6718335390845095173', status: 'pending' },
      { name: 'Reddit', url: 'https://www.reddit.com/r/oddlysatisfying/comments/1b5i7rq/', status: 'pending' },
      { name: 'SoundCloud', url: 'https://soundcloud.com/imaginedragons/whatever-it-takes', status: 'pending' },
      { name: 'Niconico', url: 'https://www.nicovideo.jp/watch/sm9', status: 'pending' },
    ]
  },
  {
    label: '国内主流平台',
    icon: '🇨🇳',
    items: [
      { name: 'Bilibili（B站）', url: 'https://www.bilibili.com/video/BV1GJ411x7h7', status: 'pending' },
      { name: 'B站番剧', url: 'https://www.bilibili.com/bangumi/play/ep639250', status: 'pending' },
      { name: '抖音 Douyin', url: 'https://www.douyin.com/video/7126708528793936147', status: 'pending' },
      { name: '快手', url: 'https://www.kuaishou.com/short-video/3x8h5ibfn7k8jfe', status: 'pending' },
      { name: '小红书', url: 'https://www.xiaohongshu.com/explore/65f5a9e6000000000702b9b7', status: 'pending' },
      { name: '西瓜视频', url: 'https://www.ixigua.com/7358888414263478799', status: 'pending' },
      { name: '优酷', url: 'https://v.youku.com/v_show/id_XNTgxOTM5NjM0NA==.html', status: 'pending' },
      { name: '微博视频', url: 'https://video.weibo.com/show?fid=1034:4abc123def456', status: 'pending' },
      { name: 'AcFun', url: 'https://www.acfun.cn/v/ac30825469', status: 'pending' },
      { name: '网易云音乐', url: 'https://music.163.com/#/mv?id=1436709601', status: 'pending' },
    ]
  },
  {
    label: '新闻 / 媒体 / 教育',
    icon: '📺',
    items: [
      { name: 'TED', url: 'https://www.ted.com/talks/tim_urban_inside_the_mind_of_a_master_procrastinator', status: 'pending' },
      { name: 'BBC', url: 'https://www.bbc.co.uk/programmes/p0glb7rk', status: 'pending' },
      { name: 'CNN', url: 'https://edition.cnn.com/videos/world/2024/01/01/', status: 'pending' },
      { name: 'NHK', url: 'https://www.nhk.or.jp/nhkworld/', status: 'pending' },
      { name: 'YouTube 教育', url: 'https://www.youtube.com/watch?v=rfscVS0vtbw', status: 'pending' },
    ]
  }
])

const allItems = computed(() => {
  const items: { item: TestItem; category: string }[] = []
  for (const cat of categories) {
    for (const item of cat.items) {
      items.push({ item, category: cat.label })
    }
  }
  return items
})

function getStatusIcon(status: TestStatus) {
  switch (status) {
    case 'success': return CheckCircle
    case 'error': return XCircle
    case 'testing': return Loader2
    default: return Play
  }
}

function getStatusLabel(status: TestStatus) {
  switch (status) {
    case 'success': return '通过'
    case 'error': return '失败'
    case 'testing': return '测试中'
    default: return '待测试'
  }
}

async function testSingle(item: TestItem) {
  item.status = 'testing'
  item.result = ''
  item.error = ''
  try {
    const info = await invoke<any>('get_video_info', {
      url: item.url,
      ytdlpPath: configStore.ytdlpPath || undefined,
    })
    item.status = 'success'
    item.result = info.title || '成功获取视频信息'
    successCount.value++
  } catch (err: any) {
    item.status = 'error'
    const errMsg = String(err)
    item.error = errMsg.length > 200 ? errMsg.substring(0, 200) + '...' : errMsg
    errorCount.value++
  }
  testedCount.value++
}

async function testAll() {
  if (isTestingAll.value) return
  isTestingAll.value = true
  testedCount.value = 0
  successCount.value = 0
  errorCount.value = 0

  // Reset all to pending first
  for (const cat of categories) {
    for (const item of cat.items) {
      item.status = 'pending'
      item.result = ''
      item.error = ''
    }
  }

  // Test all items sequentially to avoid overwhelming the system
  for (const cat of categories) {
    for (const item of cat.items) {
      await testSingle(item)
      // Small delay between tests
      await new Promise(resolve => setTimeout(resolve, 500))
    }
  }

  isTestingAll.value = false
}

function resetAll() {
  testedCount.value = 0
  successCount.value = 0
  errorCount.value = 0
  for (const cat of categories) {
    for (const item of cat.items) {
      item.status = 'pending'
      item.result = ''
      item.error = ''
    }
  }
}

onMounted(async () => {
  await configStore.loadConfig()
})
</script>

<template>
  <div class="h-screen w-full bg-gray-50 overflow-y-auto overflow-x-auto" style="min-width: 700px">
    <n-space vertical :size="16" class="p-5">
      <!-- Header -->
      <div class="bg-gradient-to-r from-teal-500 to-cyan-600 text-white p-5 rounded-none shadow-sm">
        <div class="flex items-center justify-between">
          <div>
            <h1 class="m-0 text-2xl font-bold">平台兼容性测试</h1>
            <p class="mt-1 text-teal-100 text-sm">测试 yt-dlp 对各视频平台的访问与解析能力</p>
          </div>
          <n-space>
            <n-button quaternary circle @click="resetAll" class="text-white hover:text-teal-100">
              <template #icon>
                <n-icon :size="18"><RefreshCw /></n-icon>
              </template>
            </n-button>
            <n-button
              type="primary"
              :loading="isTestingAll"
              @click="testAll"
            >
              <template #icon>
                <n-icon :size="16"><Play /></n-icon>
              </template>
              {{ isTestingAll ? `测试中 (${testedCount}/${allItems.length})` : '开始全部测试' }}
            </n-button>
          </n-space>
        </div>
      </div>

      <!-- Statistics -->
      <n-grid :cols="4" :x-gap="12">
        <n-grid-item>
          <n-card size="small">
            <n-statistic label="总平台数" :value="allItems.length" />
          </n-card>
        </n-grid-item>
        <n-grid-item>
          <n-card size="small">
            <n-statistic label="已测试">
              <template #default>
                <n-progress
                  type="line"
                  :percentage="allItems.length ? Math.round(testedCount / allItems.length * 100) : 0"
                  :height="20"
                  :show-indicator="true"
                  indicator-placement="inside"
                />
              </template>
            </n-statistic>
          </n-card>
        </n-grid-item>
        <n-grid-item>
          <n-card size="small">
            <n-statistic label="通过" :value="successCount">
              <template #prefix>
                <n-icon color="#10b981"><CheckCircle /></n-icon>
              </template>
            </n-statistic>
          </n-card>
        </n-grid-item>
        <n-grid-item>
          <n-card size="small">
            <n-statistic label="失败" :value="errorCount">
              <template #prefix>
                <n-icon color="#ef4444"><XCircle /></n-icon>
              </template>
            </n-statistic>
          </n-card>
        </n-grid-item>
      </n-grid>

      <!-- Platform Categories -->
      <n-collapse :default-expanded-names="categories.map((_, i) => String(i))" arrow-placement="right">
        <n-collapse-item
          v-for="(category, catIdx) in categories"
          :key="catIdx"
          :title="`${category.icon} ${category.label}`"
          :name="String(catIdx)"
        >
          <template #header-extra>
            <n-space :size="4">
              <n-tag size="small" type="success" round v-if="category.items.filter(i => i.status === 'success').length > 0">
                {{ category.items.filter(i => i.status === 'success').length }} 通过
              </n-tag>
              <n-tag size="small" type="error" round v-if="category.items.filter(i => i.status === 'error').length > 0">
                {{ category.items.filter(i => i.status === 'error').length }} 失败
              </n-tag>
              <n-tag size="small" round>
                {{ category.items.length }} 个平台
              </n-tag>
            </n-space>
          </template>

          <div class="grid grid-cols-1 gap-2">
            <div
              v-for="(item, itemIdx) in category.items"
              :key="itemIdx"
              class="flex items-center gap-3 p-3 rounded-lg border transition-colors"
              :class="{
                'border-green-200 bg-green-50': item.status === 'success',
                'border-red-200 bg-red-50': item.status === 'error',
                'border-yellow-200 bg-yellow-50': item.status === 'testing',
                'border-gray-200 bg-white': item.status === 'pending',
              }"
            >
              <!-- Status icon -->
              <n-icon
                :size="20"
                :color="{
                  success: '#10b981',
                  error: '#ef4444',
                  testing: '#f59e0b',
                  pending: '#9ca3af',
                }[item.status]"
                :class="{ 'animate-spin': item.status === 'testing' }"
              >
                <component :is="getStatusIcon(item.status)" />
              </n-icon>

              <!-- Platform name -->
              <div class="w-36 flex-shrink-0">
                <span class="text-sm font-medium">{{ item.name }}</span>
              </div>

              <!-- URL -->
              <div class="flex-1 min-w-0">
                <n-tooltip trigger="hover">
                  <template #trigger>
                    <span class="text-xs text-gray-400 truncate block cursor-pointer">{{ item.url }}</span>
                  </template>
                  {{ item.url }}
                </n-tooltip>
              </div>

              <!-- Result -->
              <div class="w-64 flex-shrink-0 text-right">
                <span v-if="item.result" class="text-xs text-green-700">{{ item.result }}</span>
                <n-tooltip v-else-if="item.error" trigger="hover">
                  <template #trigger>
                    <span class="text-xs text-red-600 truncate block cursor-pointer">{{ item.error }}</span>
                  </template>
                  {{ item.error }}
                </n-tooltip>
                <span v-else class="text-xs text-gray-400">{{ getStatusLabel(item.status) }}</span>
              </div>

              <!-- Test button -->
              <n-button
                size="tiny"
                :loading="item.status === 'testing'"
                :disabled="item.status === 'testing'"
                @click="testSingle(item)"
              >
                {{ item.status === 'testing' ? '测试中...' : item.status === 'pending' ? '测试' : '重试' }}
              </n-button>
            </div>
          </div>
        </n-collapse-item>
      </n-collapse>

      <!-- Footer -->
      <div class="text-center text-xs text-gray-400 py-4">
        测试结果取决于网络环境、Cookie 配置和 yt-dlp 版本。部分平台可能需要代理或 Cookie 支持。
      </div>
    </n-space>
  </div>
</template>

<style scoped>
.animate-spin {
  animation: spin 1s linear infinite;
}
@keyframes spin {
  from { transform: rotate(0deg); }
  to { transform: rotate(360deg); }
}
</style>
