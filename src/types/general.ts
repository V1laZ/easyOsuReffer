export interface PendingRequest {
  promise: Promise<string>
  resolve: (url: string) => void
  reject: (error: unknown) => void
}

export type UpdateInfo = {
  current_version: string
  latest_version: string
  date: string
  body: string
}

export type UserData = {
  id: number
  username: string
  avatar_url: string
  country: string
  pp: number
  rank: number | null
  country_rank: number | null
  accuracy: number
}
