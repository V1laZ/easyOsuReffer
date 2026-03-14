<template>
  <Teleport to="body">
    <Transition
      appear
      enter-active-class="transition-opacity duration-150 ease-out"
      enter-from-class="opacity-0"
      enter-to-class="opacity-100"
    >
      <div
        v-if="modelValue"
        class="fixed inset-x-0 top-8 bottom-0 z-50 flex items-center justify-center p-4"
      >
        <!-- Backdrop -->
        <div
          class="absolute inset-0 bg-black/80"
          @click="close"
        />
        <!-- Content -->
        <Transition
          appear
          enter-active-class="transition-all duration-150 ease-out"
          enter-from-class="opacity-0 scale-95"
          enter-to-class="opacity-100 scale-100"
        >
          <div
            class="relative z-10 w-full"
            :inert="!modelValue || undefined"
            @click.stop
          >
            <slot />
          </div>
        </Transition>
      </div>
    </Transition>
  </Teleport>
</template>

<script setup lang="ts">
import { onMounted, onUnmounted } from 'vue'

const modelValue = defineModel<boolean>({ default: false })

function close() {
  modelValue.value = false
}

function onKeydown(e: KeyboardEvent) {
  if (e.key === 'Escape' && modelValue.value) close()
}

onMounted(() => document.addEventListener('keydown', onKeydown))
onUnmounted(() => document.removeEventListener('keydown', onKeydown))
</script>
