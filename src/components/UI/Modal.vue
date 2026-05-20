<template>
  <AppModal v-model="open">
    <div
      :class="[
        'mx-auto flex w-full flex-col overflow-hidden rounded-xl border border-slate-800 bg-slate-900 shadow-2xl',
        sizeClass,
        wrapperClass,
      ]"
      @click.stop
    >
      <div
        v-if="$slots.header || title"
        class="flex items-start justify-between gap-3 border-b border-slate-800 px-5 py-4"
      >
        <div class="min-w-0">
          <slot name="header">
            <h2 class="text-lg font-semibold text-slate-100">
              {{ title }}
            </h2>
            <p
              v-if="subtitle"
              class="mt-0.5 text-sm text-slate-400"
            >
              {{ subtitle }}
            </p>
          </slot>
        </div>
        <IconBtn
          icon="close"
          size="sm"
          @click="open = false"
        />
      </div>

      <div
        :class="[
          'min-h-0 flex-1',
          scroll ? 'overflow-y-auto' : '',
          bodyPaddingClass,
        ]"
      >
        <slot />
      </div>

      <div
        v-if="$slots.footer"
        class="flex items-center justify-end gap-2 border-t border-slate-800 px-5 py-3"
      >
        <slot name="footer" />
      </div>
    </div>
  </AppModal>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import AppModal from './AppModal.vue'
import IconBtn from './IconBtn.vue'

type Size = 'sm' | 'md' | 'lg' | 'xl'

const open = defineModel<boolean>({ required: true })

const {
  size = 'md',
  bodyPadding = 'md',
  scroll = false,
  wrapperClass = '',
} = defineProps<{
  title?: string
  subtitle?: string
  size?: Size
  bodyPadding?: 'none' | 'sm' | 'md' | 'lg'
  scroll?: boolean
  wrapperClass?: string
}>()

const sizeClass = computed(() => ({
  sm: 'max-w-sm',
  md: 'max-w-md',
  lg: 'max-w-2xl',
  xl: 'max-w-4xl',
}[size]))

const bodyPaddingClass = computed(() => ({
  none: '',
  sm: 'p-3',
  md: 'p-5',
  lg: 'p-6',
}[bodyPadding]))
</script>
