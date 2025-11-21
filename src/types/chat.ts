export type RoomType = 'Channel' | 'PrivateMessage' | 'MultiplayerLobby'

export interface ConnectionStatus {
  type: string
  message: string
}

export interface Room {
  id: string
  displayName: string
  roomType: RoomType
  messages: IrcMessage[]
  unreadCount: number
  isActive: boolean
}

export interface IrcMessage {
  roomId: string
  username: string
  message: string
  timestamp: number
  isPrivate: boolean
}

export interface UserJoinEvent {
  username: string
  channel: string
}
