<template>
  <div class="h-screen bg-gray-900 text-white flex overflow-hidden">
    <!-- Left Drawer - Channels -->
    <RoomsDrawer 
      :is-open="leftDrawerOpen"
      :rooms="rooms"
      :active-room="activeRoom"
      @close="leftDrawerOpen = false"
      @select-room="selectRoom"
      @join-channel="joinChannel"
      @leave-room="leaveRoom"
      @open-create-lobby="createLobbyOpen = true"
      @start-private-message="startPrivateMessage"      
    />

    <!-- Main Chat Area -->
    <div class="flex-1 flex flex-col min-w-0 relative">
      <ChatHeader 
        :active-channel="activeRoom"
        :lobby-state="currentLobbyState"
        @toggle-left-drawer="leftDrawerOpen = !leftDrawerOpen"
        @toggle-right-drawer="rightDrawerOpen = !rightDrawerOpen"
        @open-settings="settingsOpen = true"
        @open-mappools="mappoolsOpen = true"
        @refresh="refreshLobbyState"
      />

      <QuickActionBar 
        v-if="activeRoom && activeRoom.startsWith('#mp_') && currentLobbyState"
        :channel="activeRoom"
        :lobby-state="currentLobbyState"
      />

      <div v-if="!activeRoom" class="text-center mt-2 flex-1 py-4 text-gray-500">
        <svg class="size-12 mx-auto mb-2 text-gray-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v3m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
        </svg>
        Select or join a channel to start chatting
      </div>
      <ChatMessages 
        v-else
        :messages="currentMessages"
        class="flex-1"
      />

      <MessageInput 
        :disabled="!globalState.isConnected || !activeRoom"
        @send-message="sendMessage"
      />
    </div>

    <UserDrawer
      v-if="activeRoom && activeRoom.startsWith('#mp_') && currentLobbyState"
      :is-open="rightDrawerOpen"
      :lobby-state="currentLobbyState"
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
      v-if="mappoolsOpen"
      @close="mappoolsOpen = false"
    />

    <CreateLobbyModal 
      v-if="createLobbyOpen"
      @close="createLobbyOpen = false"
      @create-lobby="handleCreateLobby"
    />

    <!-- Mobile Overlay -->
    <div
      class="fixed transition-colors inset-0 bg-black/80 z-30 lg:hidden"
      :class="{
        'pointer-events-none bg-transparent': !leftDrawerOpen && !rightDrawerOpen
      }"
      @click="closeDrawers"
    ></div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted, watch } from 'vue'
import { useRouter } from 'vue-router'
import { invoke } from '@tauri-apps/api/core'
import { dbService } from '../services/database'
import { listen, type UnlistenFn } from '@tauri-apps/api/event'
import RoomsDrawer from '../components/chat/RoomsDrawer.vue'
import UserDrawer from '../components/chat/UserDrawer.vue'
import ChatHeader from '../components/chat/ChatHeader.vue'
import QuickActionBar from '../components/chat/QuickActionBar.vue'
import ChatMessages from '../components/chat/ChatMessages.vue'
import MessageInput from '../components/chat/MessageInput.vue'
import SettingsModal from '../components/modals/SettingsModal.vue'
import MappoolModal from '../components/modals/MappoolModal.vue'
import CreateLobbyModal from '../components/modals/CreateLobbyModal.vue'
import { globalState } from '../stores/global'

const router = useRouter()

const leftDrawerOpen = ref(false)
const rightDrawerOpen = ref(false)
const settingsOpen = ref(false)
const mappoolsOpen = ref(false)
const createLobbyOpen = ref(false)

const rooms = ref<Room[]>([])
const activeRoom = ref<string | null>(null)
const currentMessages = ref<IrcMessage[]>([])
const currentLobbyState = ref<LobbyState | null>(null)

watch(activeRoom, async (newRoomId) => {
  if (!newRoomId) {
    currentMessages.value = []
    currentLobbyState.value = null
    return
  }

  // Load messages for the new room
  try {
    await invoke('set_active_room', { roomId: newRoomId })
    currentMessages.value = await invoke('get_room_messages', { roomId: newRoomId })
    
    // If it's a multiplayer lobby, get lobby state
    if (newRoomId.startsWith('#mp_')) {
      currentLobbyState.value = await getLobbyState(newRoomId)
    } else {
      currentLobbyState.value = null
    }
  } catch (error) {
    console.error('Failed to switch room:', error)
    currentMessages.value = []
    currentLobbyState.value = null
  }
})

let unlistenMessage: UnlistenFn | null = null
let unlistenChannelError: UnlistenFn | null = null
let unlistenLobbyUpdate: UnlistenFn | null = null

onMounted(async () => {
  try {
    await loadRooms()

    unlistenMessage = await listen('irc-message', (event) => {
      processMessage(event.payload as Omit<IrcMessage, 'id'>)
    })

    unlistenChannelError = await listen('room-error', (event) => {
      const errorData = event.payload as { channel: string, error: string }
      console.error('Room error:', errorData)
      
      // Remove the room from our list if it was optimistically added
      const roomIndex = rooms.value.findIndex(r => r.id === errorData.channel)
      if (roomIndex !== -1) {
        rooms.value.splice(roomIndex, 1)
        
        // If this was the active room, switch to another
        if (activeRoom.value === errorData.channel) {
          if (rooms.value.length > 0) {
            activeRoom.value = rooms.value[0].id
          } else {
            activeRoom.value = null
          }
        }
      }
      
      alert(`Failed to join ${errorData.channel}: ${errorData.error}`)
    })

    unlistenLobbyUpdate = await listen('lobby-updated', (event) => {
      const lobby = event.payload as LobbyState
      if (currentLobbyState.value && currentLobbyState.value.channel === lobby.channel) {
        currentLobbyState.value = lobby
      }
    })

  } catch (error) {
    console.error('Failed to initialize chat:', error)
    router.push('/login')
  }
})

onUnmounted(() => {
  if (unlistenMessage) unlistenMessage()
  if (unlistenChannelError) unlistenChannelError()
  if (unlistenLobbyUpdate) unlistenLobbyUpdate()
})


const getLobbyState = async (channel: string): Promise<LobbyState | null> => {
  return await invoke('get_lobby_state', { roomId: channel })
}

const refreshLobbyState = async () => {
  if (!activeRoom.value || !activeRoom.value.startsWith('#mp_')) {
    return
  }
  
  try {
    await invoke('send_message_to_room', {
      roomId: activeRoom.value,
      message: '!mp settings'
    })
  } catch (error) {
    console.error('Failed to refresh lobby state:', error)
  }
}

const processMessage = (message: IrcMessage) => {
  if (activeRoom.value !== message.roomId) return

  currentMessages.value.push(message)
}

const loadRooms = async () => {
  try {
    const roomList = await invoke('get_joined_rooms') as Room[]
    rooms.value = roomList
    if (roomList.length > 0 && !activeRoom.value) {
      activeRoom.value = roomList[0].id
    }
  } catch (error) {
    console.error('Failed to load rooms:', error)
  }
}

const selectRoom = async (roomId: string) => {
  activeRoom.value = roomId
  leftDrawerOpen.value = false
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

    if (rooms.value.some(r => r.id === channel)) {
      alert(`Already in channel ${channel}`)
      return
    }

    await invoke('join_channel', { roomId: channel })
    await loadRooms()
    
    activeRoom.value = channel
    
    // If it's a multiplayer lobby, auto-send !mp settings to get lobby information
    if (channel.startsWith('#mp_')) {
      try {
        await invoke('send_message_to_room', {
          roomId: channel,
          message: '!mp settings'
        })
      } catch (error) {
        console.error('Failed to send !mp settings:', error)
      }
    }
    
  } catch (error) {
    console.error('Failed to join channel:', error)
    const errorMessage = error instanceof Error ? error.message : 'Unknown error occurred'
    alert(`Failed to join channel: ${errorMessage}`)
  }
}

const startPrivateMessage = async (username: string) => {
  try {
    await invoke('start_private_message', { username })
    await loadRooms()
    activeRoom.value = username
  } catch (error) {
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
      roomId: activeRoom.value,
      message: messageText
    })
  } catch (error) {
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
    await dbService.deleteCredentials()
    globalState.user = null
    globalState.isConnected = false
    router.push('/login')
  } catch (error) {
    console.error('Failed to logout:', error)
  }
}

const leaveRoom = async (roomId: string) => {
  try {
    const room = rooms.value.find(r => r.id === roomId)
    if (!room) return
    
    if (room.roomType === 'Channel' || room.roomType === 'MultiplayerLobby') {
      await invoke('leave_channel', { roomId: roomId })
    } else if (room.roomType === 'PrivateMessage') {
      await invoke('close_private_message', { username: roomId })
    }
    
    // If this was the active room, switch to another one
    if (activeRoom.value === roomId) {
      const remainingRooms = rooms.value.filter(r => r.id !== roomId)
      if (remainingRooms.length > 0) {
        activeRoom.value = remainingRooms[0].id
      } else {
        activeRoom.value = null
      }
    }
    
    await loadRooms()
  } catch (error) {
    console.error('Failed to leave room:', error)
  }
}

const handleCreateLobby = async (command: string) => {
  try {
    await invoke('send_message_to_room', {
      roomId: '#multiplayer',
      message: command
    })

    createLobbyOpen.value = false
  } catch (error) {
    console.error('Failed to create lobby:', error)
    alert('Failed to create lobby. Make sure you are connected and try again.')
  }
}
</script>
