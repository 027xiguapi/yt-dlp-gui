import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { invoke } from '@tauri-apps/api/core'

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
}

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
    }
    tasks.value.set(taskId, task)
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
    }
  }

  function removeTask(id: string) {
    const task = tasks.value.get(id)
    if (task && (task.status === 'Queued' || task.status === 'Finished' || task.status === 'ERROR')) {
      tasks.value.delete(id)
    }
  }

  function clearCompleted() {
    for (const [id, task] of tasks.value.entries()) {
      if (task.status === 'Finished' || task.status === 'ERROR') {
        tasks.value.delete(id)
      }
    }
  }

  function clearAll() {
    tasks.value.clear()
  }

  function getTask(id: string) {
    return tasks.value.get(id)
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
  }
})
