import { defineStore } from 'pinia'
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'

export interface ChannelExtractionResult {
  urls: Vec<string>
  channel_name: string
  total_videos: number
}

export const useChannelStore = defineStore('channel', () => {
  const channelUrl = ref('')
  const isExtracting = ref(false)
  const extractionProgress = ref(0)
  const extractedUrls = ref<string[]>([])
  const extractedChannelName = ref('')
  const error = ref('')

  async function extractChannelUrls(url: string) {
    if (!url.trim()) {
      error.value = 'Please enter a YouTube channel URL'
      return false
    }

    isExtracting.value = true
    extractionProgress.value = 0
    extractedUrls.value = []
    extractedChannelName.value = ''
    error.value = ''

    try {
      const result = await invoke<any>('extract_channel_urls', {
        channelUrl: url,
      })
      extractedUrls.value = result.urls
      extractedChannelName.value = result.channel_name
      return true
    } catch (err) {
      error.value = `Error: ${err}`
      console.error('Failed to extract channel URLs:', err)
      return false
    } finally {
      isExtracting.value = false
    }
  }

  function setChannelUrl(url: string) {
    channelUrl.value = url
  }

  function setExtractionProgress(progress: number) {
    extractionProgress.value = progress
  }

  function clearExtraction() {
    extractedUrls.value = []
    extractedChannelName.value = ''
    channelUrl.value = ''
    error.value = ''
  }

  return {
    channelUrl,
    isExtracting,
    extractionProgress,
    extractedUrls,
    extractedChannelName,
    error,
    extractChannelUrls,
    setChannelUrl,
    setExtractionProgress,
    clearExtraction,
  }
})
