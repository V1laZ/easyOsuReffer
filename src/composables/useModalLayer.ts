import { type Ref, computed, onBeforeUnmount, ref, watch } from 'vue'

interface Layer {
  id: number
  close: () => void
}

const stack = ref<Layer[]>([])
let nextId = 0

const BASE_Z = 50
const STEP_Z = 10

let escListenerAttached = false

function ensureEscListener() {
  if (escListenerAttached) return
  if (typeof document === 'undefined') return
  escListenerAttached = true
  document.addEventListener('keydown', (e) => {
    if (e.key !== 'Escape') return
    const top = stack.value[stack.value.length - 1]
    if (!top) return
    e.stopPropagation()
    top.close()
  })
}

export function useModalLayer(
  isOpen: Ref<boolean> | (() => boolean),
  close: () => void,
) {
  const id = ++nextId
  const getIsOpen = typeof isOpen === 'function' ? isOpen : () => isOpen.value

  ensureEscListener()

  watch(getIsOpen, (open) => {
    const idx = stack.value.findIndex(l => l.id === id)
    if (open && idx === -1) {
      stack.value = [...stack.value, { id, close }]
    }
    else if (!open && idx !== -1) {
      stack.value = stack.value.filter(l => l.id !== id)
    }
  }, { immediate: true })

  onBeforeUnmount(() => {
    stack.value = stack.value.filter(l => l.id !== id)
  })

  const zIndex = computed(() => {
    const idx = stack.value.findIndex(l => l.id === id)
    if (idx === -1) return BASE_Z
    return BASE_Z + (idx + 1) * STEP_Z
  })

  const isTopmost = computed(() => {
    const top = stack.value[stack.value.length - 1]
    return top?.id === id
  })

  return { zIndex, isTopmost }
}

export const modalLayerCount = computed(() => stack.value.length)

export function closeTopModalLayer(): boolean {
  const top = stack.value[stack.value.length - 1]
  if (!top) return false
  top.close()
  return true
}
