<template>
  <div
    class="flex fixed top-0 left-0 z-50 w-full bg-black flex-col items-center justify-center h-screen space-y-4"
  >
    <div class="absolute top-3 right-3">
      <button
        class="text-white hover:text-gray-400 transition-colors"
        @click="modalsState.showOAuthCallback = false"
      >
        <svg
          xmlns="http://www.w3.org/2000/svg"
          width="24"
          height="24"
          viewBox="0 0 24 24"
        ><path
          fill="currentColor"
          d="M19.41 6.58L17.83 5l-7.59 7.59L2.83 5 1.41 6.41l7.59 7.59L1.41 21l1.42 1.41L12 14.83l7.59 7.58L21 21l-7.59-7.59z"
        /></svg>
      </button>
    </div>
    <div class="p-4">
      <LoadingText
        class="mb-4 text-center text-gray-200"
        text="Waiting for OAuth callback"
      />
      <input
        v-model="manualConnectionString"
        type="text"
        placeholder="Paste token string here"
        class="w-full bg-white px-4 py-2 border rounded-lg"
      >
      <button
        class="w-full px-4 py-2 bg-pink-600 text-white rounded-lg mt-2 hover:bg-pink-700 transition-colors"
        @click="submitManualConnection"
      >
        Connect Manually
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { onMounted, ref } from 'vue'
import { once } from '@tauri-apps/api/event'
import { dbService } from '../../services/database'
import { globalState, modalsState } from '../../stores/global'
import LoadingText from '../UI/LoadingText.vue'

const manualConnectionString = ref('')

const submitManualConnection = async () => {
  if (!manualConnectionString.value.trim()) {
    console.error('No manual connection string provided')
    return
  }

  try {
    const decoded = atob(manualConnectionString.value.trim())
    const tokenData = JSON.parse(decoded)

    await handleTokenData(tokenData)
  }
  catch (error) {
    console.error('Failed to process manual connection string:', error)
  }
}

onMounted(() => {
  once('oauth-token-callback', handleOAuthTokenCallback)
})

async function handleOAuthTokenCallback(payload: { payload: OauthTokenCallback }) {
  const data = payload.payload
  await handleTokenData(data)
}

async function handleTokenData(data: any) {
  if (!data) {
    console.error('No OAuth token data received')
    return
  }

  try {
    await dbService.saveOAuthToken(
      globalState.user || '',
      data.access_token,
      data.refresh_token,
      data.expires_in,
    )
    globalState.isConnectedOsu = true
    modalsState.showOAuthCallback = false
  }
  catch (error) {
    console.error('Failed to save OAuth token:', error)
  }
}
</script>
