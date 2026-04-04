<script setup lang="ts">
import { ref, onMounted, onUnmounted, computed } from 'vue'
import { useI18n } from 'vue-i18n'
import { useRouter } from 'vue-router'
import {
  NButton, NInput, NCard, NSpace, NEmpty, NModal, NCheckbox, useMessage,
  NImage, NTag, NIcon, NSpin, NList, NListItem, NThing, NAvatar, NFlex
} from 'naive-ui'
import { Rss, RefreshCw, Plus, Trash2, Download, Check, Eye } from '@lucide/vue'
import { useRssStore } from '../stores/rssStore'
import { useDownloadStore } from '../stores/downloadStore'
import { useConfigStore } from '../stores/configStore'

const router = useRouter()
const { t } = useI18n()
const rssStore = useRssStore()
const downloadStore = useDownloadStore()
const configStore = useConfigStore()
const message = useMessage()

// Add subscription dialog
const showAddDialog = ref(false)
const showPreviewDialog = ref(false)
const rssUrl = ref('')
const selectedVideoIds = ref<Set<string>>(new Set())

// Loading states
const isRefreshing = ref(false)

onMounted(async () => {
  await configStore.loadConfig()
  await rssStore.loadFeeds()
  rssStore.startAutoRefresh()
})

onUnmounted(() => {
  rssStore.stopAutoRefresh()
})

function formatTime(dateStr: string) {
  if (!dateStr) return t('rss.neverChecked')
  try {
    const date = new Date(dateStr)
    return date.toLocaleString()
  } catch {
    return dateStr
  }
}

async function handlePreview() {
  if (!rssUrl.value.trim()) return
  await rssStore.previewFeed(rssUrl.value.trim())
  if (rssStore.preview) {
    showPreviewDialog.value = true
  } else {
    message.error(t('rss.parseFailed', { error: rssStore.error }))
  }
}

async function handleSubscribe() {
  if (!rssUrl.value.trim()) return
  await rssStore.subscribe(rssUrl.value.trim())
  if (!rssStore.error) {
    message.success(t('rss.subscribeSuccess'))
    showPreviewDialog.value = false
    showAddDialog.value = false
    rssUrl.value = ''
  } else {
    message.error(t('rss.parseFailed', { error: rssStore.error }))
  }
}

function handleUnsubscribe(feedId: number) {
  rssStore.unsubscribe(feedId)
  message.success(t('rss.unsubscribeSuccess'))
}

async function handleRefresh(feedId: number) {
  isRefreshing.value = true
  await rssStore.refreshFeed(feedId)
  isRefreshing.value = false
  if (!rssStore.error) {
    message.success(t('rss.refreshSuccess'))
  }
}

async function handleRefreshAll() {
  isRefreshing.value = true
  await rssStore.refreshAll()
  isRefreshing.value = false
  if (!rssStore.error) {
    message.success(t('rss.refreshSuccess'))
  }
}

function toggleSelectVideo(videoId: string) {
  if (selectedVideoIds.value.has(videoId)) {
    selectedVideoIds.value.delete(videoId)
  } else {
    selectedVideoIds.value.add(videoId)
  }
  // Force reactivity
  selectedVideoIds.value = new Set(selectedVideoIds.value)
}

function selectAll() {
  selectedVideoIds.value = new Set(rssStore.items.map(i => i.video_id))
}

function deselectAll() {
  selectedVideoIds.value = new Set()
}

const allSelected = computed(() =>
  rssStore.items.length > 0 && selectedVideoIds.value.size === rssStore.items.length
)

async function addSelectedToQueue() {
  if (selectedVideoIds.value.size === 0) {
    message.warning(t('rss.noSelected'))
    return
  }
  if (!configStore.downloadPath) {
    message.warning(t('rss.noDownloadPath'))
    return
  }

  const selectedItems = rssStore.items.filter(i => selectedVideoIds.value.has(i.video_id))
  for (const item of selectedItems) {
    await downloadStore.addTask(
      item.url,
      configStore.selectedPreset,
      configStore.downloadPath,
      { title: item.title, thumbnail: item.thumbnail }
    )
    await rssStore.markItemDownloaded(item.id, '')
  }

  selectedVideoIds.value = new Set()
  message.success(t('channelExtraction.addSuccess', { count: selectedItems.length }))
}
</script>

<template>
  <div class="h-screen w-full bg-gray-50 overflow-y-auto overflow-x-auto" style="min-width: 700px">
    <n-space vertical :size="16" class="p-5">
      <!-- Header -->
      <div class="bg-gradient-to-r from-orange-500 to-orange-600 text-white p-5 rounded-none shadow-sm">
        <div class="flex items-center justify-between">
          <div>
            <h1 class="m-0 text-2xl font-bold">{{ t('rss.title') }}</h1>
            <p class="mt-1 text-orange-100 text-sm">{{ t('rss.subtitle') }}</p>
          </div>
          <n-space>
            <n-button
              quaternary
              circle
              :loading="isRefreshing"
              @click="handleRefreshAll"
              class="text-white hover:text-orange-100"
            >
              <template #icon>
                <n-icon :size="18"><RefreshCw /></n-icon>
              </template>
            </n-button>
            <n-button type="primary" @click="showAddDialog = true">
              <template #icon>
                <n-icon :size="16"><Plus /></n-icon>
              </template>
              {{ t('rss.addSubscribe') }}
            </n-button>
          </n-space>
        </div>
      </div>

      <!-- Main Content -->
      <div class="flex gap-4" style="min-height: 500px">
        <!-- Left Sidebar: Feed List -->
        <div class="w-72 flex-shrink-0">
          <n-card :title="t('rss.feeds')" size="small" class="h-full">
            <template #header-extra>
              <n-tag size="small" round>{{ rssStore.feeds.length }}</n-tag>
            </template>

            <n-spin :show="rssStore.isLoading">
              <div v-if="rssStore.feeds.length === 0">
                <n-empty :description="t('rss.noFeeds')" size="small" class="py-8" />
              </div>

              <div v-else class="space-y-1">
                <div
                  v-for="feed in rssStore.feeds"
                  :key="feed.id"
                  class="flex items-center gap-2 p-2 rounded-lg cursor-pointer transition-colors"
                  :class="rssStore.selectedFeedId === feed.id ? 'bg-blue-50 border border-blue-200' : 'hover:bg-gray-100'"
                  @click="rssStore.selectFeed(feed.id)"
                >
                  <n-avatar
                    v-if="feed.thumbnail"
                    :src="feed.thumbnail"
                    :size="36"
                    round
                    class="flex-shrink-0"
                  />
                  <n-avatar v-else :size="36" round class="flex-shrink-0 bg-orange-500">
                    {{ feed.title.charAt(0).toUpperCase() }}
                  </n-avatar>
                  <div class="flex-1 min-w-0">
                    <div class="text-sm font-medium truncate">{{ feed.title }}</div>
                    <div class="text-xs text-gray-400 truncate">
                      {{ t('rss.lastChecked') }}: {{ formatTime(feed.last_checked) }}
                    </div>
                  </div>
                  <n-button
                    quaternary
                    circle
                    size="tiny"
                    @click.stop="handleRefresh(feed.id)"
                    :loading="isRefreshing"
                  >
                    <template #icon>
                      <n-icon :size="14"><RefreshCw /></n-icon>
                    </template>
                  </n-button>
                </div>
              </div>
            </n-spin>
          </n-card>
        </div>

        <!-- Right: Video List -->
        <div class="flex-1 min-w-0">
          <n-card v-if="rssStore.selectedFeed" size="small" class="h-full">
            <template #header>
              <div class="flex items-center justify-between w-full pr-2">
                <span>{{ rssStore.selectedFeed.title }}</span>
                <n-space>
                  <n-button size="tiny" @click="allSelected ? deselectAll() : selectAll()">
                    {{ allSelected ? t('rss.deselectAll') : t('rss.selectAll') }}
                  </n-button>
                  <n-button
                    size="tiny"
                    type="primary"
                    :disabled="selectedVideoIds.size === 0"
                    @click="addSelectedToQueue"
                  >
                    <template #icon>
                      <n-icon :size="14"><Download /></n-icon>
                    </template>
                    {{ t('rss.addSelected') }} ({{ selectedVideoIds.size }})
                  </n-button>
                  <n-button
                    size="tiny"
                    type="error"
                    ghost
                    @click="handleUnsubscribe(rssStore.selectedFeed!.id)"
                  >
                    <template #icon>
                      <n-icon :size="14"><Trash2 /></n-icon>
                    </template>
                    {{ t('rss.unsubscribe') }}
                  </n-button>
                </n-space>
              </div>
            </template>

            <n-spin :show="rssStore.isLoading">
              <div v-if="rssStore.items.length === 0">
                <n-empty :description="t('rss.noItems')" size="small" class="py-8" />
              </div>

              <div v-else class="space-y-2">
                <div
                  v-for="item in rssStore.items"
                  :key="item.id"
                  class="flex items-center gap-3 p-3 rounded-lg border border-gray-200 hover:border-blue-300 transition-colors cursor-pointer"
                  :class="{ 'bg-blue-50 border-blue-300': selectedVideoIds.has(item.video_id) }"
                >
                  <n-checkbox
                    :checked="selectedVideoIds.has(item.video_id)"
                    @update:checked="toggleSelectVideo(item.video_id)"
                  />

                  <n-image
                    v-if="item.thumbnail"
                    :src="item.thumbnail"
                    :width="120"
                    :height="68"
                    object-fit="cover"
                    class="rounded-md flex-shrink-0"
                    preview-disabled
                    :fallback-src="''"
                  />
                  <div
                    v-else
                    class="w-[120px] h-[68px] rounded-md flex-shrink-0 bg-gray-200 flex items-center justify-center"
                  >
                    <n-icon :size="24" class="text-gray-400"><Rss /></n-icon>
                  </div>

                  <div class="flex-1 min-w-0">
                    <div class="text-sm font-medium truncate mb-1">{{ item.title }}</div>
                    <div class="flex items-center gap-2 text-xs text-gray-400">
                      <span>{{ formatTime(item.published_at) }}</span>
                      <n-tag
                        :type="item.downloaded ? 'success' : 'default'"
                        size="tiny"
                        round
                      >
                        <template #icon>
                          <n-icon :size="12">
                            <Check v-if="item.downloaded" />
                            <Rss v-else />
                          </n-icon>
                        </template>
                        {{ item.downloaded ? t('rss.downloaded') : t('rss.notDownloaded') }}
                      </n-tag>
                    </div>
                  </div>

                  <n-button
                    text
                    type="primary"
                    @click="() => { selectedVideoIds.add(item.video_id); selectedVideoIds = new Set(selectedVideoIds); addSelectedToQueue() }"
                    :disabled="item.downloaded === 1"
                  >
                    <template #icon>
                      <n-icon :size="16"><Download /></n-icon>
                    </template>
                  </n-button>
                </div>
              </div>
            </n-spin>
          </n-card>

          <n-card v-else class="h-full">
            <n-empty :description="t('rss.noFeeds')" class="py-16" />
          </n-card>
        </div>
      </div>
    </n-space>

    <!-- Add Subscription Dialog -->
    <n-modal
      v-model:show="showAddDialog"
      preset="card"
      :title="t('rss.addSubscribe')"
      style="width: 560px"
    >
      <n-space vertical :size="16">
        <n-input
          v-model:value="rssUrl"
          :placeholder="t('rss.subscribeUrlPlaceholder')"
          type="textarea"
          :autosize="{ minRows: 2, maxRows: 4 }"
        />
        <n-space justify="end">
          <n-button @click="showAddDialog = false">{{ t('rss.cancel') }}</n-button>
          <n-button
            type="primary"
            :loading="rssStore.isLoading"
            :disabled="!rssUrl.trim()"
            @click="handlePreview"
          >
            {{ rssStore.isLoading ? t('rss.previewing') : t('rss.preview') }}
          </n-button>
        </n-space>
      </n-space>
    </n-modal>

    <!-- Preview Dialog -->
    <n-modal
      v-model:show="showPreviewDialog"
      preset="card"
      :title="t('rss.previewTitle')"
      style="width: 700px"
    >
      <div v-if="rssStore.preview" class="space-y-4">
        <!-- Channel Info -->
        <div class="flex items-center gap-4 p-4 bg-gray-50 rounded-lg">
          <n-avatar
            v-if="rssStore.preview.thumbnail"
            :src="rssStore.preview.thumbnail"
            :size="56"
            round
          />
          <div>
            <h3 class="text-lg font-bold m-0">{{ rssStore.preview.title }}</h3>
            <p class="text-sm text-gray-500 m-0 mt-1">
              {{ t('rss.videoCount', { count: rssStore.preview.items.length }) }}
            </p>
          </div>
        </div>

        <!-- Preview Items -->
        <div class="max-h-80 overflow-y-auto space-y-2">
          <div
            v-for="(item, idx) in rssStore.preview.items.slice(0, 10)"
            :key="idx"
            class="flex items-center gap-3 p-2 rounded-lg border border-gray-100"
          >
            <n-image
              v-if="item.thumbnail"
              :src="item.thumbnail"
              :width="96"
              :height="54"
              object-fit="cover"
              class="rounded"
              preview-disabled
              :fallback-src="''"
            />
            <div class="flex-1 min-w-0">
              <div class="text-sm truncate">{{ item.title }}</div>
              <div class="text-xs text-gray-400 mt-1">{{ formatTime(item.published_at) }}</div>
            </div>
          </div>
          <div
            v-if="rssStore.preview.items.length > 10"
            class="text-center text-sm text-gray-400 py-2"
          >
            ... {{ t('rss.videoCount', { count: rssStore.preview.items.length - 10 }) }}
          </div>
        </div>

        <n-space justify="end">
          <n-button @click="showPreviewDialog = false">{{ t('rss.cancel') }}</n-button>
          <n-button
            type="primary"
            :loading="rssStore.isLoading"
            @click="handleSubscribe"
          >
            {{ rssStore.isLoading ? t('rss.subscribing') : t('rss.subscribe') }}
          </n-button>
        </n-space>
      </div>

      <!-- Error in preview -->
      <n-empty v-else :description="t('rss.parseFailed', { error: rssStore.error })" />
    </n-modal>
  </div>
</template>

<style scoped>
/* Tailwind CSS handles all styling via utility classes above */
</style>
