import { useI18n as useVueI18n } from 'vue-i18n'
import type { MessageSchema } from '../i18n'

export const useI18n = () => {
  return useVueI18n<{ message: MessageSchema }, 'zh' | 'en'>()
}
