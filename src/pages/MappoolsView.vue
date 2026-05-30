<template>
  <div class="flex grow flex-col overflow-hidden bg-slate-950 text-slate-100">
    <!-- Top bar -->
    <header class="flex items-center gap-3 border-b border-slate-800 bg-slate-900 px-4 py-3">
      <IconBtn
        icon="back"
        size="sm"
        title="Back to chat"
        @click="router.replace('/')"
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

    <div class="relative flex min-h-0 grow overflow-hidden">
      <!-- Pool list -->
      <aside
        class="w-full flex-col border-r border-slate-800 bg-slate-900/40 lg:flex lg:w-72 lg:shrink-0"
        :class="hasDetail ? 'hidden lg:flex' : 'flex'"
      >
        <MappoolList
          class="flex-1"
          :mappools="mappools"
          :selected-id="activeId"
          @select="goToPool"
        />

        <div class="space-y-2 border-t border-slate-800 p-3">
          <Btn
            block
            @click="poolFormOpen = true"
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
            @click="importOpen = true"
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
        :class="hasDetail ? 'flex' : 'hidden lg:flex'"
      >
        <RouterView />

        <div
          v-if="!hasDetail"
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
    </div>

    <MappoolFormModal
      v-model="poolFormOpen"
      @created="onCreated"
    />

    <ImportSheetModal
      v-model="importOpen"
      @imported="onCreated"
    />
  </div>
</template>

<script setup lang="ts">
import { computed, onMounted, ref } from 'vue'
import { RouterView, useRoute, useRouter } from 'vue-router'
import { useMappools } from '@/composables/useMappools'
import MappoolList from '@/components/Mappool/List.vue'
import MappoolFormModal from '@/components/Mappool/MappoolFormModal.vue'
import ImportSheetModal from '@/components/Mappool/ImportSheetModal.vue'
import IconBtn from '@/components/UI/IconBtn.vue'
import Icon from '@/components/UI/Icon.vue'
import Btn from '@/components/UI/Btn.vue'
import type { Mappool } from '@/types'

const route = useRoute()
const router = useRouter()
const { mappools, loadMappools } = useMappools()

const poolFormOpen = ref(false)
const importOpen = ref(false)

const activeId = computed(() => (route.params.id ? Number(route.params.id) : undefined))
const hasDetail = computed(() => !!route.params.id)

onMounted(loadMappools)

function goToPool(mappool: Mappool) {
  router.push(`/mappools/${mappool.id}`)
}

async function onCreated(id: number) {
  await loadMappools()
  router.push(`/mappools/${id}`)
}
</script>
