export type OAuthToken = {
  id: number
  access_token: string
  refresh_token: string
  expires_in: number
  expires_at: string
  created_at: string
  updated_at: string
}

export type OauthTokenCallback = Pick<OAuthToken, 'access_token' | 'refresh_token' | 'expires_in'>

export type UserCredentials = {
  id: number
  username: string
  password: string
  created_at: string
  updated_at: string
}
