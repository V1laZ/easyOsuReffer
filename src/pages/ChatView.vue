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
    />

    <!-- Main Chat Area -->
    <div class="flex-1 flex flex-col min-w-0 relative">
      <!-- Header -->
      <ChatHeader 
        :active-channel="activeChannel"
        :connection-status="connectionStatus"
        @toggle-left-drawer="leftDrawerOpen = !leftDrawerOpen"
        @toggle-right-drawer="rightDrawerOpen = !rightDrawerOpen"
        @open-settings="settingsOpen = true"
        @open-mappools="mappoolsOpen = true"
      />

      <!-- Quick Action Bar (for match controls) -->
      <QuickActionBar 
        v-if="activeChannel && activeChannel.startsWith('#mp_')"
        :channel="activeChannel"
      />

      <!-- Messages Area -->
      <ChatMessages 
        :messages="messages"
        :loading="messagesLoading"
        class="flex-1"
      />

      <!-- Message Input -->
      <MessageInput 
        :disabled="!connectionStatus.connected"
        @send-message="sendMessage"
      />
    </div>

    <!-- Right Drawer - Users -->
    <UserDrawer 
      :is-open="rightDrawerOpen"
      :users="channelUsers"
      @close="rightDrawerOpen = false"
    />

    <!-- Settings Modal -->
    <SettingsModal 
      v-if="settingsOpen"
      @close="settingsOpen = false"
      @logout="handleLogout"
    />

    <!-- Mappools Modal -->
    <MappoolModal 
      v-if="mappoolsOpen"
      @close="mappoolsOpen = false"
    />

    <!-- Mobile Overlay -->
    <div 
      v-if="(leftDrawerOpen || rightDrawerOpen)"
      class="fixed inset-0 bg-black bg-opacity-50 z-30 lg:hidden"
      @click="closeDrawers"
    ></div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue'
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

interface IrcMessage {
  channel: string
  username: string
  message: string
  timestamp: number
  id: number
}

interface ConnectionStatus {
  connected: boolean
  username?: string
}

const router = useRouter()

// State
const leftDrawerOpen = ref(false)
const rightDrawerOpen = ref(false)
const settingsOpen = ref(false)
const mappoolsOpen = ref(false)
const messagesLoading = ref(false)

const channels = ref<string[]>([])
const activeChannel = ref<string | null>(null)
const messages = ref<IrcMessage[]>([])
const channelUsers = ref<string[]>([])
const messageIdCounter = ref(0)

const connectionStatus = ref<ConnectionStatus>({
  connected: false
})

// Event listeners
let unlistenMessage: UnlistenFn | null = null
let unlistenUserJoined: UnlistenFn | null = null
let unlistenUserLeft: UnlistenFn | null = null
let unlistenChannelError: UnlistenFn | null = null

onMounted(async () => {
  // Check connection status
  try {
    const connected = await invoke('get_connection_status') as boolean
    if (!connected) {
      router.push('/login')
      return
    }

    connectionStatus.value.connected = true
    await loadChannels()

    // Set up event listeners
    unlistenMessage = await listen('irc-message', (event) => {
      const message = event.payload as Omit<IrcMessage, 'id'>
      
      // Only process messages from channels we're still in
      if (!channels.value.includes(message.channel)) {
        return
      }
      
      const messageWithId = {
        ...message,
        id: messageIdCounter.value++
      }
      
      // Only add message if it's for the active channel or if no active channel
      if (!activeChannel.value || message.channel === activeChannel.value) {
        messages.value.push(messageWithId)
      }
    })

    unlistenUserJoined = await listen('user-joined', (event) => {
      console.log('User joined:', event.payload)
      // TODO: Update user list
    })

    unlistenUserLeft = await listen('user-left', (event) => {
      console.log('User left:', event.payload)
      // TODO: Update user list
    })

    unlistenChannelError = await listen('channel-error', (event) => {
      const errorData = event.payload as { channel: string, error: string }
      console.error('Channel error:', errorData)
      
      // Remove the channel from our list if it was optimistically added
      const channelIndex = channels.value.indexOf(errorData.channel)
      if (channelIndex !== -1) {
        channels.value.splice(channelIndex, 1)
        
        // If this was the active channel, switch to another
        if (activeChannel.value === errorData.channel) {
          if (channels.value.length > 0) {
            activeChannel.value = channels.value[0]
          } else {
            activeChannel.value = null
          }
          messages.value = []
        }
      }
      
      // Show error to user (you could replace this with a toast notification)
      alert(`Failed to join ${errorData.channel}: ${errorData.error}`)
    })

  } catch (error) {
    console.error('Failed to initialize chat:', error)
    router.push('/login')
  }
})

onUnmounted(() => {
  // Clean up event listeners
  if (unlistenMessage) unlistenMessage()
  if (unlistenUserJoined) unlistenUserJoined()
  if (unlistenUserLeft) unlistenUserLeft()
  if (unlistenChannelError) unlistenChannelError()
})

// Methods
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

const selectChannel = (channel: string) => {
  activeChannel.value = channel
  messages.value = [] // Clear messages when switching channels
  leftDrawerOpen.value = false
  
  // TODO: Load channel-specific users
  // TODO: Load recent messages for this channel if needed
}

const joinChannel = async (channelName: string) => {
  try {
    let channel = channelName.trim()
    
    // Basic validation
    if (!channel) {
      alert('Please enter a channel name')
      return
    }
    
    // Handle multiplayer room IDs
    const mpId = parseInt(channel, 10)
    if (!isNaN(mpId)) {
      channel = `#mp_${mpId}`
    }
    
    // Ensure channel starts with #
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
    messages.value = []
    
  } catch (error) {
    console.error('Failed to join channel:', error)
    // Show error message to user
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
    
    // If the left channel was the active channel, switch to another one
    if (activeChannel.value === channelName) {
      const remainingChannels = channels.value.filter(c => c !== channelName)
      if (remainingChannels.length > 0) {
        activeChannel.value = remainingChannels[0]
      } else {
        activeChannel.value = null
      }
      // Clear messages when switching channels
      messages.value = []
    }
    
    // Reload channels to update the list
    await loadChannels()
  } catch (error) {
    console.error('Failed to leave channel:', error)
  }
}
</script>
