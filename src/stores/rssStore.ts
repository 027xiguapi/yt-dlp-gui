import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import * as db from '../services/database'

export interface RssFeed {
  id: number
  channel_id: string
  title: string
  url: string
  description: string
  thumbnail: string
  last_checked: string
  auto_refresh: number
  refresh_interval_minutes: number
  created_at: string
  updated_at: string
}

export interface RssItem {
  id: number
  feed_id: number
  video_id: string
  title: string
  url: string
  thumbnail: string
  published_at: string
  downloaded: number
  download_task_id: string
  created_at: string
}

export interface RssFeedPreview {
  channel_id: string
  title: string
  url: string
  thumbnail: string
  description: string
  items: RssFeedPreviewItem[]
}

export interface RssFeedPreviewItem {
  video_id: string
  title: string
  url: string
  thumbnail: string
  published_at: string
}

export const useRssStore = defineStore('rss', () => {
  const feeds = ref<RssFeed[]>([])
  const items = ref<RssItem[]>([])
  const selectedFeedId = ref<number | null>(null)
  const isLoading = ref(false)
  const preview = ref<RssFeedPreview | null>(null)
  const error = ref('')

  let autoRefreshTimer: ReturnType<typeof setInterval> | null = null

  const selectedFeed = computed(() =>
    feeds.value.find(f => f.id === selectedFeedId.value) || null
  )

  async function loadFeeds() {
    try {
      feeds.value = await db.getRssFeeds()
    } catch (err) {
      console.error('Failed to load RSS feeds:', err)
    }
  }

  async function loadItems(feedId?: number) {
    const id = feedId || selectedFeedId.value
    if (!id) return
    try {
      items.value = await db.getRssItems(id)
    } catch (err) {
      console.error('Failed to load RSS items:', err)
    }
  }

  async function previewFeed(url: string) {
    isLoading.value = true
    error.value = ''
    try {
      preview.value = await invoke<RssFeedPreview>('parse_rss_feed', { url })
    } catch (err) {
      error.value = String(err)
      preview.value = null
    } finally {
      isLoading.value = false
    }
  }

  async function subscribe(url: string) {
    isLoading.value = true
    error.value = ''
    try {
      const feedPreview = await invoke<RssFeedPreview>('parse_rss_feed', { url })
      if (!feedPreview.channel_id) {
        error.value = 'Invalid RSS feed: missing channel ID'
        return
      }

      await db.saveRssFeed({
        channel_id: feedPreview.channel_id,
        title: feedPreview.title,
        url,
        description: feedPreview.description,
        thumbnail: feedPreview.thumbnail,
      })

      await loadFeeds()

      // Find the newly created feed and save its items
      const newFeed = feeds.value.find(f => f.channel_id === feedPreview.channel_id)
      if (newFeed) {
        for (const item of feedPreview.items) {
          await db.saveRssItem({
            feed_id: newFeed.id,
            video_id: item.video_id,
            title: item.title,
            url: item.url,
            thumbnail: item.thumbnail,
            published_at: item.published_at,
          })
        }
        selectedFeedId.value = newFeed.id
        await loadItems(newFeed.id)
      }

      preview.value = null
    } catch (err) {
      error.value = String(err)
    } finally {
      isLoading.value = false
    }
  }

  async function unsubscribe(feedId: number) {
    await db.deleteRssFeed(feedId)
    if (selectedFeedId.value === feedId) {
      selectedFeedId.value = null
      items.value = []
    }
    await loadFeeds()
  }

  async function refreshFeed(feedId: number) {
    const feed = feeds.value.find(f => f.id === feedId)
    if (!feed) return

    isLoading.value = true
    error.value = ''
    try {
      const feedPreview = await invoke<RssFeedPreview>('parse_rss_feed', { url: feed.url })
      for (const item of feedPreview.items) {
        await db.saveRssItem({
          feed_id: feedId,
          video_id: item.video_id,
          title: item.title,
          url: item.url,
          thumbnail: item.thumbnail,
          published_at: item.published_at,
        })
      }
      await db.updateRssFeed(feedId, {
        title: feedPreview.title || feed.title,
        thumbnail: feedPreview.thumbnail || feed.thumbnail,
        last_checked: new Date().toISOString(),
      })
      await loadFeeds()
      await loadItems(feedId)
    } catch (err) {
      error.value = String(err)
    } finally {
      isLoading.value = false
    }
  }

  async function refreshAll() {
    for (const feed of feeds.value) {
      if (feed.auto_refresh) {
        const lastChecked = feed.last_checked ? new Date(feed.last_checked) : new Date(0)
        const intervalMs = feed.refresh_interval_minutes * 60 * 1000
        if (Date.now() - lastChecked.getTime() >= intervalMs) {
          await refreshFeed(feed.id)
        }
      }
    }
  }

  async function markItemDownloaded(itemId: number, taskId: string) {
    await db.updateRssItem(itemId, { downloaded: true, download_task_id: taskId })
    await loadItems()
  }

  function selectFeed(feedId: number | null) {
    selectedFeedId.value = feedId
    if (feedId) {
      loadItems(feedId)
    } else {
      items.value = []
    }
  }

  function clearPreview() {
    preview.value = null
    error.value = ''
  }

  function startAutoRefresh() {
    if (autoRefreshTimer) return
    autoRefreshTimer = setInterval(() => {
      refreshAll()
    }, 60 * 1000)
  }

  function stopAutoRefresh() {
    if (autoRefreshTimer) {
      clearInterval(autoRefreshTimer)
      autoRefreshTimer = null
    }
  }

  return {
    feeds,
    items,
    selectedFeedId,
    isLoading,
    preview,
    error,
    selectedFeed,
    loadFeeds,
    loadItems,
    previewFeed,
    subscribe,
    unsubscribe,
    refreshFeed,
    refreshAll,
    markItemDownloaded,
    selectFeed,
    clearPreview,
    startAutoRefresh,
    stopAutoRefresh,
  }
})
