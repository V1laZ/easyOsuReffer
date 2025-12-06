import { dialogRegistry } from '@/main'
import { MaybeRef, onMounted, onUnmounted, unref } from 'vue'

export function useDialog(dialog?: MaybeRef<HTMLDialogElement | null> | string) {
  if (typeof dialog === 'string') {
    const dialogId = dialog
    const dialogEl = dialogRegistry.get(dialogId)

    return dialogEl
  }

  onMounted(() => {
    const dialogElement = unref(dialog)
    if (!dialogElement) return

    const dialogId = dialogElement.id
    if (!dialogId) {
      console.error('Dialog element must have an ID to be registered.')
      return
    }

    dialogRegistry.set(dialogId, dialogElement)
  })

  onUnmounted(() => {
    const dialogElement = unref(dialog)
    if (!dialogElement) return

    const dialogId = dialogElement.id
    if (dialogId) {
      dialogRegistry.delete(dialogId)
    }
  })
}
