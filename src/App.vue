<template>
  <div>
    <main class="h-[100dvh] overflow-hidden">
      <div v-if="loading" class="h-full bg-gray-900 flex items-center justify-center">
        <div class="text-center">
          <svg class="animate-spin w-12 h-12 mx-auto text-pink-500 mb-4" fill="none" viewBox="0 0 24 24">
            <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
            <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
          </svg>
          
          <h2 class="text-xl text-white mb-2">osu! Reffer</h2>
          <p v-if="errorMessage" class="text-red-400 mb-2">{{ errorMessage }}</p>
          <p class="text-gray-400">{{ loadingMessage }}</p>
        </div>
      </div>
  
      <div v-else-if="disconnected" class="h-full bg-gray-900 flex items-center justify-center">
        <div class="text-center">
          <h2 class="text-xl text-white mb-2">Disconnected from Bancho</h2>
          <p class="text-gray-400 mb-4">You have been disconnected from Bancho. Please check your connection and try to reconnect.</p>
          <button @click="reconnectToBancho" class="px-4 py-2 bg-pink-500 text-white rounded hover:bg-pink-600">Reconnect</button>
          <p v-if="errorMessage" class="text-red-400 mt-2">{{ errorMessage }}</p>
        </div>
      </div>
  
      <RouterView v-else />
    </main>

    <Modals />
  </div>
</template>

<script setup lang="ts">
import { onMounted, onUnmounted, ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { RouterView, useRouter } from 'vue-router'
import { dbService } from './services/database'
import { globalState } from './stores/global'
import { type UnlistenFn, listen } from '@tauri-apps/api/event'
import Modals from './components/modals/Modals.vue'

const router = useRouter()

const loading = ref(true)
const disconnected = ref(false)
const loadingMessage = ref('Loading...')
const errorMessage = ref('')

let unlisteDisconnect: UnlistenFn | null = null

async function connectWithCredentials(saved: UserCredentials) {
  globalState.user = saved.username
  globalState.userId = saved.id
  globalState.isConnectedOsu = await dbService.getOsuConnectedStatus(saved.username)
  try {
    loadingMessage.value = 'Connecting...'
    errorMessage.value = ''
    const connected = await invoke<boolean>('get_connection_status')
    if (connected) {
      globalState.isConnected = true
      loading.value = false
      disconnected.value = false
      router.replace('/')
      return true
    }
    const config = {
      username: saved.username,
      password: saved.password
    }
    await invoke('connect_to_bancho', { config })
    globalState.isConnected = true
    loading.value = false
    disconnected.value = false
    router.replace('/')
    return true
  } catch (error) {
    errorMessage.value = 'Failed to connect with saved credentials.' + (error instanceof Error ? ' ' + error.message : error ? ' ' + String(error) : '')
    console.error('Failed to connect with saved credentials:', error)
    return false
  }
}

async function reconnectToBancho() {
  errorMessage.value = ''
  try {
    loading.value = true
    loadingMessage.value = 'Reconnecting to Bancho...'
    await invoke('reconnect_to_bancho')
    globalState.isConnected = true
    disconnected.value = false
    loading.value = false
    router.replace('/')
  } catch (e) {
    errorMessage.value = 'Failed to reconnect to Bancho. Please try to restart the app.'
  }
}

function handleOfflineState() {
  globalState.isConnected = false
  loading.value = false
  disconnected.value = true
  loadingMessage.value = ''
  errorMessage.value = ''
}

function setRealVh() {
  document.documentElement.style.setProperty('--real-vh', `${window.innerHeight * 0.01}px`);
}

onMounted(async () => {
  unlisteDisconnect = await listen('irc-disconnected', handleOfflineState)

  window.addEventListener('resize', setRealVh);
  setRealVh();

  loading.value = true
  try {
    loadingMessage.value = 'Initializing...'
    errorMessage.value = ''
    await dbService.init()
    loadingMessage.value = 'Checking credentials...'
    const saved = await dbService.getCredentials()
    if (saved) {
      await connectWithCredentials(saved)
    } else {
      loading.value = false
      router.replace('/login')
    }
  } catch (error) {
    errorMessage.value = 'Failed to initialize database.' + (error instanceof Error ? ' ' + error.message : error ? ' ' + String(error) : '')
    console.error('Failed to initialize database:', error)
    return
  }
})

onUnmounted(() => {
  if (unlisteDisconnect) unlisteDisconnect();
  window.removeEventListener('resize', setRealVh);
})
</script>

<style>
main {
  height: calc(var(--real-vh, 1vh) * 100);
}

::-webkit-scrollbar {
  display: none;
}

.select-arrow {
  pointer-events: none;
  position: absolute;
  right: 1rem;
  top: 50%;
  transform: translateY(-50%);
  width: 0.8em;
  height: 0.5em;
  background: none;
  display: inline-block;
}

.select-arrow::after {
  content: "";
  display: block;
  width: 100%;
  height: 100%;
  background-color: #fff;
  clip-path: polygon(100% 0%, 0 0%, 50% 100%);
  opacity: 0.6;
}
</style>