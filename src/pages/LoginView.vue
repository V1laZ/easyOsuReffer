<template>
  <div class="flex min-h-full items-center justify-center bg-slate-950 px-4 py-8">
    <div class="w-full max-w-sm">
      <div class="mb-8 flex flex-col items-center text-center">
        <div class="mb-4 flex size-16 items-center justify-center rounded-full bg-pink-500/15 text-pink-200 ring-1 ring-inset ring-pink-400/30">
          <svg
            class="size-8"
            fill="currentColor"
            viewBox="0 0 24 24"
            aria-hidden="true"
          >
            <path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm-2 15l-5-5 1.41-1.41L10 14.17l7.59-7.59L19 8l-9 9z" />
          </svg>
        </div>
        <h1 class="text-2xl font-semibold text-slate-100">
          osu!Reffer
        </h1>
        <p class="mt-1 text-sm text-slate-400">
          Ref osu! matches or just chat with others.
        </p>
      </div>

      <div class="rounded-xl border border-slate-800 bg-slate-900 p-6 shadow-xl">
        <form
          class="space-y-5"
          @submit.prevent="handleLogin"
        >
          <Field
            label="osu! username"
            required
          >
            <Input
              v-model="loginForm.username"
              placeholder="Your osu! username"
              :disabled="isConnecting"
              autofocus
              required
            />
          </Field>

          <Field
            label="IRC password"
            required
          >
            <div class="relative">
              <Input
                v-model="loginForm.password"
                :type="showPassword ? 'text' : 'password'"
                placeholder="Your IRC password"
                :disabled="isConnecting"
                required
              />
              <IconBtn
                :icon="showPassword ? 'eyeOff' : 'eye'"
                size="sm"
                class="absolute right-1.5 top-1/2 -translate-y-1/2"
                :disabled="isConnecting"
                @click="showPassword = !showPassword"
              />
            </div>
          </Field>

          <label class="flex cursor-pointer items-center gap-2 text-sm text-slate-300">
            <input
              v-model="loginForm.rememberMe"
              type="checkbox"
              class="size-4 rounded border-slate-600 bg-slate-800 text-pink-400 accent-pink-400 focus:ring-pink-400/60"
              :disabled="isConnecting"
            >
            <span>Remember my credentials</span>
          </label>

          <Btn
            type="submit"
            block
            size="lg"
            :loading="isConnecting"
            :disabled="!loginForm.username.trim() || !loginForm.password.trim()"
          >
            {{ isConnecting ? 'Connecting...' : 'Connect to Bancho' }}
          </Btn>
        </form>

        <div
          v-if="connectionStatus"
          class="mt-4 flex items-start gap-2 rounded-lg p-3 text-sm"
          :class="connectionStatus.type === 'error'
            ? 'bg-rose-500/10 text-rose-200 ring-1 ring-inset ring-rose-400/30'
            : 'bg-emerald-500/10 text-emerald-200 ring-1 ring-inset ring-emerald-400/30'"
        >
          <Icon
            :name="connectionStatus.type === 'error' ? 'alert' : 'check'"
            size="sm"
            class="mt-0.5 flex-shrink-0"
          />
          <span>{{ connectionStatus.message }}</span>
        </div>

        <div class="mt-6 border-t border-slate-800 pt-4 text-center">
          <button
            type="button"
            class="text-sm text-pink-300 transition-colors hover:text-pink-200"
            @click="showHelp = true"
          >
            How to get an IRC password?
          </button>
        </div>
      </div>
    </div>

    <Modal
      v-model="showHelp"
      title="Getting your IRC password"
      size="sm"
    >
      <div class="space-y-2 text-sm text-slate-300">
        <p><span class="text-slate-500">1.</span> Open your <span class="text-pink-300">osu! account settings</span>.</p>
        <p><span class="text-slate-500">2.</span> Scroll down to the <span class="text-pink-300">Legacy API</span> section.</p>
        <p><span class="text-slate-500">3.</span> Copy the <span class="text-pink-300">IRC server password</span>.</p>
        <p><span class="text-slate-500">4.</span> Paste it into the IRC password field.</p>
      </div>
      <template #footer>
        <Btn @click="showHelp = false">
          Got it
        </Btn>
      </template>
    </Modal>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { useRouter } from 'vue-router'
import { invoke } from '@tauri-apps/api/core'
import { dbService } from '@/services/database'
import { globalState } from '@/stores/global'
import type { ConnectionStatus } from '@/types'
import Btn from '@/components/UI/Btn.vue'
import IconBtn from '@/components/UI/IconBtn.vue'
import Icon from '@/components/UI/Icon.vue'
import Input from '@/components/UI/Input.vue'
import Field from '@/components/UI/Field.vue'
import Modal from '@/components/UI/Modal.vue'

const router = useRouter()

const loginForm = ref({
  username: '',
  password: '',
  rememberMe: true,
})

const showPassword = ref(false)
const isConnecting = ref(false)
const connectionStatus = ref<ConnectionStatus | null>(null)
const showHelp = ref(false)

const handleLogin = async () => {
  if (!loginForm.value.username.trim() || !loginForm.value.password.trim()) {
    connectionStatus.value = {
      type: 'error',
      message: 'Please fill in all fields',
    }
    return
  }

  isConnecting.value = true
  connectionStatus.value = null

  try {
    const config = {
      username: loginForm.value.username.trim(),
      password: loginForm.value.password.trim(),
    }

    await invoke('connect_to_bancho', { config })

    if (loginForm.value.rememberMe) {
      await dbService.saveCredentials(config.username, config.password)
    }
    else {
      await dbService.deleteCredentials()
    }

    globalState.user = config.username
    globalState.isConnected = true

    router.replace('/')
  }
  catch (error) {
    console.error('Connection failed:', error)
    connectionStatus.value = {
      type: 'error',
      message: `Connection failed: ${error}`,
    }
  }
  finally {
    isConnecting.value = false
  }
}
</script>
