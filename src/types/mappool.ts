export type Mappool = {
  id: number
  name: string
  description?: string
  created_at: string
  updated_at: string
}

export type BeatmapEntry = {
  id: number
  mappool_id: number
  beatmap_id: number
  artist: string
  title: string
  difficulty: string
  mapper: string
  mod_combination?: string
  category?: string
  created_at: string
  position: number
}

export type BeatmapData = {
  id: number
  beatmapset_id: number
  artist: string
  title: string
  difficulty: string
  mapper: string
  mode: number
  total_length: number
  bpm: number
  difficulty_rating: number
}

export type NewMappoolForm = Omit<Mappool, 'id' | 'created_at' | 'updated_at'>

export type ExtractedEntry = {
  category: string
  mods: string
  beatmapId: number
  artist: string
  title: string
  difficulty: string
  mapper: string
  needsReview: boolean
}

export type ExtractedRound = {
  name: string
  entries: ExtractedEntry[]
}

export type ExtractedSheet = {
  sheetTitle: string | null
  rounds: ExtractedRound[]
}
