<template>
  <div class="border-b border-slate-800 bg-slate-900/60 p-4">
    <div class="space-y-3">
      <div
        v-if="!beatmapPreview"
        class="flex items-center gap-2"
      >
        <Input
          ref="beatmapInputRef"
          v-model="beatmapInput"
          placeholder="Beatmap ID or osu.ppy.sh link"
          @keyup.enter="fetchBeatmapData"
        />
        <Btn
          :disabled="!beatmapInput || isLoading"
          :loading="isLoading"
          @click="fetchBeatmapData"
        >
          Fetch
        </Btn>
      </div>

      <div
        v-if="beatmapPreview"
        class="rounded-lg border border-slate-800 bg-slate-800/50 p-4"
      >
        <div class="mb-4 flex items-center gap-3">
          <img
            :src="`https://assets.ppy.sh/beatmaps/${beatmapPreview.beatmapset_id}/covers/cover.jpg`"
            :alt="beatmapPreview.title"
            class="size-16 rounded-md object-cover ring-1 ring-inset ring-slate-700"
          >
          <div class="min-w-0 flex-1">
            <p class="truncate font-semibold text-slate-100">
              {{ beatmapPreview.artist }} - {{ beatmapPreview.title }}
            </p>
            <p class="truncate text-sm text-slate-300">
              [{{ beatmapPreview.difficulty }}] by {{ beatmapPreview.mapper }}
            </p>
            <div class="mt-1 flex items-center gap-3 text-xs text-slate-400">
              <span>★{{ beatmapPreview.difficulty_rating?.toFixed(2) }}</span>
              <span>{{ Math.floor(beatmapPreview.total_length / 60) }}:{{ String(beatmapPreview.total_length % 60).padStart(2, '0') }}</span>
              <span>{{ beatmapPreview.bpm }}&nbsp;BPM</span>
            </div>
          </div>
        </div>

        <div class="mb-4 grid grid-cols-1 gap-3 md:grid-cols-2">
          <Field
            label="Category"
            required
          >
            <Input
              v-model="beatmapMeta.category"
              placeholder="e.g. NM2, HD1"
            />
          </Field>
          <Field label="Mod combination">
            <div class="flex flex-wrap items-center gap-2 pt-1">
              <Mod
                v-for="mod in mods"
                :key="mod"
                class="cursor-pointer"
                :mod="mod"
                :active="computedMods.includes(mod)"
                @click="handleModSelect(mod)"
              />
            </div>
          </Field>
        </div>

        <div class="flex gap-2">
          <Btn
            variant="ghost"
            block
            @click="emit('cancel')"
          >
            Cancel
          </Btn>
          <Btn
            variant="success"
            block
            :disabled="!beatmapMeta.category"
            @click="addBeatmap"
          >
            Add to pool
          </Btn>
        </div>
      </div>

      <div
        v-if="fetchError"
        class="flex items-start gap-2 rounded-lg bg-rose-500/10 p-3 ring-1 ring-inset ring-rose-400/30"
      >
        <Icon
          name="alert"
          size="sm"
          class="mt-0.5 flex-shrink-0 text-rose-300"
        />
        <p class="text-sm text-rose-200">
          {{ fetchError }}
        </p>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, nextTick, onMounted, ref, useTemplateRef } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { dbService } from '@/services/database'
import { globalState } from '@/stores/global'
import Mod from '@/components/Mod.vue'
import Btn from '@/components/UI/Btn.vue'
import Input from '@/components/UI/Input.vue'
import Field from '@/components/UI/Field.vue'
import Icon from '@/components/UI/Icon.vue'
import { BeatmapData } from '@/types'

const props = defineProps<{
  selectedMappoolId: number
}>()

const emit = defineEmits<{
  cancel: []
  add: []
}>()

const beatmapInputRef = useTemplateRef<{ focus: () => void }>('beatmapInputRef')

onMounted(() => {
  nextTick(() => beatmapInputRef.value?.focus())
})

const mods = ['NF', 'HD', 'HR', 'DT', 'EZ', 'FL', 'HT', 'FM']
const selectedMods = ref<string[]>(['NF'])
const beatmapPreview = ref<BeatmapData | null>(null)
const beatmapInput = ref('')
const beatmapMeta = ref<{ category: string, mod_combination: string }>({ category: '', mod_combination: '' })
const isLoading = ref(false)
const fetchError = ref('')

const computedMods = computed(() => {
  if (selectedMods.value.includes('FM')) return ['FM']
  return selectedMods.value
})

const fetchBeatmapData = async () => {
  if (!globalState.user) return

  const beatmapId = extractBeatmapId(beatmapInput.value)
  if (!beatmapId) {
    fetchError.value = 'Invalid beatmap ID or URL format'
    return
  }

  isLoading.value = true
  fetchError.value = ''

  try {
    const accessToken = await dbService.getAccessToken(globalState.user || '')
    const data = await invoke<BeatmapData>('fetch_beatmap_data', {
      beatmapId: beatmapId,
      accessToken,
    })

    beatmapPreview.value = data
  }
  catch (error) {
    console.error('Failed to fetch beatmap data:', error)
    fetchError.value = error instanceof Error ? error.message : 'Failed to fetch beatmap data'
  }
  finally {
    isLoading.value = false
  }
}

const handleModSelect = (mod: string) => {
  if (selectedMods.value.includes(mod)) {
    selectedMods.value = selectedMods.value.filter(m => m !== mod)
    return
  }

  selectedMods.value.push(mod)

  switch (mod) {
    case 'DT':
      selectedMods.value = selectedMods.value.filter(m => m !== 'HT')
      break
    case 'HT':
      selectedMods.value = selectedMods.value.filter(m => m !== 'DT')
      break
    case 'HR':
      selectedMods.value = selectedMods.value.filter(m => m !== 'EZ')
      break
    case 'EZ':
      selectedMods.value = selectedMods.value.filter(m => m !== 'HR')
      break
  }
}

const extractBeatmapId = (input: string): string | null => {
  const urlPatterns = [
    /osu\.ppy\.sh\/beatmapsets\/\d+#\w+\/(\d+)/,
    /osu\.ppy\.sh\/b\/(\d+)/,
    /osu\.ppy\.sh\/beatmaps\/(\d+)/,
  ]

  for (const pattern of urlPatterns) {
    const match = input.match(pattern)
    if (match) return match[1]
  }

  if (/^\d+$/.test(input.trim())) {
    return input.trim()
  }

  return null
}

const addBeatmap = async () => {
  if (!beatmapPreview.value || !beatmapMeta.value.category) return

  const category = beatmapMeta.value.category.trim().toUpperCase()
  const modCombination = computedMods.value.join('')

  try {
    await dbService.addBeatmapToPool(
      props.selectedMappoolId,
      beatmapPreview.value.id,
      beatmapPreview.value.artist,
      beatmapPreview.value.title,
      beatmapPreview.value.difficulty,
      beatmapPreview.value.mapper,
      modCombination,
      category,
    )
    emit('add')
  }
  catch (error) {
    console.error('Failed to add beatmap to pool:', error)
    fetchError.value = error instanceof Error ? error.message : 'Failed to add beatmap to pool'
  }
}
</script>
