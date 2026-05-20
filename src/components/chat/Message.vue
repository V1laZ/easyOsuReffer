<template>
  <div class="group hover:bg-gray-800/50 px-3 py-2 rounded-lg transition-colors">
    <div class="flex items-start space-x-3">
      <!-- Avatar -->
      <div class="flex-shrink-0 mt-1">
        <button
          class="rounded-full focus:outline-none"
          :class="{
            'cursor-pointer': message.username !== 'BanchoBot',
            'cursor-default': message.username === 'BanchoBot',
          }"
          :disabled="message.username === 'BanchoBot'"
          @click="handleUsernameClick"
        >
          <Avatar :username="message.username" />
        </button>
      </div>

      <!-- Message Content -->
      <div class="flex-1 min-w-0">
        <!-- Header -->
        <div class="flex items-baseline space-x-2">
          <button
            class="font-semibold text-white"
            :class="{
              'hover:underline cursor-pointer': message.username !== 'BanchoBot',
            }"
            @click="handleUsernameClick"
          >
            {{ message.username }}
          </button>

          <span class="text-xs text-gray-500">
            {{ formattedTime }}
          </span>
        </div>

        <!-- Message Text -->
        <div class="mt-1">
          <p
            class="text-gray-100 break-words"
            v-html="formattedMessage"
          />
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { IrcMessage } from '@/types'
import { computed } from 'vue'
import { openUrl } from '@tauri-apps/plugin-opener'
import { globalState } from '@/stores/global'
import Avatar from '@/components/UI/Avatar.vue'

const props = defineProps<{
  message: IrcMessage
}>()

const emit = defineEmits<{
  clickUsername: [username: string]
}>()

const formattedTime = computed(() => {
  return new Date(props.message.timestamp * 1000).toLocaleTimeString([], {
    hour: '2-digit',
    minute: '2-digit',
  })
})

const formattedMessage = computed(() => {
  return props.message.message.replace(
    /(https?:\/\/[^\s]+)/g,
    '<a href="$1" target="_blank" rel="noopener noreferrer" class="text-blue-400 hover:underline">$1</a>',
  )
})

const handleUsernameClick = () => {
  if (props.message.username === 'BanchoBot') return
  if (!globalState.isConnectedOsu) {
    openUrl(`https://osu.ppy.sh/users/${encodeURIComponent(props.message.username)}`)
    return
  }
  emit('clickUsername', props.message.username)
}
</script>
