<template>
  <div class="flex flex-grow flex-col overflow-hidden bg-slate-950 text-slate-100">
    <!-- Top bar -->
    <header class="flex items-center gap-3 border-b border-slate-800 bg-slate-900 px-4 py-3">
      <IconBtn
        icon="back"
        size="sm"
        title="Back to chat"
        @click="goBack"
      />
      <div class="min-w-0">
        <h1 class="truncate text-base font-semibold text-slate-100">
          Mappools
        </h1>
        <p class="hidden text-xs text-slate-400 sm:block">
          Manage your tournament mappools
        </p>
      </div>
    </header>

    <div class="relative flex min-h-0 flex-grow overflow-hidden">
      <!-- Pool list -->
      <aside
        class="w-full flex-col border-r border-slate-800 bg-slate-900/40 lg:flex lg:w-72 lg:shrink-0"
        :class="selectedMappool ? 'hidden lg:flex' : 'flex'"
      >
        <MappoolList
          class="flex-1"
          :mappools="mappools"
          :selected-id="selectedMappool?.id"
          @select="selectMappool"
        />

        <div class="space-y-2 border-t border-slate-800 p-3">
          <Btn
            block
            @click="createOpen = true"
          >
            <template #icon>
              <Icon
                name="plus"
                size="sm"
              />
            </template>
            New mappool
          </Btn>
          <Btn
            block
            variant="secondary"
            disabled
            title="Coming soon"
          >
            <template #icon>
              <Icon
                name="download"
                size="sm"
              />
            </template>
            Import from Google Sheets
          </Btn>
        </div>
      </aside>

      <!-- Detail -->
      <section
        class="min-w-0 flex-1 flex-col overflow-hidden"
        :class="selectedMappool ? 'flex' : 'hidden lg:flex'"
      >
        <template v-if="selectedMappool">
          <div class="flex items-start gap-3 border-b border-slate-800 bg-slate-900/60 p-4">
            <IconBtn
              icon="back"
              size="sm"
              class="lg:hidden"
              title="Back to mappools"
              @click="selectedMappool = null"
            />
            <div class="min-w-0 flex-1">
              <h2 class="truncate text-base font-medium text-slate-100">
                {{ selectedMappool.name }}
              </h2>
              <p
                v-if="selectedMappool.description"
                class="mt-0.5 truncate text-sm text-slate-400"
              >
                {{ selectedMappool.description }}
              </p>
              <span class="mt-1 block text-xs text-slate-500">{{ beatmaps.length }} maps</span>
            </div>

            <div class="flex shrink-0 items-center gap-2">
              <Btn
                variant="success"
                size="sm"
                class="gap-0! sm:gap-2"
                :class="{ 'ring-2 ring-emerald-400/50': panelOpen && !panelBeatmap }"
                @click="openAdd"
              >
                <template #icon>
                  <Icon
                    name="plus"
                    size="sm"
                  />
                </template>
                <span class="hidden sm:inline">Add beatmap</span>
              </Btn>
              <IconBtn
                icon="trash"
                variant="danger"
                size="sm"
                title="Delete mappool"
                @click="deleteMappool"
              />
            </div>
          </div>

          <BeatmapList
            class="flex-1"
            :beatmaps="beatmaps"
            editable
            @remove="onRemove"
            @edit="openEdit"
            @reordered="beatmaps = $event"
          />
        </template>

        <div
          v-else
          class="hidden flex-1 flex-col items-center justify-center p-8 text-center text-slate-500 lg:flex"
        >
          <Icon
            name="musicCollection"
            size="xl"
            class="mb-3 text-slate-600"
          />
          <p class="text-sm">
            Select a mappool
          </p>
          <p class="mt-1 text-xs text-slate-600">
            or create one to get started
          </p>
        </div>
      </section>

      <!-- Desktop add/edit slideover -->
      <SlideOver
        v-model="panelOpen"
        class="hidden lg:block"
      >
        <BeatmapPanel
          v-if="selectedMappool"
          :mappool-id="selectedMappool.id"
          :beatmap="panelBeatmap"
          :visible="panelOpen"
          @added="refreshBeatmaps"
          @saved="onSaved"
          @close="closePanel"
        />
      </SlideOver>

      <!-- Mobile add/edit bottom sheet -->
      <BottomSheet
        v-model="panelOpen"
        class="lg:hidden"
      >
        <BeatmapPanel
          v-if="selectedMappool"
          :mappool-id="selectedMappool.id"
          :beatmap="panelBeatmap"
          :visible="panelOpen"
          @added="refreshBeatmaps"
          @saved="onSaved"
          @close="closePanel"
        />
      </BottomSheet>
    </div>

    <CreateModal
      v-model="createOpen"
      @created="onCreated"
    />
  </div>
</template>

<script setup lang="ts">
import { onMounted, ref } from 'vue'
import { useRouter } from 'vue-router'
import { dbService } from '@/services/database'
import { confirm } from '@/composables/useConfirm'
import MappoolList from '@/components/Mappool/List.vue'
import BeatmapList from '@/components/Mappool/Beatmap/List.vue'
import BeatmapPanel from '@/components/Mappool/BeatmapPanel.vue'
import CreateModal from '@/components/Mappool/CreateModal.vue'
import BottomSheet from '@/components/UI/BottomSheet.vue'
import SlideOver from '@/components/UI/SlideOver.vue'
import IconBtn from '@/components/UI/IconBtn.vue'
import Icon from '@/components/UI/Icon.vue'
import Btn from '@/components/UI/Btn.vue'
import type { Mappool, BeatmapEntry } from '@/types'

const router = useRouter()

const mappools = ref<Mappool[]>([])
const selectedMappool = ref<Mappool | null>(null)
const beatmaps = ref<BeatmapEntry[]>([])
const createOpen = ref(false)
const panelOpen = ref(false)
const panelBeatmap = ref<BeatmapEntry | null>(null)

onMounted(loadMappools)

async function loadMappools() {
  try {
    mappools.value = await dbService.getMappools()
  }
  catch (error) {
    console.error('Failed to load mappools:', error)
  }
}

async function refreshBeatmaps() {
  if (!selectedMappool.value) return

  try {
    beatmaps.value = await dbService.getMappoolBeatmaps(selectedMappool.value.id)
  }
  catch (error) {
    console.error('Failed to load beatmaps:', error)
    beatmaps.value = []
  }
}

async function selectMappool(mappool: Mappool) {
  selectedMappool.value = mappool
  closePanel()
  await refreshBeatmaps()
}

function openAdd() {
  panelBeatmap.value = null
  panelOpen.value = true
}

function openEdit(beatmap: BeatmapEntry) {
  panelBeatmap.value = beatmap
  panelOpen.value = true
}

function closePanel() {
  panelOpen.value = false
}

function onSaved() {
  closePanel()
  refreshBeatmaps()
}

function onRemove(id: number) {
  if (panelBeatmap.value?.id === id) closePanel()
  refreshBeatmaps()
}

async function onCreated(id: number) {
  await loadMappools()
  const created = mappools.value.find(pool => pool.id === id)
  if (created) await selectMappool(created)
}

async function deleteMappool() {
  if (!selectedMappool.value) return

  const ok = await confirm({
    title: 'Delete mappool?',
    message: 'This action cannot be undone.',
    confirmText: 'Delete',
    tone: 'danger',
  })
  if (!ok) return

  try {
    await dbService.deleteMappool(selectedMappool.value.id)
    selectedMappool.value = null
    beatmaps.value = []
    closePanel()
    await loadMappools()
  }
  catch (error) {
    console.error('Failed to delete mappool:', error)
    alert('Failed to delete mappool')
  }
}

function goBack() {
  router.push('/')
}
</script>
