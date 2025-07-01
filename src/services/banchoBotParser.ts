import { lobbyActions, lobbyState } from '../stores/lobbyStore'

const BANCHOBOT_PATTERNS = {
  ROOM_NAME: /^Room name: (.+), History: https:\/\/osu\.ppy\.sh\/mp\/(\d+)$/,
  TEAM_MODE: /^Team mode: (.+), Win condition: (.+)$/,
  ACTIVE_MODS: /^Active mods: (.+)$/,
  PLAYERS: /^Players: (\d+)$/,
  SLOT_INFO: /^Slot (\d+)\s+(.+)$/,
  
  BEATMAP: /^Beatmap: https:\/\/osu\.ppy\.sh\/b\/(\d+) (.+) \[(.+)\]$/,
  BEATMAP_CHANGED: /^Beatmap changed to: (.+) \[(.+)\] \(https:\/\/osu\.ppy\.sh\/b\/(\d+)\)$/,
  
  PLAYER_JOINED: /^(.+) joined in slot (\d+)( for team (red|blue))?\.$/,
  PLAYER_LEFT: /^(.+) left the game\.$/,
  PLAYER_MOVED: /^(.+) moved to slot (\d+)( for team (red|blue))?\.$/,
  PLAYER_READY: /^(.+) is ready$/,
  PLAYER_NOT_READY: /^(.+) is not ready$/,
  
  ALL_PLAYERS_READY: /^All players are ready$/,
  MATCH_STARTED: /^The match has started!$/,
  MATCH_FINISHED: /^(.+) finished playing \(Score: ([\d,]+), (PASSED|FAILED)\)\.$/,
  
  HOST_CHANGED: /^(.+) became the host\.$/,
}

const TEAM_MODES: Record<string, LobbySettings['teamMode']> = {
  'Head To Head': 'HeadToHead',
  'Tag Coop': 'TagCoop',
  'Team Vs': 'TeamVs',
  'Tag Team Vs': 'TagTeamVs'
}

const WIN_CONDITIONS: Record<string, LobbySettings['winCondition']> = {
  'Score': 'Score',
  'Accuracy': 'Accuracy',
  'Combo': 'Combo',
  'Score V2': 'ScoreV2'
}

export class BanchoBotParser {
  private static parseSlotInfo(slotText: string, slotId: number) {
    const isReady = !slotText.includes('Not Ready')
    
    let username = ''
    const userMatch = slotText.match(/https?:\/\/osu\.ppy\.sh\/u\/\d+\s+([^\s\[]+)/)
    if (userMatch) {
      username = userMatch[1].trim()
    }
    
    if (!username) return
    
    const isHost = slotText.includes('[Host')
    
    let team: 'red' | 'blue' | null = null
    if (slotText.includes('[Team Blue]') || slotText.includes('/ Team Blue]')) {
      team = 'blue'
    } else if (slotText.includes('[Team Red]') || slotText.includes('/ Team Red]')) {
      team = 'red'
    }
    
    lobbyActions.addPlayer(slotId, {
      username,
      team,
      isReady,
      isPlaying: false,
      isHost
    })
  }

  static parseIrcMessage(message: IrcMessage): boolean {
    if (message.username !== 'BanchoBot') {
      const userLeftMatch = message.message.match(/^(.+) left (#mp_\d+)$/)
      if (userLeftMatch) {
        const username = userLeftMatch[1]
        const slot = lobbyState.slots.find((s: PlayerSlot) => s.player?.username === username)
        if (slot) {
          lobbyActions.removePlayer(slot.id)
        }
        return true
      }
      
      return false
    }

    return this.parseBanchoBotMessage(message)
  }

  static parseBanchoBotMessage(message: IrcMessage): boolean {
    if (message.username !== 'BanchoBot') {
      return false
    }

    const text = message.message

    let match = text.match(BANCHOBOT_PATTERNS.ROOM_NAME)
    if (match) {
      lobbyActions.updateSettings({
        roomName: match[1]
      })
      return true
    }

    match = text.match(BANCHOBOT_PATTERNS.TEAM_MODE)
    if (match) {
      const teamMode = TEAM_MODES[match[1]] || 'HeadToHead'
      const winCondition = WIN_CONDITIONS[match[2]] || 'Score'
      lobbyActions.updateSettings({
        teamMode,
        winCondition
      })
      return true
    }

    match = text.match(BANCHOBOT_PATTERNS.PLAYERS)
    if (match) {
      const playerCount = parseInt(match[1], 10)
      lobbyActions.updateSettings({
        size: Math.max(playerCount, 16)
      })
      return true
    }

    match = text.match(BANCHOBOT_PATTERNS.SLOT_INFO)
    if (match) {
      const slotId = parseInt(match[1], 10)
      const slotInfo = match[2]
      this.parseSlotInfo(slotInfo, slotId)
      return true
    }

    match = text.match(BANCHOBOT_PATTERNS.BEATMAP)
    if (match) {
      const beatmapId = parseInt(match[1], 10)
      const fullTitle = match[2]
      const difficulty = match[3]
      
      const titleMatch = fullTitle.match(/^(.+) - (.+)$/)
      if (titleMatch) {
        const currentMap: CurrentMap = {
          beatmapId,
          artist: titleMatch[1],
          title: titleMatch[2],
          difficulty,
        }
        lobbyActions.updateCurrentMap(currentMap)
      }
      return true
    }

    match = text.match(BANCHOBOT_PATTERNS.BEATMAP_CHANGED)
    if (match) {
      const fullTitle = match[1]
      const difficulty = match[2]
      const beatmapId = parseInt(match[3], 10)
      
      const titleMatch = fullTitle.match(/^(.+) - (.+)$/)
      if (titleMatch) {
        const currentMap: CurrentMap = {
          beatmapId,
          artist: titleMatch[1],
          title: titleMatch[2],
          difficulty,
        }
        lobbyActions.updateCurrentMap(currentMap)
      }
      return true
    }

    match = text.match(BANCHOBOT_PATTERNS.PLAYER_JOINED)
    if (match) {
      const username = match[1]
      const slotId = parseInt(match[2], 10)
      const team = (match[4] as 'red' | 'blue') || (slotId <= 8 ? 'red' : 'blue')
      
      lobbyActions.addPlayer(slotId, {
        username,
        team,
        isReady: false,
        isPlaying: false,
        isHost: false
      })
      return true
    }

    match = text.match(BANCHOBOT_PATTERNS.PLAYER_LEFT)
    if (match) {
      const username = match[1]
      const slot = lobbyState.slots.find((s: PlayerSlot) => s.player?.username === username)
      if (slot) {
        lobbyActions.removePlayer(slot.id)
      }
      return true
    }

    match = text.match(BANCHOBOT_PATTERNS.PLAYER_MOVED)
    if (match) {
      const username = match[1]
      const newSlotId = parseInt(match[2], 10)
      
      const oldSlot = lobbyState.slots.find((s: PlayerSlot) => s.player?.username === username)
      if (oldSlot) {
        const player = oldSlot.player!
        lobbyActions.removePlayer(oldSlot.id)
        
        lobbyActions.addPlayer(newSlotId, player)
      }
      return true
    }

    match = text.match(BANCHOBOT_PATTERNS.PLAYER_READY)
    if (match) {
      const username = match[1]
      const slot = lobbyState.slots.find((s: PlayerSlot) => s.player?.username === username)
      if (slot?.player) {
        slot.player.isReady = true
      }
      return true
    }

    match = text.match(BANCHOBOT_PATTERNS.PLAYER_NOT_READY)
    if (match) {
      const username = match[1]
      const slot = lobbyState.slots.find((s: PlayerSlot) => s.player?.username === username)
      if (slot?.player) {
        slot.player.isReady = false
      }
      return true
    }

    if (text.match(BANCHOBOT_PATTERNS.ALL_PLAYERS_READY)) {
      lobbyActions.updateMatchStatus('ready')
      return true
    }

    if (text.match(BANCHOBOT_PATTERNS.MATCH_STARTED)) {
      lobbyActions.updateMatchStatus('active')
      lobbyState.slots.forEach((slot: PlayerSlot) => {
        if (slot.player) {
          slot.player.isPlaying = true
        }
      })
      return true
    }

    match = text.match(BANCHOBOT_PATTERNS.MATCH_FINISHED)
    if (match) {
      const username = match[1]
      const slot = lobbyState.slots.find((s: PlayerSlot) => s.player?.username === username)
      if (slot?.player) {
        slot.player.isPlaying = false
      }
      
      lobbyActions.updateMatchStatus('idle')
      return true
    }

    match = text.match(BANCHOBOT_PATTERNS.HOST_CHANGED)
    if (match) {
      const newHost = match[1]
      lobbyActions.updateHost(newHost)
      return true
    }

    return false
  }
}
