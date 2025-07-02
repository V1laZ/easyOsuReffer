<template>
  <div class="h-screen bg-gray-900 text-white flex overflow-hidden">
    <!-- Left Drawer - Channels -->
    <ChannelDrawer 
      :is-open="leftDrawerOpen"
      :channels="channels"
      :active-channel="activeChannel"
      @close="leftDrawerOpen = false"
      @select-channel="selectChannel"
      @join-channel="joinChannel"
      @leave-channel="leaveChannel"
      @open-create-lobby="openCreateLobby"
    />

    <!-- Main Chat Area -->
    <div class="flex-1 flex flex-col min-w-0 relative">
      <!-- Header -->
      <ChatHeader 
        :active-channel="activeChannel"
        :lobby-state="currentLobbyState"
        @toggle-left-drawer="leftDrawerOpen = !leftDrawerOpen"
        @toggle-right-drawer="rightDrawerOpen = !rightDrawerOpen"
        @open-settings="settingsOpen = true"
        @open-mappools="mappoolsOpen = true"
        @refresh="refreshLobbyState"
      />

      <!-- Quick Action Bar (for match controls) -->
      <QuickActionBar 
        v-if="activeChannel && activeChannel.startsWith('#mp_') && currentLobbyState"
        :channel="activeChannel"
        :lobby-state="currentLobbyState"
      />

      <!-- Messages Area -->
      <div v-if="!activeChannel" class="text-center mt-2 flex-1 py-4 text-gray-500">
        <svg class="size-12 mx-auto mb-2 text-gray-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v3m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
        </svg>
        Select a channel to start chatting
      </div>
      <ChatMessages 
        v-else
        :messages="messages"
        class="flex-1"
      />

      <!-- Message Input -->
      <MessageInput 
        :disabled="!isConnected || !activeChannel"
        @send-message="sendMessage"
      />
    </div>

    <!-- Right Drawer - Users (only for multiplayer lobbies) -->
    <UserDrawer
      v-if="activeChannel && activeChannel.startsWith('#mp_') && currentLobbyState"
      :is-open="rightDrawerOpen"
      :lobby-state="currentLobbyState"
      @close="rightDrawerOpen = false"
    />

    <!-- Settings Modal -->
    <SettingsModal 
      v-if="settingsOpen"
      :current-user="user"
      :is-connected="isConnected"
      @close="settingsOpen = false"
      @logout="handleLogout"
    />

    <!-- Mappools Modal -->
    <MappoolModal 
      v-if="mappoolsOpen"
      @close="mappoolsOpen = false"
    />

    <!-- Create Lobby Modal -->
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
import { ref, onMounted, onUnmounted, computed, watch } from 'vue'
import { useRouter } from 'vue-router'
import { invoke } from '@tauri-apps/api/core'
import { dbService } from '../services/database'
import { listen, type UnlistenFn } from '@tauri-apps/api/event'
import ChannelDrawer from '../components/chat/ChannelDrawer.vue'
import UserDrawer from '../components/chat/UserDrawer.vue'
import ChatHeader from '../components/chat/ChatHeader.vue'
import QuickActionBar from '../components/chat/QuickActionBar.vue'
import ChatMessages from '../components/chat/ChatMessages.vue'
import MessageInput from '../components/chat/MessageInput.vue'
import SettingsModal from '../components/modals/SettingsModal.vue'
import MappoolModal from '../components/modals/MappoolModal.vue'
import CreateLobbyModal from '../components/modals/CreateLobbyModal.vue'

const props = defineProps<{
  user: string | null
  isConnected: boolean
}>()

const router = useRouter()

const leftDrawerOpen = ref(false)
const rightDrawerOpen = ref(false)
const settingsOpen = ref(false)
const mappoolsOpen = ref(false)
const createLobbyOpen = ref(false)

const channels = ref<string[]>([])
const activeChannel = ref<string | null>(null)
const channelMessages = ref<Record<string, IrcMessage[]>>({})
const messageIdCounter = ref(0)
const currentLobbyState = ref<LobbyState | null>(null)

const messages = computed(() => {
  if (!activeChannel.value) return []
  return channelMessages.value[activeChannel.value] || []
})

watch(activeChannel, async (newChannel) => {
  if (!newChannel || !newChannel.startsWith('#mp_')) {
    return
  }

  try {
    currentLobbyState.value = await getLobbyState(newChannel)
  } catch (error) {
    console.error('Failed to get lobby state:', error)
    currentLobbyState.value = null
  }
})

let unlistenMessage: UnlistenFn | null = null
let unlistenChannelError: UnlistenFn | null = null
let unlistenLobbyUpdate: UnlistenFn | null = null

onMounted(async () => {
  try {
    await loadChannels()

    unlistenMessage = await listen('irc-message', (event) => {
      processMessage(event.payload as Omit<IrcMessage, 'id'>)
    })

    unlistenChannelError = await listen('channel-error', (event) => {
      const errorData = event.payload as { channel: string, error: string }
      console.error('Channel error:', errorData)
      
      // Remove the channel from our list if it was optimistically added
      const channelIndex = channels.value.indexOf(errorData.channel)
      if (channelIndex !== -1) {
        channels.value.splice(channelIndex, 1)
        
        // Remove messages for this channel
        delete channelMessages.value[errorData.channel]
        
        // If this was the active channel, switch to another
        if (activeChannel.value === errorData.channel) {
          if (channels.value.length > 0) {
            activeChannel.value = channels.value[0]
          } else {
            activeChannel.value = null
          }
        }
      }
      
      // Show error to user (you could replace this with a toast notification)
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
  return await invoke('get_lobby_state', { channel })
}

const refreshLobbyState = async () => {
  if (!activeChannel.value || !activeChannel.value.startsWith('#mp_')) {
    return
  }
  
  try {
    await invoke('send_message_to_channel', {
      channel: activeChannel.value,
      message: '!mp settings'
    })
    // Backend now manages lobby state automatically
  } catch (error) {
    console.error('Failed to refresh lobby state:', error)
  }
}

const processMessage = (message: Omit<IrcMessage, 'id'>) => {
  // Only process messages from channels we're still in
  if (!channels.value.includes(message.channel)) {
    return
  }
  
  const messageWithId = {
    ...message,
    id: messageIdCounter.value++
  }
  
  if (!channelMessages.value[message.channel]) {
    channelMessages.value[message.channel] = []
  }
  
  channelMessages.value[message.channel].push(messageWithId)
}

const loadChannels = async () => {
  try {
    const channelList = await invoke('get_joined_channels') as string[]
    channels.value = channelList
    if (channelList.length > 0 && !activeChannel.value) {
      activeChannel.value = channelList[0]
    }
  } catch (error) {
    console.error('Failed to load channels:', error)
  }
}

const selectChannel = async (channel: string) => {
  activeChannel.value = channel
  leftDrawerOpen.value = false
  
  // Initialize channel messages array if it doesn't exist
  if (!channelMessages.value[channel]) {
    channelMessages.value[channel] = []
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

    // Check if already in channel
    if (channels.value.includes(channel)) {
      alert(`Already in channel ${channel}`)
      return
    }

    console.log(`Attempting to join channel: ${channel}`)
    await invoke('join_channel', { channel })
    await loadChannels()
    
    // Switch to the newly joined channel
    activeChannel.value = channel
    
    // Initialize channel messages array if it doesn't exist
    if (!channelMessages.value[channel]) {
      channelMessages.value[channel] = []
    }
    
    // If it's a multiplayer lobby, auto-send !mp settings to get lobby information
    if (channel.startsWith('#mp_')) {
      try {
        await invoke('send_message_to_channel', {
          channel: channel,
          message: '!mp settings'
        })
        console.log(`Auto-sent !mp settings to ${channel}`)
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

const sendMessage = async (messageText: string) => {
  if (!activeChannel.value || !messageText.trim()) {
    return
  }

  try {
    await invoke('send_message_to_channel', {
      channel: activeChannel.value,
      message: messageText
    })
    const message: Omit<IrcMessage, 'id'> = {
      channel: activeChannel.value,
      username: props.user || 'Unknown',
      message: messageText,
      timestamp: Math.floor(Date.now() / 1000),
    }
    
    processMessage(message)
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
    router.push('/login')
  } catch (error) {
    console.error('Failed to logout:', error)
  }
}

const leaveChannel = async (channelName: string) => {
  try {
    await invoke('leave_channel', { channel: channelName })
    
    // Remove messages for the left channel
    delete channelMessages.value[channelName]
    
    // If the left channel was the active channel, switch to another one
    if (activeChannel.value === channelName) {
      const remainingChannels = channels.value.filter(c => c !== channelName)
      if (remainingChannels.length > 0) {
        activeChannel.value = remainingChannels[0]
      } else {
        activeChannel.value = null
      }
    }
    
    await loadChannels()
  } catch (error) {
    console.error('Failed to leave channel:', error)
  }
}

const openCreateLobby = () => {
  createLobbyOpen.value = true
  leftDrawerOpen.value = false
}

const handleCreateLobby = async (settings: CreateLobbySettings) => {
  console.log(settings)
  createLobbyOpen.value = false
  return
}
</script>
