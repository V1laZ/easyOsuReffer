<template>
  <div class="group hover:bg-gray-800/50 px-3 py-2 rounded-lg transition-colors">
    <div class="flex items-start space-x-3">
      <!-- Avatar -->
      <div class="flex-shrink-0 mt-1">
        <div 
          class="size-8 rounded-full flex items-center justify-center"
          :class="{
            'bg-gray-600': message.username === 'BanchoBot',
            'bg-gradient-to-br from-pink-500 to-purple-600': message.username !== 'BanchoBot'
          }"
        >
          <span class="text-sm font-medium text-white">
            <template v-if="message.username === 'BanchoBot'">
              <svg  class="size-5 -mt-0.5" xmlns="http://www.w3.org/2000/svg" width="32" height="32" viewBox="0 0 24 24"><!-- Icon from Material Design Icons by Pictogrammers - https://github.com/Templarian/MaterialDesign/blob/master/LICENSE --><path fill="currentColor" d="M22 14h-1c0-3.87-3.13-7-7-7h-1V5.73A2 2 0 1 0 10 4c0 .74.4 1.39 1 1.73V7h-1c-3.87 0-7 3.13-7 7H2c-.55 0-1 .45-1 1v3c0 .55.45 1 1 1h1v1a2 2 0 0 0 2 2h14c1.11 0 2-.89 2-2v-1h1c.55 0 1-.45 1-1v-3c0-.55-.45-1-1-1M8.68 17.04L7.5 15.86l-1.18 1.18l-1.18-1.18L7.5 13.5l2.36 2.36zm9 0l-1.18-1.18l-1.18 1.18l-1.18-1.18l2.36-2.36l2.36 2.36z"/></svg>
            </template>
            <template v-else>
              {{ message.username.charAt(0).toUpperCase() }}
            </template>
          </span>
        </div>
      </div>
        

      <!-- Message Content -->
      <div class="flex-1 min-w-0">
        <!-- Header -->
        <div class="flex items-baseline space-x-2">
          <span class="font-semibold text-white hover:underline cursor-pointer">
            {{ message.username }}
          </span>
          
          <span class="text-xs text-gray-500">
            {{ formattedTime }}
          </span>
        </div>

        <!-- Message Text -->
        <div class="mt-1">
          <p 
            class="text-gray-100 break-words"
            v-html="formattedMessage"
          ></p>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue';

const props = defineProps<{
  message: IrcMessage
}>()

const formattedTime = computed(() => {
  return new Date(props.message.timestamp * 1000).toLocaleTimeString([], { 
    hour: '2-digit', 
    minute: '2-digit' 
  })
})

const formattedMessage = computed(() => {
  // Make URLs clickable
  return props.message.message.replace(
    /(https?:\/\/[^\s]+)/g,
    '<a href="$1" target="_blank" rel="noopener noreferrer" class="text-blue-400 hover:underline">$1</a>'
  )
})
</script>