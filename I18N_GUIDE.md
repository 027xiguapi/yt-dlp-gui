# 国际化 (i18n) 使用指南

本项目使用 `vue-i18n` 实现国际化支持，支持中文 (zh) 和英文 (en) 两种语言。

## 项目结构

```
src/
├── i18n/
│   ├── index.ts                 # i18n 配置文件
│   └── locales/
│       ├── zh.json              # 中文语言文件
│       └── en.json              # 英文语言文件
├── composables/
│   └── useI18n.ts              # i18n composable (可选)
├── main.ts                      # 应用入口（已集成 i18n）
└── App.vue                      # 主应用组件
```

## 使用方式

### 1. 在 Vue 组件中使用

```vue
<script setup lang="ts">
import { useI18n } from 'vue-i18n'

const { t, locale } = useI18n()

// 切换语言
const changeLocale = (newLocale: 'zh' | 'en') => {
  locale.value = newLocale
  localStorage.setItem('locale', newLocale)
}
</script>

<template>
  <!-- 使用翻译 -->
  <h1>{{ t('main.title') }}</h1>
  <p>{{ t('main.subtitle') }}</p>

  <!-- 获取当前语言 -->
  <span>Current language: {{ locale }}</span>
</template>
```

### 2. 添加新的翻译

编辑 `src/i18n/locales/zh.json` 和 `src/i18n/locales/en.json`：

```json
{
  "section": {
    "key": "翻译文本"
  }
}
```

### 3. 在脚本中使用

```typescript
import { useI18n } from 'vue-i18n'

const { t } = useI18n()

const message = t('main.title')  // 获取翻译
```

## 语言切换

用户可以在应用侧边栏底部的语言切换按钮中选择语言：
- 中文 (简体中文)
- English (English)

选择的语言会自动保存到 `localStorage`，下次打开应用时会使用相同的语言。

## 当前支持的翻译

### 通用 (common)
- yes, no, ok, cancel, save, reset, delete, close, edit, add, export, import, search, loading, success, error, warning, info

### 菜单 (menu)
- download, channelExtraction, sniffer, settings

### 主页 (main)
- title, subtitle, pasteUrl, addQueue, downloadQueue, startSelected, deleteSelected, clearCompleted, tableTitle, tableStatus, tableProgress, tableInfo, tableAction, openFolder, copyUrl, startDownload, deleteTask, deleteConfirm, copySuccess, copyFailed, openFolderSuccess, openFolderFailed, noUrlError, noValidUrl, addSuccess

### 频道提取 (channelExtraction)
- title, channelUrl, placeholder, extract, extracting, results, channel, videosCount, extractedUrls, addToQueue, clearResults, noResults

### 资源嗅探 (sniffer)
- title, sniffResources, videoUrl, startSniff, sniffing, capturedResources, videos, images, statistics, totalVideos, totalImages, totalResources, videoSize, noVideos, noImages, noResults, copy, view

### 设置 (settings)
- title, subtitle, downloadConfig, downloadPath, browse, ytdlpPath, placeholder, selectFile, cookieSource, cookiePath, cookieHint, cookiePathPlaceholder, clear, customCookie, fromBrowser, envTest, checkVersion, checking, versionHint

### 状态 (status)
- finished, processing, downloading, converting, queued, error

## 技术栈

- **vue-i18n**: 11.3.0
- **Vue**: 3.5.13
- **TypeScript**: 5.6.2

## 注意事项

1. 语言设置保存在浏览器 localStorage 中，键为 `locale`
2. 默认语言为中文 (`zh`)
3. 当翻译缺失时，会自动回退到中文版本
4. 添加新语言时需要同时更新两个语言文件的结构
