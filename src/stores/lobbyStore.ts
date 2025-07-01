import { reactive } from 'vue'

const createEmptySlots = (): PlayerSlot[] => {
  return Array.from({ length: 16 }, (_, i) => ({
    id: i + 1,
    player: null
  }))
}

const createEmptyLobbyState = (): LobbyState => ({
  channel: null,
  settings: null,
  currentMap: null,
  slots: createEmptySlots(),
  matchStatus: 'idle',
  host: null,
})

const lobbyStates = reactive<Record<string, LobbyState>>({})
const activeLobbyChannel = reactive<{ value: string | null }>({ value: null })

export const lobbyActions = {
  joinLobby(channel: string) {
    if (!lobbyStates[channel]) {
      lobbyStates[channel] = {
        ...createEmptyLobbyState(),
        channel
      }
    }
    activeLobbyChannel.value = channel
    console.log(`Joined lobby: ${channel}`)
  },

  leaveLobby(channel: string) {
    delete lobbyStates[channel]
    
    if (activeLobbyChannel.value === channel) {
      activeLobbyChannel.value = null
    }
    console.log(`Left lobby: ${channel}`)
  },

  setActiveLobby(channel: string) {
    if (lobbyStates[channel]) {
      activeLobbyChannel.value = channel
    }
  },

  clearPlayers(channel: string) {
    const lobby = lobbyStates[channel]
    if (!lobby) return

    lobby.slots.forEach(slot => {
      slot.player = null
    })
  },

  updateSettings(channel: string, settings: Partial<LobbySettings>) {
    const lobby = lobbyStates[channel]
    if (!lobby) return

    if (!lobby.settings) {
      lobby.settings = {
        roomName: '',
        teamMode: 'HeadToHead',
        winCondition: 'Score',
        size: 16,
        ...settings
      }
    } else {
      Object.assign(lobby.settings, settings)
    }
    console.log(`Updated lobby settings for ${channel}:`, lobby.settings)
  },

  updateCurrentMap(channel: string, map: CurrentMap) {
    const lobby = lobbyStates[channel]
    if (!lobby) return

    lobby.currentMap = map
    console.log(`Updated current map for ${channel}:`, map)
  },

  addPlayer(channel: string, slotId: number, player: Player) {
    const lobby = lobbyStates[channel]
    if (!lobby) return

    const slot = lobby.slots.find(s => s.id === slotId)
    if (slot) {
      slot.player = player
      console.log(`Added player ${player.username} to slot ${slotId} in ${channel}`)
    }
  },

  removePlayer(channel: string, slotId: number) {
    const lobby = lobbyStates[channel]
    if (!lobby) return

    const slot = lobby.slots.find(s => s.id === slotId)
    if (slot) {
      slot.player = null
      console.log(`Removed player from slot ${slotId} in ${channel}`)
    }
  },

  updateMatchStatus(channel: string, status: LobbyState['matchStatus']) {
    const lobby = lobbyStates[channel]
    if (!lobby) return

    lobby.matchStatus = status

    if (status === 'ready') {
      lobby.slots.forEach(slot => {
        if (slot.player) {
          slot.player.isReady = true
        }
      })
    }
    console.log(`Match status updated to: ${status} in ${channel}`)
  },

  updateHost(channel: string, hostUsername: string) {
    const lobby = lobbyStates[channel]
    if (!lobby) return

    lobby.host = hostUsername
    
    // Update host flag on all players
    lobby.slots.forEach(slot => {
      if (slot.player) {
        slot.player.isHost = slot.player.username === hostUsername
      }
    })
    console.log(`Host updated to: ${hostUsername} in ${channel}`)
  },

  removePlayerByUsername(channel: string, username: string) {
    const lobby = lobbyStates[channel]
    if (!lobby) return

    const slot = lobby.slots.find(s => s.player?.username === username)
    if (slot) {
      this.removePlayer(channel, slot.id)
    }
  },
}

export const getLobbyState = (channel: string): LobbyState | null => {
  return lobbyStates[channel] || null
}

export const getAllLobbyChannels = (): string[] => {
  return Object.keys(lobbyStates)
}

export { lobbyStates, activeLobbyChannel }
