<template>
  <div class="flex items-center justify-between gap-3 border-b border-slate-800 bg-slate-900 px-4 py-3">
    <div class="flex min-w-0 items-center gap-3">
      <IconBtn
        icon="menu"
        size="sm"
        class="lg:hidden"
        @click="emit('toggleLeftDrawer')"
      />

      <div class="min-w-0">
        <h1 class="truncate text-base font-semibold text-slate-100">
          {{ displayChannelName }}
        </h1>
        <div
          v-if="activeChannel && activeChannel.roomType === 'MultiplayerLobby' && activeChannel.lobbyState.settings?.roomName"
          class="max-w-xs truncate text-xs text-slate-400 sm:max-w-md"
        >
          {{ activeChannel.lobbyState.settings.roomName }}
        </div>
      </div>
    </div>

    <div class="flex items-center gap-1">
      <IconBtn
        icon="musicCollection"
        size="sm"
        title="Mappools"
        @click="emit('openMappools')"
      />

      <IconBtn
        v-if="!isMpLobby"
        icon="settings"
        size="sm"
        title="Settings"
        @click="emit('openSettings')"
      />

      <IconBtn
        v-if="isMpLobby"
        icon="refresh"
        size="sm"
        title="Refresh"
        @click="emit('refresh')"
      />

      <IconBtn
        v-if="isMpLobby"
        icon="group"
        size="sm"
        title="Players"
        class="lg:hidden"
        @click="emit('toggleRightDrawer')"
      />
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import IconBtn from '@/components/UI/IconBtn.vue'
import type { RoomUnion } from '@/types'

const props = defineProps<{
  activeChannel: RoomUnion | null
}>()

const emit = defineEmits<{
  toggleLeftDrawer: []
  toggleRightDrawer: []
  openSettings: []
  openMappools: []
  refresh: []
}>()

const isMpLobby = computed(() => {
  return props.activeChannel && props.activeChannel.roomType === 'MultiplayerLobby'
})

const displayChannelName = computed(() => {
  if (!props.activeChannel) return ''
  if (props.activeChannel.roomType === 'MultiplayerLobby') {
    return props.activeChannel.id.replace('#mp_', 'Lobby ')
  }
  return props.activeChannel.id
})
</script>
