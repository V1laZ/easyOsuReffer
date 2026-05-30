import { fetch } from '@tauri-apps/plugin-http'
import type { ExtractedRound } from '@/types'

const EXTRACT_URL = 'https://osureffer.vilaz.dev/extract-mappool'

export async function extractMappoolFromSheet(url: string): Promise<ExtractedRound[]> {
  const res = await fetch(EXTRACT_URL, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ url }),
  })

  const data = await res.json().catch(() => null) as { rounds?: ExtractedRound[], error?: string } | null

  if (!res.ok) {
    throw new Error(data?.error || `Failed to read the sheet (${res.status})`)
  }

  return data?.rounds ?? []
}
