import { createRouter, createWebHistory } from 'vue-router'

const routes = [
  {
    path: '/',
    component: () => import('../views/Main.vue'),
    meta: { title: 'Downloader' }
  },
  {
    path: '/sniffer',
    component: () => import('../views/Sniffer.vue'),
    meta: { title: 'Resource Sniffer' }
  },
  {
    path: '/settings',
    component: () => import('../views/Settings.vue'),
    meta: { title: 'Settings' }
  }
]

const router = createRouter({
  history: createWebHistory(),
  routes
})

export default router