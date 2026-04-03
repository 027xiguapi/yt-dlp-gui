import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import * as db from '../services/database'

export interface DownloadTask {
  id: string
  url: string
  preset: string
  path: string
  status: string
  progress: number
  speed: string
  eta: string
  size: string
  title: string
  error: any
}

const STORAGE_KEY = 'yt-dlp-downloads'

export const useDownloadStore = defineStore('download', () => {
  const tasks = ref<Map<string, DownloadTask>>(new Map())

  const taskList = computed(() => Array.from(tasks.value.values()))

  const isDownloading = computed(() =>
    taskList.value.some(t => t.status === 'Downloading' || t.status === 'Processing')
  )

  const stats = computed(() => ({
    total: taskList.value.length,
    queued: taskList.value.filter(t => t.status === 'Queued').length,
    downloading: taskList.value.filter(t => t.status === 'Downloading' || t.status === 'Processing').length,
    finished: taskList.value.filter(t => t.status === 'Finished').length,
    error: taskList.value.filter(t => t.status === 'ERROR').length,
  }))

  function loadFromStorage() {
    try {
      const stored = localStorage.getItem(STORAGE_KEY)
      if (stored) {
        const taskArray = JSON.parse(stored)
        tasks.value.clear()
        taskArray.forEach((task: DownloadTask) => {
          tasks.value.set(task.id, task)
        })
      }
    } catch (error) {
      console.error('Failed to load from localStorage:', error)
    }
  }

  async function loadFromDatabase() {
    try {
      const history = await db.getDownloadHistory(1000)
      tasks.value.clear()
      history.forEach((task: any) => {
        tasks.value.set(task.id, {
          id: task.id,
          url: task.url,
          preset: task.preset,
          path: task.path,
          status: task.status,
          progress: task.progress || 0,
          speed: task.speed || '-',
          eta: task.eta || '-',
          size: task.size || '-',
          title: task.title || task.url,
          error: task.error || '',
        })
      })
    } catch (error) {
      console.error('Failed to load from database:', error)
    }
  }

  function saveToStorage() {
    try {
      const taskArray = Array.from(tasks.value.values())
      localStorage.setItem(STORAGE_KEY, JSON.stringify(taskArray))
    } catch (error) {
      console.error('Failed to save to localStorage:', error)
    }
  }

  async function addTask(url: string, preset: string, path: string) {
    const taskId = await invoke<string>('generate_task_id')
    const task: DownloadTask = {
      id: taskId,
      url,
      preset,
      path,
      status: 'Queued',
      progress: 0,
      speed: '-',
      eta: '-',
      size: '-',
      title: url,
      error: '',
    }
    tasks.value.set(taskId, task)
    saveToStorage()

    // 保存到数据库
    try {
      await db.saveDownloadHistory(task)
    } catch (error) {
      console.error('Failed to save task to database:', error)
    }

    return taskId
  }

  async function addMultipleTasks(urls: string[], preset: string, path: string) {
    const taskIds: string[] = []
    for (const url of urls) {
      const taskId = await addTask(url, preset, path)
      taskIds.push(taskId)
    }
    return taskIds
  }

  function updateTask(id: string, updates: Partial<DownloadTask>) {
    const task = tasks.value.get(id)
    if (task) {
      Object.assign(task, updates)
      saveToStorage()

      // 更新数据库
      try {
        db.updateDownloadHistory(id, updates)
      } catch (error) {
        console.error('Failed to update task in database:', error)
      }
    }
  }

  function removeTask(id: string) {
    const task = tasks.value.get(id)
    if (task && (task.status === 'Queued' || task.status === 'Finished' || task.status === 'ERROR')) {
      tasks.value.delete(id)
      saveToStorage()

      // 从数据库删除
      try {
        db.deleteDownloadHistory(id)
      } catch (error) {
        console.error('Failed to delete task from database:', error)
      }
    }
  }

  function clearCompleted() {
    for (const [id, task] of tasks.value.entries()) {
      if (task.status === 'Finished' || task.status === 'ERROR') {
        tasks.value.delete(id)
      }
    }
    saveToStorage()
  }

  function clearAll() {
    tasks.value.clear()
    saveToStorage()
  }

  function getTask(id: string) {
    return tasks.value.get(id)
  }

  function initializeFromStorage() {
    loadFromStorage()
  }

  async function initializeFromDatabase() {
    await loadFromDatabase()
  }

  return {
    tasks,
    taskList,
    isDownloading,
    stats,
    addTask,
    addMultipleTasks,
    updateTask,
    removeTask,
    clearCompleted,
    clearAll,
    getTask,
    initializeFromStorage,
    initializeFromDatabase,
  }
})
