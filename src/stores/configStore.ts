import { defineStore } from 'pinia'
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'

export interface Config {
  general: {
    current_preset: number
    path: string
    global_args: string
    update_ytdlp: boolean
    cookie_path: string
    ytdlp_path: string
  }
  presets: Record<string, string>
}

const STORAGE_KEY = 'yt-dlp-config'

export const useConfigStore = defineStore('config', () => {
  const config = ref<Config | null>(null)
  const downloadPath = ref('')
  const cookiePath = ref('')
  const cookieBrowser = ref('chrome')
  const ytdlpPath = ref('')
  const selectedPreset = ref('best')
  const globalArgs = ref('')
  const isLoading = ref(false)

  function loadFromStorage() {
    try {
      const stored = localStorage.getItem(STORAGE_KEY)
      if (stored) {
        const data = JSON.parse(stored)
        downloadPath.value = data.downloadPath || ''
        cookiePath.value = data.cookiePath || ''
        cookieBrowser.value = data.cookieBrowser || 'chrome'
        ytdlpPath.value = data.ytdlpPath || ''
        selectedPreset.value = data.selectedPreset || 'best'
        globalArgs.value = data.globalArgs || ''
      }
    } catch (error) {
      console.error('Failed to load from localStorage:', error)
    }
  }

  function saveToStorage() {
    try {
      const data = {
        downloadPath: downloadPath.value,
        cookiePath: cookiePath.value,
        cookieBrowser: cookieBrowser.value,
        ytdlpPath: ytdlpPath.value,
        selectedPreset: selectedPreset.value,
        globalArgs: globalArgs.value,
      }
      localStorage.setItem(STORAGE_KEY, JSON.stringify(data))
    } catch (error) {
      console.error('Failed to save to localStorage:', error)
    }
  }

  async function loadConfig() {
    isLoading.value = true
    try {
      loadFromStorage()
      const defaultConfig = await invoke<Config>('get_default_config')
      config.value = defaultConfig
      if (!downloadPath.value) {
        downloadPath.value = defaultConfig.general.path
      }
      if (!cookiePath.value) {
        cookiePath.value = defaultConfig.general.cookie_path
      }
      if (!ytdlpPath.value) {
        ytdlpPath.value = defaultConfig.general.ytdlp_path || './win/yt-dlp.exe'
      }
      if (!selectedPreset.value) {
        selectedPreset.value = Object.keys(defaultConfig.presets)[defaultConfig.general.current_preset] || 'best'
      }
      if (!globalArgs.value) {
        globalArgs.value = defaultConfig.general.global_args
      }
      saveToStorage()
    } catch (error) {
      console.error('Failed to load config:', error)
    } finally {
      isLoading.value = false
    }
  }

  async function saveConfig() {
    saveToStorage()
  }

  function setDownloadPath(path: string) {
    downloadPath.value = path
    saveToStorage()
  }

  function setCookiePath(path: string) {
    cookiePath.value = path
    saveToStorage()
  }

  function setCookieBrowser(browser: string) {
    cookieBrowser.value = browser
    saveToStorage()
  }

  function clearCookiePath() {
    cookiePath.value = ''
    saveToStorage()
  }

  function setYtdlpPath(path: string) {
    ytdlpPath.value = path
    saveToStorage()
  }

  function setSelectedPreset(preset: string) {
    selectedPreset.value = preset
    saveToStorage()
  }

  function setGlobalArgs(args: string) {
    globalArgs.value = args
    saveToStorage()
  }

  return {
    config,
    downloadPath,
    cookiePath,
    cookieBrowser,
    ytdlpPath,
    selectedPreset,
    globalArgs,
    isLoading,
    loadConfig,
    saveConfig,
    setDownloadPath,
    setCookiePath,
    setCookieBrowser,
    clearCookiePath,
    setYtdlpPath,
    setSelectedPreset,
    setGlobalArgs,
  }
})
