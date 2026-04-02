import { createI18n } from 'vue-i18n'
import zh from './locales/zh.json'
import en from './locales/en.json'

export type MessageSchema = typeof zh

const i18n = createI18n<{ message: MessageSchema }, 'zh' | 'en'>({
  legacy: false,
  locale: localStorage.getItem('locale') || 'zh',
  fallbackLocale: 'zh',
  messages: {
    zh,
    en
  }
})

export default i18n
