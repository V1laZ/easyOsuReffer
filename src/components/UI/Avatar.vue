<template>
  <div
    class="rounded-full flex items-center justify-center overflow-hidden shrink-0"
    :class="[sizeClass, backgroundClass]"
  >
    <img
      v-if="avatarUrl"
      :src="avatarUrl"
      :alt="username"
      class="size-full object-cover"
    >
    <svg
      v-else-if="isBancho"
      class="-mt-0.5"
      :class="iconSizeClass"
      xmlns="http://www.w3.org/2000/svg"
      viewBox="0 0 24 24"
    ><path
      fill="currentColor"
      d="M22 14h-1c0-3.87-3.13-7-7-7h-1V5.73A2 2 0 1 0 10 4c0 .74.4 1.39 1 1.73V7h-1c-3.87 0-7 3.13-7 7H2c-.55 0-1 .45-1 1v3c0 .55.45 1 1 1h1v1a2 2 0 0 0 2 2h14c1.11 0 2-.89 2-2v-1h1c.55 0 1-.45 1-1v-3c0-.55-.45-1-1-1M8.68 17.04L7.5 15.86l-1.18 1.18l-1.18-1.18L7.5 13.5l2.36 2.36zm9 0l-1.18-1.18l-1.18 1.18l-1.18-1.18l2.36-2.36l2.36 2.36z"
    /></svg>
    <span
      v-else
      class="font-medium text-white"
      :class="textSizeClass"
    >
      {{ initial }}
    </span>
  </div>
</template>

<script setup lang="ts">
import { computed, onMounted } from 'vue'
import { useUserAvatar } from '@/composables/useUserAvatar'
import { globalState } from '@/stores/global'

const props = withDefaults(defineProps<{
  username: string
  size?: 'sm' | 'md' | 'lg'
}>(), {
  size: 'md',
})

const isBancho = computed(() => props.username === 'BanchoBot')

const { avatarUrl, fetchAvatar } = useUserAvatar(props.username)

const sizeClass = computed(() => ({
  sm: 'size-8',
  md: 'size-10',
  lg: 'size-12',
}[props.size]))

const iconSizeClass = computed(() => ({
  sm: 'size-4',
  md: 'size-5',
  lg: 'size-6',
}[props.size]))

const textSizeClass = computed(() => ({
  sm: 'text-xs',
  md: 'text-sm',
  lg: 'text-lg',
}[props.size]))

const backgroundClass = computed(() => {
  if (isBancho.value) return 'bg-gray-600'
  if (avatarUrl.value) return ''
  return 'bg-linear-to-br from-pink-500 to-purple-600'
})

const initial = computed(() => props.username.charAt(0).toUpperCase() || '?')

onMounted(() => {
  if (isBancho.value) return
  if (!globalState.isConnectedOsu) return
  if (!props.username) return
  fetchAvatar()
})
</script>
