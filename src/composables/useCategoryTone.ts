export type BadgeTone = 'neutral' | 'accent' | 'success' | 'warning' | 'danger' | 'info'

const CATEGORY_TONES: Record<string, BadgeTone> = {
  NM: 'neutral',
  HD: 'warning',
  HR: 'danger',
  DT: 'info',
  FL: 'neutral',
  FM: 'success',
  TB: 'accent',
}

export function useCategoryTone() {
  const categoryTone = (category?: string): BadgeTone => {
    const prefix = (category ?? '').slice(0, 2).toUpperCase()
    return CATEGORY_TONES[prefix] ?? 'neutral'
  }

  return { categoryTone }
}
