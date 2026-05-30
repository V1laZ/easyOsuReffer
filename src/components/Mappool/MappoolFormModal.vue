<template>
  <Modal
    v-model="open"
    size="md"
  >
    <template #header>
      <h2 class="text-lg font-semibold text-slate-100">
        {{ isEdit ? 'Edit mappool' : 'New mappool' }}
      </h2>
      <p class="mt-0.5 text-sm text-slate-400">
        {{ isEdit ? 'Update the mappool name and description' : 'Give your mappool a name to get started' }}
      </p>
    </template>

    <form
      class="space-y-4"
      @submit.prevent="submit"
    >
      <Field
        label="Name"
        required
      >
        <Input
          ref="nameRef"
          v-model="name"
          placeholder="e.g. OWC 2024 Finals"
          required
        />
      </Field>

      <Field label="Description">
        <textarea
          v-model="description"
          placeholder="Optional description"
          rows="3"
          class="w-full resize-none rounded-lg bg-slate-800 px-3 py-2 text-sm text-slate-100 placeholder:text-slate-500 ring-1 ring-inset ring-slate-700 focus:outline-none focus:ring-2 focus:ring-pink-400/60"
        />
      </Field>

      <div class="flex gap-2 pt-1">
        <Btn
          variant="ghost"
          block
          @click="open = false"
        >
          Cancel
        </Btn>
        <Btn
          type="submit"
          block
          :loading="loading"
          :disabled="!name.trim()"
        >
          {{ isEdit ? 'Save changes' : 'Create' }}
        </Btn>
      </div>
    </form>
  </Modal>
</template>

<script setup lang="ts">
import { computed, nextTick, ref, useTemplateRef, watch } from 'vue'
import { dbService } from '@/services/database'
import Modal from '@/components/UI/Modal.vue'
import Input from '@/components/UI/Input.vue'
import Field from '@/components/UI/Field.vue'
import Btn from '@/components/UI/Btn.vue'
import type { Mappool } from '@/types'

const open = defineModel<boolean>({ default: false })

const props = defineProps<{
  mappool?: Mappool | null
}>()

const emit = defineEmits<{
  created: [id: number]
  updated: []
}>()

const nameRef = useTemplateRef('nameRef')

const name = ref('')
const description = ref('')
const loading = ref(false)

const isEdit = computed(() => !!props.mappool)

watch(open, (isOpen) => {
  if (!isOpen) return
  name.value = props.mappool?.name ?? ''
  description.value = props.mappool?.description ?? ''
  nextTick(() => nameRef.value?.focus())
})

const submit = async () => {
  if (!name.value.trim()) return

  loading.value = true
  try {
    if (props.mappool) {
      await dbService.updateMappool(props.mappool.id, name.value.trim(), description.value.trim() || undefined)
      emit('updated')
    }
    else {
      const id = await dbService.createMappool(name.value.trim(), description.value.trim() || undefined)
      emit('created', id)
    }
    open.value = false
  }
  catch (error) {
    console.error('Failed to save mappool:', error)
    alert('Failed to save mappool')
  }
  finally {
    loading.value = false
  }
}
</script>
