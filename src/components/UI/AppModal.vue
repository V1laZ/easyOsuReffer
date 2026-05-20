<template>
  <Teleport to="body">
    <Transition
      appear
      enter-active-class="transition-opacity duration-150 ease-out"
      enter-from-class="opacity-0"
      enter-to-class="opacity-100"
      leave-active-class="transition-opacity duration-150 ease-in"
      leave-from-class="opacity-100"
      leave-to-class="opacity-0"
    >
      <div
        v-if="modelValue"
        :style="{ zIndex }"
        class="fixed inset-safe flex items-center justify-center p-4"
      >
        <div
          class="absolute inset-0 bg-slate-950/80 backdrop-blur-sm"
          @click="close"
        />
        <Transition
          appear
          enter-active-class="transition duration-150 ease-out"
          enter-from-class="opacity-0 scale-95"
          enter-to-class="opacity-100 scale-100"
        >
          <div
            class="relative w-full"
            :inert="!modelValue || undefined"
            @click="close"
          >
            <slot />
          </div>
        </Transition>
      </div>
    </Transition>
  </Teleport>
</template>

<script setup lang="ts">
import { useModalLayer } from '@/composables/useModalLayer'

const modelValue = defineModel<boolean>({ default: false })

function close() {
  modelValue.value = false
}

const { zIndex } = useModalLayer(modelValue, close)
</script>
