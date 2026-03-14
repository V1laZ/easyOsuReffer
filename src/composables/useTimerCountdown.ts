import { computed, watch, type Ref } from 'vue'
import { useCountdown } from '@vueuse/core'
import type { LobbyState } from '@/types'

export function useTimerCountdown(lobbyState: Readonly<Ref<LobbyState>>) {
  const { remaining, start, stop, isActive } = useCountdown(0)

  watch(
    () => [lobbyState.value.timerStartTime, lobbyState.value.timerDuration] as const,
    ([startTime, duration]) => {
      if (startTime != null && duration != null) {
        const elapsed = Math.floor(Date.now() / 1000 - startTime)
        const remaining = Math.max(0, duration - elapsed)
        start(remaining)
      }
      else {
        stop()
      }
    },
    { immediate: true },
  )

  const formattedTime = computed(() => {
    if (!isActive.value) return null
    const m = Math.floor(remaining.value / 60)
    const s = remaining.value % 60
    return `${m}:${s.toString().padStart(2, '0')}`
  })

  return { formattedTime, isActive }
}
