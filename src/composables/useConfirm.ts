import { reactive } from 'vue'

type Tone = 'primary' | 'danger'

export interface ConfirmOptions {
  title: string
  message?: string
  confirmText?: string
  cancelText?: string
  tone?: Tone
}

interface ConfirmState extends Required<ConfirmOptions> {
  open: boolean
  resolve: ((value: boolean) => void) | null
}

export const confirmState = reactive<ConfirmState>({
  open: false,
  title: '',
  message: '',
  confirmText: 'Confirm',
  cancelText: 'Cancel',
  tone: 'primary',
  resolve: null,
})

export function confirm(options: ConfirmOptions): Promise<boolean> {
  return new Promise((resolve) => {
    if (confirmState.resolve) confirmState.resolve(false)

    confirmState.title = options.title
    confirmState.message = options.message ?? ''
    confirmState.confirmText = options.confirmText ?? 'Confirm'
    confirmState.cancelText = options.cancelText ?? 'Cancel'
    confirmState.tone = options.tone ?? 'primary'
    confirmState.resolve = resolve
    confirmState.open = true
  })
}

export function settle(value: boolean) {
  confirmState.resolve?.(value)
  confirmState.resolve = null
  confirmState.open = false
}
