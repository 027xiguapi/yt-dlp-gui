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
  }
  presets: Record<string, string>
}

export const useConfigStore = defineStore('config', () => {
  const config = ref<Config | null>(null)
  const downloadPath = ref('')
  const cookiePath = ref('')
  const selectedPreset = ref('best')
  const globalArgs = ref('')
  const updateYtdlp = ref(true)
  const isLoading = ref(false)

  async function loadConfig() {
    isLoading.value = true
    try {
      const defaultConfig = await invoke<Config>('get_default_config')
      config.value = defaultConfig
      downloadPath.value = defaultConfig.general.path
      cookiePath.value = defaultConfig.general.cookie_path
      selectedPreset.value = Object.keys(defaultConfig.presets)[defaultConfig.general.current_preset] || 'best'
      globalArgs.value = defaultConfig.general.global_args
      updateYtdlp.value = defaultConfig.general.update_ytdlp
    } catch (error) {
      console.error('Failed to load config:', error)
    } finally {
      isLoading.value = false
    }
  }

  async function saveConfig() {
    if (!config.value) return

    const updatedConfig: Config = {
      ...config.value,
      general: {
        ...config.value.general,
        path: downloadPath.value,
        cookie_path: cookiePath.value,
        global_args: globalArgs.value,
        update_ytdlp: updateYtdlp.value,
      },
    }

    // try {
    //   await invoke('save_config', {
    //     path: 'config.toml',
    //     config: updatedConfig,
    //   })
    //   config.value = updatedConfig
    //   return true
    // } catch (error) {
    //   console.error('Failed to save config:', error)
    //   throw error
    // }
  }

  function setDownloadPath(path: string) {
    downloadPath.value = path
  }

  function setCookiePath(path: string) {
    cookiePath.value = path
  }

  function clearCookiePath() {
    cookiePath.value = ''
  }

  function setSelectedPreset(preset: string) {
    selectedPreset.value = preset
  }

  function setGlobalArgs(args: string) {
    globalArgs.value = args
  }

  function setUpdateYtdlp(value: boolean) {
    updateYtdlp.value = value
  }

  return {
    config,
    downloadPath,
    cookiePath,
    selectedPreset,
    globalArgs,
    updateYtdlp,
    isLoading,
    loadConfig,
    saveConfig,
    setDownloadPath,
    setCookiePath,
    clearCookiePath,
    setSelectedPreset,
    setGlobalArgs,
    setUpdateYtdlp,
  }
})
