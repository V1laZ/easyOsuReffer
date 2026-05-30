import { watch } from 'vue'
import { platform } from '@tauri-apps/plugin-os'
import { onBackButtonPress } from '@tauri-apps/api/app'
import type { PluginListener } from '@tauri-apps/api/core'
import { closeTopModalLayer, modalLayerCount } from './useModalLayer'

export function useAndroidBackButton() {
  if (platform() !== 'android') return

  let listener: PluginListener | null = null

  watch(modalLayerCount, async (count) => {
    if (count > 0) {
      if (!listener) {
        listener = await onBackButtonPress(({ canGoBack }) => {
          if (!closeTopModalLayer() && canGoBack) window.history.back()
        })
      }
    }
    else if (listener) {
      listener.unregister()
      listener = null
    }
  }, { immediate: true })
}
