import { reactive } from 'vue'

const createEmptySlots = (): PlayerSlot[] => {
  return Array.from({ length: 16 }, (_, i) => ({
    id: i + 1,
    isLocked: true,
    player: null
  }))
}

const lobbyState = reactive<LobbyState>({
  isInLobby: false,
  channel: null,
  settings: null,
  currentMap: null,
  slots: createEmptySlots(),
  matchStatus: 'idle',
  host: null,
})

export const lobbyActions = {
  joinLobby(channel: string) {
    lobbyState.isInLobby = true
    lobbyState.channel = channel
    lobbyState.slots = createEmptySlots()
    lobbyState.matchStatus = 'idle'
    console.log(`Joined lobby: ${channel}`)
  },

  leaveLobby() {
    lobbyState.isInLobby = false
    lobbyState.channel = null
    lobbyState.settings = null
    lobbyState.currentMap = null
    lobbyState.slots = createEmptySlots()
    lobbyState.matchStatus = 'idle'
    lobbyState.host = null
    console.log('Left lobby')
  },

  updateSettings(settings: Partial<LobbySettings>) {
    if (!lobbyState.settings) {
      lobbyState.settings = {
        roomName: '',
        teamMode: 'HeadToHead',
        winCondition: 'Score',
        size: 16,
        ...settings
      }
    } else {
      Object.assign(lobbyState.settings, settings)
    }
    console.log('Updated lobby settings:', lobbyState.settings)
  },

  updateCurrentMap(map: CurrentMap) {
    lobbyState.currentMap = map
    console.log('Updated current map:', map)
  },

  updateSlot(slotId: number, updates: Partial<PlayerSlot>) {
    const slot = lobbyState.slots.find(s => s.id === slotId)
    if (slot) {
      Object.assign(slot, updates)
      console.log(`Updated slot ${slotId}:`, slot)
    }
  },

  addPlayer(slotId: number, player: Player) {
    const slot = lobbyState.slots.find(s => s.id === slotId)
    if (slot) {
      slot.player = player
      slot.isLocked = false
      console.log(`Added player ${player.username} to slot ${slotId}`)
    }
  },

  removePlayer(slotId: number) {
    const slot = lobbyState.slots.find(s => s.id === slotId)
    if (slot) {
      slot.player = null
      console.log(`Removed player from slot ${slotId}`)
    }
  },

  setSlotLocked(slotId: number, locked: boolean) {
    const slot = lobbyState.slots.find(s => s.id === slotId)
    if (slot) {
      slot.isLocked = locked
      if (locked) {
        slot.player = null
      }
      console.log(`Set slot ${slotId} locked: ${locked}`)
    }
  },

  updateMatchStatus(status: LobbyState['matchStatus']) {
    lobbyState.matchStatus = status
    console.log(`Match status updated to: ${status}`)
  },

  updateHost(hostUsername: string) {
    lobbyState.host = hostUsername
    
    lobbyState.slots.forEach(slot => {
      if (slot.player) {
        slot.player.isHost = slot.player.username === hostUsername
      }
    })
    console.log(`Host updated to: ${hostUsername}`)
  },
}

export { lobbyState }
