<template>
  <div class="h-[100dvh] bg-gray-900 text-white flex overflow-hidden">
    <!-- Left Drawer - Channels -->
    <RoomsDrawer
      :is-open="leftDrawerOpen"
      :rooms="roomsList"
      :active-room-id="activeRoom?.id"
      @close="leftDrawerOpen = false"
      @select-room="handleSelectRoom"
      @join-channel="joinChannel"
      @leave-room="leaveRoom"
      @open-create-lobby="createLobbyOpen = true"
      @start-private-message="startPrivateMessage"
    />

    <!-- Main Chat Area -->
    <div class="relative flex-1 flex flex-col min-w-0">
      <SelectMap
        v-if="activeRoom && activeRoom.roomType === 'MultiplayerLobby'"
        :is-open="isOpenSelectMap"
        :lobby-state="activeRoom.lobbyState"
        :room-id="activeRoom.id"
        @close="isOpenSelectMap = false"
        @set-mappool="activeRoom.lobbyState.currentMappoolId = $event"
        @select-beatmap="selectMap"
      />

      <ChatHeader
        :active-channel="activeRoom"
        @toggle-left-drawer="leftDrawerOpen = !leftDrawerOpen"
        @toggle-right-drawer="rightDrawerOpen = !rightDrawerOpen"
        @open-settings="settingsOpen = true"
        @open-mappools="mappoolsOpen = true"
        @refresh="refreshLobbyState"
      />

      <QuickActionBar
        v-if="activeRoom && activeRoom.roomType === 'MultiplayerLobby'"
        :room="activeRoom"
        @open-select-map="isOpenSelectMap = true"
        @send-message="sendMessage"
      />

      <div
        v-if="!activeRoom"
        class="text-center mt-2 flex-1 py-4 text-gray-500"
      >
        <svg
          class="size-12 mx-auto mb-2 text-gray-600"
          fill="none"
          stroke="currentColor"
          viewBox="0 0 24 24"
        >
          <path
            stroke-linecap="round"
            stroke-linejoin="round"
            stroke-width="2"
            d="M12 9v3m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
          />
        </svg>
        Select or join a channel to start chatting
      </div>
      <ChatMessages
        v-else
        :messages="activeRoom.messages"
        class="flex-1"
      />

      <MessageInput
        :disabled="!globalState.isConnected || !activeRoom"
        @send-message="sendMessage"
      />
    </div>

    <PlayersDrawer
      v-if="activeRoom && activeRoom.roomType === 'MultiplayerLobby'"
      :is-open="rightDrawerOpen"
      :lobby-state="activeRoom.lobbyState"
      @move="sendMessage(`!mp move ${$event.playerName} ${$event.to}`)"
      @team-change="sendMessage(`!mp team ${$event.playerName} ${$event.team}`)"
      @host="($event) => {
        if ($event === null) {
          sendMessage('!mp clearhost')
        } else {
          sendMessage(`!mp host ${$event}`)
        }
      }"
      @close="rightDrawerOpen = false"
    />

    <SettingsModal
      v-if="settingsOpen"
      :current-user="globalState.user"
      :is-connected="globalState.isConnected"
      @close="settingsOpen = false"
      @logout="handleLogout"
    />

    <MappoolModal
      v-model="mappoolsOpen"
      @close="mappoolsOpen = false"
    />

    <CreateLobbyModal
      v-if="createLobbyOpen"
      @close="createLobbyOpen = false"
      @create-lobby="handleCreateLobby"
    />

    <InvitePlayerModal
      @invite="sendMessage(`!mp invite ${$event}`)"
    />

    <!-- Mobile Overlay -->
    <div
      class="fixed transition-colors inset-0 bg-black/80 z-30 lg:hidden"
      :class="{
        'pointer-events-none bg-transparent': !leftDrawerOpen && !rightDrawerOpen
      }"
      @click="closeDrawers"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue'
import { useRouter } from 'vue-router'
import { invoke } from '@tauri-apps/api/core'
import { listen, type UnlistenFn } from '@tauri-apps/api/event'
import RoomsDrawer from '@/components/Drawer/Rooms/Rooms.vue'
import PlayersDrawer from '@/components/Drawer/Players/Players.vue'
import MappoolModal from '@/components/modals/Mappool.vue'
import ChatHeader from '@/components/chat/ChatHeader.vue'
import QuickActionBar from '@/components/chat/QuickActionBar.vue'
import ChatMessages from '@/components/chat/ChatMessages.vue'
import MessageInput from '@/components/chat/MessageInput.vue'
import SettingsModal from '@/components/modals/SettingsModal.vue'
import CreateLobbyModal from '@/components/modals/CreateLobbyModal.vue'
import { globalState } from '@/stores/global'
import SelectMap from '@/components/Drawer/SelectMap.vue'
import InvitePlayerModal from '@/components/modals/InvitePlayerModal.vue'
import { useIrcRooms } from '@/composables/useIrcRooms'
import type { CreateLobbySettings, BeatmapEntry, UserJoinEvent } from '@/types'

const router = useRouter()

const { roomsMap, activeRoom, roomsList, selectRoom } = useIrcRooms()

const isOpenSelectMap = ref(false)
const leftDrawerOpen = ref(false)
const rightDrawerOpen = ref(false)
const settingsOpen = ref(false)
const mappoolsOpen = ref(false)
const createLobbyOpen = ref(false)
const settingsForNewLobby = ref<CreateLobbySettings | null>(null)

let unlistenUserJoin: UnlistenFn | null = null

onMounted(async () => {
  unlistenUserJoin = await listen<UserJoinEvent>('user-joined', async ({ payload: joinEvent }) => {
    if (joinEvent.username.toLowerCase() !== globalState.user?.toLowerCase()) return

    leftDrawerOpen.value = false

    if (!joinEvent.channel.startsWith('#mp_')) return

    if (settingsForNewLobby.value) {
      try {
        await invoke('send_message_to_room', {
          roomId: joinEvent.channel,
          message: `!mp set ${settingsForNewLobby.value.teamMode} ${settingsForNewLobby.value.scoreMode} 16`,
        })
        settingsForNewLobby.value = null
      }
      catch (error) {
        console.error('Failed to set lobby settings:', error)
      }
    }

    try {
      await invoke('send_message_to_room', {
        roomId: joinEvent.channel,
        message: '!mp settings',
      })
    }
    catch (error) {
      console.error('Failed to send !mp settings:', error)
    }
  })
})

onUnmounted(() => {
  if (unlistenUserJoin) unlistenUserJoin()
})

const handleSelectRoom = async (roomId: string) => {
  await selectRoom(roomId)
  leftDrawerOpen.value = false
}

const refreshLobbyState = async () => {
  if (!activeRoom.value || activeRoom.value.roomType !== 'MultiplayerLobby') {
    return
  }

  try {
    await invoke('send_message_to_room', {
      roomId: activeRoom.value.id,
      message: '!mp settings',
    })
  }
  catch (error) {
    console.error('Failed to refresh lobby state:', error)
  }
}

const parseMods = (modString: string) => {
  if (modString === 'None') return 'None'
  const mods = modString.match(/.{1,2}/g) || []

  if (mods.length === 0) return 'None'

  return mods.map((mod) => {
    if (mod === 'FM') return 'Freemod'
    return mod
  }).join(' ')
}

const selectMap = async (beatmap: BeatmapEntry) => {
  if (!activeRoom.value || activeRoom.value.roomType !== 'MultiplayerLobby') return
  isOpenSelectMap.value = false

  try {
    await invoke('send_message_to_room', {
      roomId: activeRoom.value.id,
      message: `!mp map ${beatmap.beatmap_id}`,
    })
  }
  catch (error) {
    console.error('Failed to select map:', error)
    alert('Failed to select map. Make sure you are connected and try again.')
  }

  const mods = parseMods(beatmap.mod_combination || 'None')
  try {
    await invoke('send_message_to_room', {
      roomId: activeRoom.value.id,
      message: `!mp mods ${mods}`,
    })
  }
  catch (error) {
    console.error('Failed to set mods:', error)
    alert('Failed to set mods. Make sure you are connected and try again.')
  }
}

const joinChannel = async (channelName: string) => {
  try {
    let channel = channelName.trim()

    if (!channel) {
      alert('Please enter a channel name')
      return
    }

    const mpId = parseInt(channel, 10)
    if (!isNaN(mpId)) {
      channel = `#mp_${mpId}`
    }

    if (!channel.startsWith('#')) {
      channel = '#' + channel
    }

    if (roomsMap.value.has(channel)) {
      alert(`Already in channel ${channel}`)
      return
    }

    await invoke('join_channel', { roomId: channel })
  }
  catch (error) {
    console.error('Failed to join channel:', error)
    const errorMessage = error instanceof Error ? error.message : 'Unknown error occurred'
    alert(`Failed to join channel: ${errorMessage}`)
  }
}

const startPrivateMessage = async (username: string) => {
  try {
    await invoke('start_private_message', { username })
    await selectRoom(username)
  }
  catch (error) {
    console.error('Failed to start private message:', error)
    alert('Failed to start private message')
  }
}

const sendMessage = async (messageText: string) => {
  if (!activeRoom.value || !messageText.trim()) {
    return
  }

  try {
    await invoke('send_message_to_room', {
      roomId: activeRoom.value.id,
      message: messageText,
    })
  }
  catch (error) {
    console.error('Failed to send message:', error)
  }
}

const closeDrawers = () => {
  leftDrawerOpen.value = false
  rightDrawerOpen.value = false
}

const handleLogout = async () => {
  try {
    await invoke('disconnect_from_bancho')
    globalState.user = null
    globalState.isConnected = false
    router.replace('/login')
  }
  catch (error) {
    console.error('Failed to logout:', error)
  }
}

const leaveRoom = async (roomId: string) => {
  try {
    const room = roomsMap.value.get(roomId)
    if (!room) return

    if (room.roomType === 'Channel' || room.roomType === 'MultiplayerLobby') {
      await invoke('leave_channel', { roomId })
    }
    else if (room.roomType === 'PrivateMessage') {
      await invoke('close_private_message', { username: roomId })
    }
  }
  catch (error) {
    console.error('Failed to leave room:', error)
  }
}

const handleCreateLobby = async (settings: CreateLobbySettings) => {
  try {
    await invoke('start_private_message', { username: 'BanchoBot' })

    await invoke('send_message_to_room', {
      roomId: 'BanchoBot',
      message: `!mp make ${settings.name}`,
    })

    settingsForNewLobby.value = settings
    createLobbyOpen.value = false
  }
  catch (error) {
    console.error('Failed to create lobby:', error)
    alert('Failed to create lobby. Make sure you are connected and try again.')
  }
}
</script>
