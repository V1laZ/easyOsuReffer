<template>
  <Modal
    v-model="open"
    title="Invite player"
    size="md"
  >
    <form
      id="invitePlayerForm"
      class="space-y-4"
      @submit.prevent="handleSubmit"
    >
      <Field
        label="Player name"
        required
      >
        <Input
          ref="playerNameInputRef"
          v-model.trim="playerName"
          placeholder="Enter player name"
          :maxlength="15"
          autofocus
        />
      </Field>
    </form>

    <template #footer>
      <Btn
        variant="ghost"
        @click="open = false"
      >
        Cancel
      </Btn>
      <Btn
        form="invitePlayerForm"
        type="submit"
        :disabled="!playerName"
      >
        Invite
      </Btn>
    </template>
  </Modal>
</template>

<script setup lang="ts">
import { ref, useTemplateRef, watch, nextTick } from 'vue'
import Modal from '@/components/UI/Modal.vue'
import Btn from '@/components/UI/Btn.vue'
import Input from '@/components/UI/Input.vue'
import Field from '@/components/UI/Field.vue'

const open = defineModel<boolean>({ required: true })

const emit = defineEmits<{
  invite: [playerName: string]
}>()

const playerName = ref('')
const playerNameInputRef = useTemplateRef<{ focus: () => void }>('playerNameInputRef')

const handleSubmit = () => {
  if (!playerName.value) return
  emit('invite', playerName.value)
  playerName.value = ''
  open.value = false
}

watch(open, (newVal) => {
  if (newVal) {
    nextTick(() => playerNameInputRef.value?.focus())
  }
}, { immediate: true, flush: 'post' })
</script>
