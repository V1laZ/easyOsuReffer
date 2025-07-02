type IrcMessage = {
  id: number
  channel: string
  username: string
  message: string
  timestamp: number
}

type Player = {
  username: string
  team: 'red' | 'blue' | null
  isReady: boolean
  isPlaying: boolean
  isHost: boolean
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
}

type CreateLobbySettings = {
  name: string
  teamMode: '0' | '1' | '2' | '3'
  scoreMode: '0' | '1' | '2' | '3'
}