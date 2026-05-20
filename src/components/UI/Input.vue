<template>
  <input
    ref="el"
    v-model="model"
    :type="type"
    :placeholder="placeholder"
    :disabled="disabled"
    :maxlength="maxlength"
    :autofocus="autofocus"
    :required="required"
    :class="[
      'w-full rounded-lg bg-slate-800 px-3 py-2 text-sm text-slate-100 placeholder:text-slate-500',
      'ring-1 ring-inset ring-slate-700',
      'focus:outline-none focus:ring-2 focus:ring-pink-400/60',
      'disabled:cursor-not-allowed disabled:opacity-50',
      hasError ? 'ring-rose-400/60 focus:ring-rose-400/70' : '',
    ]"
  >
</template>

<script setup lang="ts">
import { ref } from 'vue'

const model = defineModel<string | number>({ default: '' })

const {
  type = 'text',
  disabled = false,
  hasError = false,
  autofocus = false,
  required = false,
} = defineProps<{
  type?: string
  placeholder?: string
  disabled?: boolean
  maxlength?: number
  hasError?: boolean
  autofocus?: boolean
  required?: boolean
}>()

const el = ref<HTMLInputElement | null>(null)

defineExpose({
  focus: () => el.value?.focus(),
})
</script>
