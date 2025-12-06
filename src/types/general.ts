export interface PendingRequest {
  promise: Promise<string>
  resolve: (url: string) => void
  reject: (error: unknown) => void
}
