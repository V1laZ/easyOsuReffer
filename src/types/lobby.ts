export interface Player {
  username: string
  team: 'red' | 'blue' | null
  isReady: boolean
  isPlaying: boolean
  isHost: boolean
}

export interface PlayerMoveEvent {
  playerName: string
  to: number
}

export interface PlayerTeamChangeEvent {
  playerName: string
  team: 'red' | 'blue'
}

export interface PlayerSlot {
  id: number
  player: Player | null
}

export interface CurrentMap {
  beatmapId: number
  title: string
  difficulty: string
  artist: string
}

export interface LobbySettings {
  roomName: string
  teamMode: 'HeadToHead' | 'TagCoop' | 'TeamVs' | 'TagTeamVs'
  winCondition: 'Score' | 'Accuracy' | 'Combo' | 'ScoreV2'
  size: number
  password?: string
}

export interface LobbyState {
  settings: LobbySettings | null
  currentMap: CurrentMap | null
  slots: PlayerSlot[]
  matchStatus: 'idle' | 'ready' | 'starting' | 'active'
  host: string | null
  freemod: boolean
  selectedMods: string[]
  currentMappoolId: number | null
}

export interface CreateLobbySettings {
  name: string
  teamMode: '0' | '1' | '2' | '3'
  scoreMode: '0' | '1' | '2' | '3'
}
