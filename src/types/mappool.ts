export interface Mappool {
  id: number
  name: string
  description?: string
  created_at: string
  updated_at: string
}

export interface BeatmapEntry {
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
}

export interface BeatmapData {
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
