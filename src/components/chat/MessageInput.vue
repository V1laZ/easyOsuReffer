<template>
  <div class="border-t border-slate-800 bg-slate-900 p-3">
    <div class="relative">
      <input
        v-model="messageText"
        :disabled="disabled"
        placeholder="Type a message..."
        :class="[
          'w-full rounded-lg bg-slate-800 py-3 pl-4 pr-14 text-sm text-slate-100 placeholder:text-slate-500',
          'ring-1 ring-inset ring-slate-700',
          'focus:outline-none focus:ring-2 focus:ring-pink-400/60',
          'disabled:cursor-not-allowed disabled:opacity-50',
          hasError ? 'ring-rose-400/60' : '',
        ]"
        @keydown.enter="sendMessage"
      >

      <div
        v-if="messageText.length > 400"
        class="pointer-events-none absolute bottom-1 right-14 text-[10px]"
        :class="messageText.length > 500 ? 'text-rose-300' : 'text-slate-500'"
      >
        {{ messageText.length }}/500
      </div>

      <button
        type="button"
        :disabled="disabled || !canSend"
        class="absolute right-1.5 top-1/2 inline-flex size-9 -translate-y-1/2 items-center justify-center rounded-md bg-pink-500/15 text-pink-200 ring-1 ring-inset ring-pink-400/30 transition-colors hover:bg-pink-500/25 hover:text-pink-100 disabled:cursor-not-allowed disabled:opacity-50"
        title="Send"
        @click="sendMessage"
      >
        <Icon
          name="send"
          size="sm"
          class="rotate-90"
        />
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import Icon from '@/components/UI/Icon.vue'

const { disabled = false } = defineProps<{
  disabled?: boolean
}>()

const emit = defineEmits<{
  sendMessage: [message: string]
}>()

const messageText = ref('')
const hasError = ref(false)

const canSend = computed(() => {
  return messageText.value.trim().length > 0 && messageText.value.length <= 500
})

const sendMessage = () => {
  if (!canSend.value || disabled) return

  const message = messageText.value.trim()
  if (!message) return

  emit('sendMessage', message)
  messageText.value = ''
}
</script>
