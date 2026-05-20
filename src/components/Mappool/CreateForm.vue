<template>
  <div class="border-b border-slate-800 bg-slate-900/60 p-4">
    <h3 class="mb-3 text-sm font-medium text-slate-100">
      Create new mappool
    </h3>
    <form
      class="space-y-3"
      @submit.prevent="createMappool"
    >
      <Input
        v-model="newMappool.name"
        placeholder="Mappool name (e.g. OWC 2024 Finals)"
        required
      />
      <textarea
        v-model="newMappool.description"
        placeholder="Optional description"
        rows="2"
        class="w-full resize-none rounded-lg bg-slate-800 px-3 py-2 text-sm text-slate-100 placeholder:text-slate-500 ring-1 ring-inset ring-slate-700 focus:outline-none focus:ring-2 focus:ring-pink-400/60"
      />
      <div class="flex gap-2">
        <Btn
          variant="ghost"
          block
          @click="emit('cancel')"
        >
          Cancel
        </Btn>
        <Btn
          type="submit"
          block
          :loading="loading"
        >
          Create
        </Btn>
      </div>
    </form>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { dbService } from '@/services/database'
import { NewMappoolForm } from '@/types'
import Input from '@/components/UI/Input.vue'
import Btn from '@/components/UI/Btn.vue'

const emit = defineEmits<{
  create: [mappool: NewMappoolForm]
  cancel: []
}>()

const newMappool = ref<NewMappoolForm>({
  name: '',
  description: '',
})
const loading = ref(false)

const createMappool = async () => {
  try {
    loading.value = true
    await dbService.createMappool(newMappool.value.name, newMappool.value.description)
    emit('create', newMappool.value)
  }
  catch (error) {
    console.error('Failed to create mappool:', error)
    alert('Failed to create mappool')
  }
  finally {
    loading.value = false
  }
}
</script>
