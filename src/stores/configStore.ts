import { defineStore } from 'pinia'
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import * as db from '../services/database'

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

const VERSION_CHECKS = [
  { key: 'ytdlp', cmd: 'yt-dlp', args: ['--version'] },
  { key: 'deno', cmd: 'deno', args: ['--version'] },
  { key: 'ffmpeg', cmd: 'ffmpeg', args: ['-version'] },
  { key: 'ffprobe', cmd: 'ffprobe', args: ['-version'] }
]

export const useConfigStore = defineStore('config', () => {
  const config = ref<Config | null>(null)
  const downloadPath = ref('')
  const cookiePath = ref('')
  const cookieBrowser = ref('chrome')
  const ytdlpPath = ref('')
  const selectedPreset = ref('best')
  const globalArgs = ref('')
  const isLoading = ref(false)
  const versions = ref({
    ytdlp: '',
    deno: '',
    ffmpeg: '',
    ffprobe: ''
  })

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

  async function loadFromDatabase() {
    try {
      const allConfig = await db.getAllConfig()
      if (allConfig['downloadPath']) downloadPath.value = allConfig['downloadPath']
      if (allConfig['cookiePath']) cookiePath.value = allConfig['cookiePath']
      if (allConfig['cookieBrowser']) cookieBrowser.value = allConfig['cookieBrowser']
      if (allConfig['ytdlpPath']) ytdlpPath.value = allConfig['ytdlpPath']
      if (allConfig['selectedPreset']) selectedPreset.value = allConfig['selectedPreset']
      if (allConfig['globalArgs']) globalArgs.value = allConfig['globalArgs']
    } catch (error) {
      console.error('Failed to load from database:', error)
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

  async function saveToDatabase() {
    try {
      await db.saveConfig('downloadPath', downloadPath.value)
      await db.saveConfig('cookiePath', cookiePath.value)
      await db.saveConfig('cookieBrowser', cookieBrowser.value)
      await db.saveConfig('ytdlpPath', ytdlpPath.value)
      await db.saveConfig('selectedPreset', selectedPreset.value)
      await db.saveConfig('globalArgs', globalArgs.value)
    } catch (error) {
      console.error('Failed to save to database:', error)
    }
  }

  async function loadConfig() {
    isLoading.value = true
    try {
      loadFromStorage()
      await loadFromDatabase()
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
      await saveToDatabase()
    } catch (error) {
      console.error('Failed to load config:', error)
    } finally {
      isLoading.value = false
    }
  }

  async function saveConfig() {
    saveToStorage()
    await saveToDatabase()
  }

  function setDownloadPath(path: string) {
    downloadPath.value = path
    saveToStorage()
    saveToDatabase()
  }

  function setCookiePath(path: string) {
    cookiePath.value = path
    saveToStorage()
    saveToDatabase()
  }

  function setCookieBrowser(browser: string) {
    cookieBrowser.value = browser
    saveToStorage()
    saveToDatabase()
  }

  function clearCookiePath() {
    cookiePath.value = ''
    saveToStorage()
    saveToDatabase()
  }

  function setYtdlpPath(path: string) {
    ytdlpPath.value = path
    saveToStorage()
    saveToDatabase()
  }

  function setSelectedPreset(preset: string) {
    selectedPreset.value = preset
    saveToStorage()
    saveToDatabase()
  }

  function setGlobalArgs(args: string) {
    globalArgs.value = args
    saveToStorage()
    saveToDatabase()
  }

  async function checkVersions() {
    for (const check of VERSION_CHECKS) {
      try {
        const output = await invoke<string>('check_version', {
          cmd: check.cmd,
          args: check.args,
          ytdlpPath: ytdlpPath.value || config.value?.general.ytdlp_path
        })
        versions.value[check.key as keyof typeof versions.value] = output.split('\n')[0] || '未找到'
      } catch (error) {
        versions.value[check.key as keyof typeof versions.value] = '未安装或不在 PATH 中'
      }
    }
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
    versions,
    loadConfig,
    saveConfig,
    setDownloadPath,
    setCookiePath,
    setCookieBrowser,
    clearCookiePath,
    setYtdlpPath,
    setSelectedPreset,
    setGlobalArgs,
    checkVersions,
  }
})
