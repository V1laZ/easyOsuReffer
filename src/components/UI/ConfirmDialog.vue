<template>
  <Modal
    v-model="open"
    :title="confirmState.title"
    size="sm"
  >
    <p
      v-if="confirmState.message"
      class="text-sm text-slate-300"
    >
      {{ confirmState.message }}
    </p>

    <template #footer>
      <Btn
        variant="ghost"
        @click="settle(false)"
      >
        {{ confirmState.cancelText }}
      </Btn>
      <Btn
        :variant="confirmState.tone === 'danger' ? 'danger' : 'primary'"
        @click="settle(true)"
      >
        {{ confirmState.confirmText }}
      </Btn>
    </template>
  </Modal>
</template>

<script setup lang="ts">
import { computed, onMounted, onUnmounted } from 'vue'
import Modal from './Modal.vue'
import Btn from './Btn.vue'
import { confirmState, settle } from '@/composables/useConfirm'

const open = computed({
  get: () => confirmState.open,
  set: (value) => {
    if (!value) settle(false)
  },
})

function onKeydown(e: KeyboardEvent) {
  if (!confirmState.open) return
  if (e.key === 'Enter') {
    e.preventDefault()
    settle(true)
  }
}

onMounted(() => document.addEventListener('keydown', onKeydown))
onUnmounted(() => document.removeEventListener('keydown', onKeydown))
</script>
