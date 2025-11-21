import type { LobbyState } from './lobby'

export type RoomType = 'Channel' | 'PrivateMessage' | 'MultiplayerLobby'

export type ConnectionStatus = {
  type: string
  message: string
}

export type Room = {
  id: string
  displayName: string
  roomType: RoomType
  messages: IrcMessage[]
  unreadCount: number
  isActive: boolean
  lobbyState: LobbyState | null
}

export type RoomItem = Omit<Room, 'messages'>

export type IrcMessage = {
  roomId: string
  username: string
  message: string
  timestamp: number
  isPrivate: boolean
}

export type UserJoinEvent = {
  username: string
  channel: string
}
