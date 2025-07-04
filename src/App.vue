<template>
  <main class="h-[100dvh] overflow-hidden">
    <div v-if="loading" class="h-full bg-gray-900 flex items-center justify-center">
      <div class="text-center">
        <svg class="animate-spin w-12 h-12 mx-auto text-pink-500 mb-4" fill="none" viewBox="0 0 24 24">
          <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
          <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
        </svg>
        
        <h2 class="text-xl text-white mb-2">osu! Reffer</h2>
        <p class="text-gray-400">{{ loadingMessage }}</p>
        
        <p v-if="errorMessage" class="text-red-400 mt-4">{{ errorMessage }}</p>
      </div>
    </div>
    
    <RouterView v-else />
  </main>
</template>

<script setup lang="ts">
import { onMounted, ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { RouterView, useRouter } from 'vue-router'
import { dbService } from './services/database'
import { globalState } from './stores/global'
import { once } from '@tauri-apps/api/event'

const router = useRouter()

const loading = ref(true)
const loadingMessage = ref('Loading...')
const errorMessage = ref('')

onMounted(async () => {
  once('oauth-token-callback', async (payload) => {
    const data = payload.payload as { access_token: string, refresh_token: string, expires_in: number }
    if (!data) {
      errorMessage.value = 'Failed to retrieve OAuth token'
      loading.value = false
      return
    }

    try {
      await dbService.saveOAuthToken(
        Number(globalState.userId),
        data.access_token,
        data.refresh_token,
        data.expires_in
      )
      globalState.isConnectedOsu = true
    } catch (error) {
      console.error('Failed to save OAuth token:', error)
      errorMessage.value = 'Failed to save OAuth token'
    }
  })

  try {
    loadingMessage.value = 'Initializing...'
    await dbService.init()
  } catch (error) {
    console.error('Failed to initialize database:', error)
    errorMessage.value = 'Failed to initialize database'
    return
  }

  loadingMessage.value = 'Checking credentials...'
  const saved = await dbService.getCredentials()
  
  if (saved) {
    globalState.user = saved.username
    globalState.userId = saved.id
    globalState.isConnectedOsu = await dbService.getOsuConnectedStatus(saved.id)
    try {
      loadingMessage.value = 'Connecting...'

      const connected = await invoke('get_connection_status') as boolean
      if (connected) {
        globalState.isConnected = true
        loading.value = false
        router.push('/')
        return
      }

      const config = {
        username: saved.username,
        password: saved.password
      }
      await invoke('connect_to_bancho', { config })
      globalState.isConnected = true
      loading.value = false
      router.push('/')
      return
    } catch (error) {
      console.error('Failed to connect with saved credentials:', error)
      errorMessage.value = 'Connection failed'
    }
  }

  loading.value = false
  router.push('/login')
})
</script>