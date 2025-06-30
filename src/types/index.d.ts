type IrcMessage = {
  id: number
  channel: string
  username: string
  message: string
  timestamp: number
}

type Player = {
  username: string
  team: 'red' | 'blue'
  isReady: boolean
  isPlaying: boolean
  isHost: boolean
}

type PlayerSlot = {
  id: number
  isLocked: boolean
  player: Player | null
}