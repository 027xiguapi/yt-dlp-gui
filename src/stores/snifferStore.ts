import { defineStore } from 'pinia'
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'

export interface CapturedResource {
  url: string
  resource_type: string
  mime_type: string
  size: number
}

export const useSnifferStore = defineStore('sniffer', () => {
  const videoUrl = ref('')
  const isSniffing = ref(false)
  const sniffProgress = ref(0)
  const videos = ref<CapturedResource[]>([])
  const images = ref<CapturedResource[]>([])
  const error = ref('')

  async function sniffResources(url: string) {
    if (!url.trim()) {
      error.value = 'Please enter a YouTube video URL'
      return false
    }

    isSniffing.value = true
    sniffProgress.value = 0
    videos.value = []
    images.value = []
    error.value = ''

    try {
      const [videoResources, imageResources] = await invoke<[CapturedResource[], CapturedResource[]]>(
        'sniff_youtube_resources',
        { videoUrl: url }
      )
      videos.value = videoResources
      images.value = imageResources
      sniffProgress.value = 100
      return true
    } catch (err) {
      error.value = `Error: ${err}`
      console.error('Failed to sniff resources:', err)
      return false
    } finally {
      isSniffing.value = false
    }
  }

  function setVideoUrl(url: string) {
    videoUrl.value = url
  }

  function setSniffProgress(progress: number) {
    sniffProgress.value = progress
  }

  function clearResources() {
    videos.value = []
    images.value = []
    videoUrl.value = ''
    error.value = ''
  }

  return {
    videoUrl,
    isSniffing,
    sniffProgress,
    videos,
    images,
    error,
    sniffResources,
    setVideoUrl,
    setSniffProgress,
    clearResources,
  }
})
