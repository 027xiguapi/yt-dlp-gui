import { createRouter, createWebHistory } from 'vue-router'

const routes = [
  {
    path: '/',
    component: () => import('../views/Main.vue'),
    meta: { title: 'Downloader' }
  },
  {
    path: '/channel-extraction',
    component: () => import('../views/ChannelExtraction.vue'),
    meta: { title: 'Channel Extraction' }
  },
  {
    path: '/rss',
    component: () => import('../views/RssSubscription.vue'),
    meta: { title: 'RSS Subscription' }
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
  },
  {
    path: '/platform-test',
    component: () => import('../views/PlatformTest.vue'),
    meta: { title: 'Platform Test' }
  }
]

const router = createRouter({
  history: createWebHistory(),
  routes
})

export default router