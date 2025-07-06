type RoomType = 'Channel' | 'PrivateMessage' | 'MultiplayerLobby'

type ConnectionStatus = {
  type: string
  message: string
}

type Room = {
  id: string
  displayName: string
  roomType: RoomType
  messages: IrcMessage[]
  unreadCount: number
  isActive: boolean
}

type IrcMessage = {
  roomId: string
  username: string
  message: string
  timestamp: number
  isPrivate: boolean
}

type Player = {
  username: string
  team: 'red' | 'blue' | null
  isReady: boolean
  isPlaying: boolean
  isHost: boolean
}

type PlayerMoveEvent = {
  playerName: string
  to: number
}

type PlayerTeamChangeEvent = {
  playerName: string
  team: 'red' | 'blue'
}

type PlayerSlot = {
  id: number
  player: Player | null
}

type CurrentMap = {
  beatmapId: number
  title: string
  difficulty: string
  artist: string
}

type LobbySettings = {
  roomName: string
  teamMode: 'HeadToHead' | 'TagCoop' | 'TeamVs' | 'TagTeamVs'
  winCondition: 'Score' | 'Accuracy' | 'Combo' | 'ScoreV2'
  size: number
  password?: string
}

type LobbyState = {
  channel: string
  settings: LobbySettings | null
  currentMap: CurrentMap | null
  slots: PlayerSlot[]
  matchStatus: 'idle' | 'ready' | 'starting' | 'active'
  host: string | null
  freemod: boolean
  selectedMods: string[]
  currentMappoolId: number | null
}

type UserJoinEvent = {
  username: string
  channel: string
}

type CreateLobbySettings = {
  name: string
  teamMode: '0' | '1' | '2' | '3'
  scoreMode: '0' | '1' | '2' | '3'
}

type OAuthToken = {
  id: number
  access_token: string
  refresh_token: string
  expires_in: number
  expires_at: string
  created_at: string
  updated_at: string
}

type OauthTokenCallback = Pick<OAuthToken, 'access_token' | 'refresh_token' | 'expires_in'>

type UserCredentials = {
  id: number
  username: string
  password: string
  created_at: string
  updated_at: string
}

type Mappool = {
  id: number
  name: string
  description?: string
  created_at: string
  updated_at: string
}

type BeatmapEntry = {
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

type BeatmapData = {
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

type NewMappoolForm = Omit<Mappool, 'id' | 'created_at' | 'updated_at'>