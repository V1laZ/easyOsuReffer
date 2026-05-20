<template>
  <Modal
    v-model="isOpen"
    size="xl"
    body-padding="none"
    wrapper-class="max-h-[90vh] min-h-[70vh]"
  >
    <template #header>
      <h2 class="text-lg font-semibold text-slate-100">
        Mappools
      </h2>
      <p class="mt-0.5 text-xs text-slate-400">
        Manage your tournament mappools
      </p>
    </template>

    <div class="flex h-full flex-1 flex-col overflow-hidden">
      <div
        v-if="!selectedMappool"
        class="relative flex flex-1 flex-col overflow-hidden"
      >
        <CreateForm
          v-if="showCreateForm"
          @create="refreshMappools(); showCreateForm = false"
          @cancel="showCreateForm = false"
        />

        <MappoolList
          v-else
          :mappools="mappools"
          @select="selectMappool"
        />

        <div
          v-if="!showCreateForm"
          class="border-t border-slate-800 p-4"
        >
          <Btn
            block
            @click="showCreateForm = true"
          >
            <template #icon>
              <Icon
                name="plus"
                size="sm"
              />
            </template>
            New mappool
          </Btn>
        </div>
      </div>

      <div
        v-else
        class="flex flex-1 flex-col overflow-hidden"
      >
        <div class="border-b border-slate-800 bg-slate-900/60 p-4">
          <button
            type="button"
            class="mb-3 inline-flex items-center gap-1.5 text-sm text-slate-400 transition-colors hover:text-slate-100"
            @click="backToList"
          >
            <Icon
              name="back"
              size="sm"
            />
            <span>Back to mappools</span>
          </button>

          <h3 class="text-base font-medium text-slate-100">
            {{ selectedMappool?.name }}
          </h3>
          <p
            v-if="selectedMappool?.description"
            class="mt-0.5 text-sm text-slate-400"
          >
            {{ selectedMappool.description }}
          </p>

          <div class="mt-3 flex items-center justify-between">
            <span class="text-xs text-slate-500">{{ selectedMappoolBeatmaps.length }} maps</span>
            <button
              type="button"
              class="text-xs text-rose-300 transition-colors hover:text-rose-200"
              @click="deleteMappool(selectedMappool!.id)"
            >
              Delete mappool
            </button>
          </div>
        </div>

        <template v-if="!globalState.isConnectedOsu">
          <div class="p-6 text-center text-slate-400">
            <p class="mb-3 text-sm">
              Connect osu! to manage beatmaps
            </p>
            <ConnectOsuBtn />
          </div>
        </template>

        <template v-else>
          <div class="flex flex-1 flex-col overflow-auto">
            <AddBeatmapCard
              v-if="showAddBeatmap"
              :selected-mappool-id="selectedMappool.id"
              @add="refreshBeatmaps(selectedMappool.id); showAddBeatmap = false"
              @cancel="showAddBeatmap = false"
            />

            <BeatmapList
              :beatmaps="selectedMappoolBeatmaps"
              @remove="refreshBeatmaps(selectedMappool.id)"
            />
          </div>

          <div
            v-if="!showAddBeatmap"
            class="border-t border-slate-800 p-4"
          >
            <Btn
              block
              variant="success"
              :disabled="!globalState.isConnectedOsu"
              @click="showAddBeatmap = true"
            >
              <template #icon>
                <Icon
                  name="plus"
                  size="sm"
                />
              </template>
              Add beatmap
            </Btn>
          </div>
        </template>
      </div>
    </div>
  </Modal>
</template>

<script setup lang="ts">
import { ref, watch } from 'vue'
import { dbService } from '@/services/database'
import { globalState } from '@/stores/global'
import CreateForm from '../Mappool/CreateForm.vue'
import MappoolList from '../Mappool/List.vue'
import AddBeatmapCard from '../Mappool/Beatmap/AddCard.vue'
import BeatmapList from '../Mappool/Beatmap/List.vue'
import ConnectOsuBtn from '../ConnectOsuBtn.vue'
import { Mappool, BeatmapEntry } from '@/types'
import Modal from '@/components/UI/Modal.vue'
import Btn from '@/components/UI/Btn.vue'
import Icon from '@/components/UI/Icon.vue'
import { confirm } from '@/composables/useConfirm'

const isOpen = defineModel<boolean>({ default: false })

const mappools = ref<Mappool[]>([])
const selectedMappool = ref<Mappool | null>(null)
const selectedMappoolBeatmaps = ref<BeatmapEntry[]>([])
const showCreateForm = ref(false)
const showAddBeatmap = ref(false)

watch(isOpen, (newVal) => {
  if (newVal) {
    refreshMappools()
    selectedMappool.value = null
    selectedMappoolBeatmaps.value = []
    showCreateForm.value = false
    showAddBeatmap.value = false
  }
})

const refreshMappools = async () => {
  try {
    const loadedMappools = await dbService.getMappools()
    mappools.value = loadedMappools
  }
  catch (error) {
    console.error('Failed to load mappools:', error)
  }
}

const refreshBeatmaps = async (selectedMappoolId: number) => {
  if (!selectedMappoolId) return

  try {
    selectedMappoolBeatmaps.value = await dbService.getMappoolBeatmaps(selectedMappoolId)
  }
  catch (error) {
    console.error('Failed to refresh beatmaps:', error)
    selectedMappoolBeatmaps.value = []
  }
}

const selectMappool = async (mappool: Mappool) => {
  selectedMappool.value = mappool
  showAddBeatmap.value = false

  try {
    selectedMappoolBeatmaps.value = await dbService.getMappoolBeatmaps(mappool.id!)
  }
  catch (error) {
    console.error('Failed to load mappool beatmaps:', error)
    selectedMappoolBeatmaps.value = []
  }
}

const backToList = () => {
  selectedMappool.value = null
  selectedMappoolBeatmaps.value = []
  showAddBeatmap.value = false
}

const deleteMappool = async (mappoolId: number) => {
  const ok = await confirm({
    title: 'Delete mappool?',
    message: 'This action cannot be undone.',
    confirmText: 'Delete',
    tone: 'danger',
  })
  if (!ok) return

  try {
    await dbService.deleteMappool(mappoolId)
    await refreshMappools()
    backToList()
  }
  catch (error) {
    console.error('Failed to delete mappool:', error)
    alert('Failed to delete mappool')
  }
}
</script>
