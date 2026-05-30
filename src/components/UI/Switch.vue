<template>
  <button
    type="button"
    role="switch"
    :aria-checked="model"
    :disabled="disabled"
    class="inline-flex h-6 w-11 shrink-0 items-center rounded-full transition-colors focus-visible:ring-2 focus-visible:ring-pink-400/60 disabled:cursor-not-allowed disabled:opacity-50"
    :class="model ? 'bg-pink-500/70' : 'bg-slate-700'"
    @click="model = !model"
  >
    <span
      class="ml-0.5 size-5 rounded-full bg-white shadow transition-transform"
      :class="model ? 'translate-x-5' : 'translate-x-0'"
    />
  </button>
</template>

<script setup lang="ts">
const model = defineModel<boolean>({ default: false })

defineProps<{
  disabled?: boolean
}>()
</script>
