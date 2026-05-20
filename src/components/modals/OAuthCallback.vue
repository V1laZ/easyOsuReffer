<template>
  <div class="fixed inset-safe z-1000 flex flex-col items-center justify-center bg-slate-950/95 px-6 backdrop-blur">
    <IconBtn
      icon="close"
      size="sm"
      class="absolute right-3 top-3"
      @click="modalsState.showOAuthCallback = false"
    />
    <div class="w-full max-w-sm space-y-4">
      <LoadingText
        class="text-center text-sm text-slate-300"
        text="Waiting for OAuth callback"
      />
      <Input
        v-model="manualConnectionString"
        placeholder="Paste token string here"
      />
      <Btn
        block
        @click="submitManualConnection"
      >
        Connect manually
      </Btn>
    </div>
  </div>
</template>

<script setup lang="ts">
import { onMounted, ref } from 'vue'
import { once } from '@tauri-apps/api/event'
import { dbService } from '@/services/database'
import { globalState, modalsState } from '@/stores/global'
import LoadingText from '@/components/UI/LoadingText.vue'
import IconBtn from '@/components/UI/IconBtn.vue'
import Btn from '@/components/UI/Btn.vue'
import Input from '@/components/UI/Input.vue'
import { OauthTokenCallback } from '@/types'

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

async function handleTokenData(data: OauthTokenCallback) {
  if (!data || !data.access_token || !data.refresh_token) {
    console.error('Invalid OAuth token data received', data)
    return
  }

  if (!globalState.user) {
    console.error('Cannot save OAuth token: no IRC user is set. Reconnect to Bancho and try again.')
    return
  }

  try {
    await dbService.saveOAuthToken(
      globalState.user,
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
