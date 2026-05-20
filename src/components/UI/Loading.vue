<template>
  <div class="flex flex-col items-center justify-center gap-3">
    <Spinner class="size-8 text-pink-300" />
    <LoadingText
      class="text-sm text-slate-400"
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
