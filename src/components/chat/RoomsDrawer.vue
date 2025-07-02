<template>
  <div 
    class="fixed inset-y-0 left-0 z-40 w-80 bg-gray-800 border-r border-gray-700 transform transition-transform duration-300 ease-in-out lg:relative lg:translate-x-0 flex flex-col"
    :class="isOpen ? 'translate-x-0' : '-translate-x-full'"
  >
    <!-- Header -->
    <div class="flex items-center justify-between p-4 border-b border-gray-700">
      <h2 class="text-lg font-semibold text-white">Rooms</h2>
      <button 
        @click="emit('close')"
        class="lg:hidden p-2 rounded-lg hover:bg-gray-700 transition-colors"
      >
        <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
        </svg>
      </button>
    </div>

    <!-- Room List + Multiplayer Lobbies -->
    <div class="flex-1 overflow-y-auto flex flex-col">
      <div class="p-4 space-y-3">
        <h3 class="text-sm font-medium text-gray-400 uppercase tracking-wide">Chat</h3>
        <div 
          v-if="chatRooms.length > 0" 
          class="space-y-1"
        >
          <RoomItem 
            v-for="room in chatRooms" 
            :key="room.id" 
            :room="room" 
            :is-active="activeRoom === room.id" 
            @select="emit('selectRoom', room.id)" 
            @leave="handleLeaveRoom(room)"
          />
        </div>
  
        <div v-else class="text-gray-500 text-sm">
          No opened chat.
        </div>
      </div>

      <!-- Multiplayer Lobbies section -->
      <div class="p-4 mb-auto border-t border-gray-700">
        <div class="flex flex-col space-y-3">
          <h3 class="text-sm font-medium text-gray-400 uppercase tracking-wide">Multiplayer</h3>
          <div v-if="mutiplayerLobbies.length > 0" class="space-y-1">
            <RoomItem 
              v-for="room in mutiplayerLobbies" 
              :key="room.id" 
              :room="room" 
              :is-active="activeRoom === room.id" 
              @select="emit('selectRoom', room.id)" 
              @leave="handleLeaveRoom(room)"
            />
          </div>
          <button
            @click="emit('openCreateLobby')"
            class="w-full px-3 py-2 bg-gray-600 hover:bg-gray-700 text-white rounded-lg transition-colors flex items-center justify-center space-x-2"
          >
            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6v6m0 0v6m0-6h6m-6 0H6" />
            </svg>
            <span>Create Lobby</span>
          </button>
        </div>
      </div>
    </div>

    <!-- Join Room -->
    <div class="p-4 border-t bg-gray-800 border-gray-700">
      <div class="space-y-3">
        <h3 class="text-sm font-medium text-gray-400 uppercase tracking-wide">Join Room</h3>
        <div class="flex space-x-2">
          <input
            v-model="newRoomName"
            @keyup.enter="handleJoinOrMessage"
            type="text"
            placeholder='#channel or username'
            class="flex-1 px-3 py-2 bg-gray-700 border border-gray-600 rounded-lg text-white placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-pink-500 focus:border-transparent"
          />
          <button
            @click="handleJoinOrMessage"
            :disabled="!newRoomName.trim()"
            class="px-3 py-2 bg-pink-500 hover:bg-pink-600 disabled:bg-gray-600 disabled:cursor-not-allowed text-white rounded-lg transition-colors"
          >
            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6v6m0 0v6m0-6h6m-6 0H6" />
            </svg>
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, ref } from 'vue'
import RoomItem from './RoomItem.vue';

const props = defineProps<{
  isOpen: boolean
  rooms: Room[]
  activeRoom: string | null
}>()

const emit = defineEmits<{
  close: []
  selectRoom: [roomId: string]
  joinChannel: [channel: string]
  leaveRoom: [roomId: string]
  startPrivateMessage: [username: string]
  openCreateLobby: []
}>()

const newRoomName = ref('')

const mutiplayerLobbies = computed(() => {
  return props.rooms.filter(room => room.roomType === 'MultiplayerLobby')
})

const chatRooms = computed(() => {
  return props.rooms.filter(room => room.roomType !== 'MultiplayerLobby')
})

const handleJoinOrMessage = () => {
  const input = newRoomName.value.trim()
  
  if (!input) {
    alert('Please enter a channel name or username')
    return
  }
  
  if (input.startsWith('#')) {
    if (input.length > 49) {
      alert('Channel name is too long (max 49 characters)')
      return
    }
    
    if (input.includes(' ')) {
      alert('Channel names cannot contain spaces')
      return
    }
    
    emit('joinChannel', input)
  } else {
    if (input.length > 50) {
      alert('Username is too long (max 50 characters)')
      return
    }
    
    if (input.includes(' ')) {
      alert('Usernames cannot contain spaces')
      return
    }
    
    emit('startPrivateMessage', input)
  }
  
  newRoomName.value = ''
}

const handleLeaveRoom = (room: Room) => {
  const confirmMessage = room.roomType === 'MultiplayerLobby'
    ? `Are you sure you want to leave the multiplayer room ${room.displayName}? You will stop receiving messages from this room.`
    : room.roomType === 'Channel'
    ? `Are you sure you want to leave ${room.displayName}? You will stop receiving messages from this channel.`
    : `Are you sure you want to close the conversation with ${room.displayName}?`
  
  if (confirm(confirmMessage)) {
    emit('leaveRoom', room.id)
  }
}
</script>
