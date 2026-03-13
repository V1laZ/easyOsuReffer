<template>
  <div class="flex flex-col items-center justify-center space-y-3">
    <Spinner class="w-8 h-8 text-pink-500" />
    <LoadingText
      class="text-gray-400 text-sm"
      :text="text"
    />
  </div>
</template>

<script setup lang="ts">
import Spinner from './Spinner.vue'
import LoadingText from './LoadingText.vue'

defineProps<{
  text: string
}>()
</script>
