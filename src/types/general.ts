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
