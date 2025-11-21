<template>
  <div>
    {{ text }}{{ dots }}
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue'

defineProps<{
  text: string
}>()

const dots = ref('.')
let intervalId: number | null = null

onMounted(() => {
  intervalId = setInterval(() => {
    if (dots.value === '.') {
      dots.value = '..'
    }
    else if (dots.value === '..') {
      dots.value = '...'
    }
    else {
      dots.value = '.'
    }
  }, 500)
})

onUnmounted(() => {
  if (intervalId) {
    clearInterval(intervalId)
  }
})
</script>
