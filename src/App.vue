<template>
  <main class="h-screen overflow-hidden">
    <RouterView />
  </main>
</template>

<script setup lang="ts">
import { onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { RouterView, useRouter } from 'vue-router'
import { dbService } from './services/database'

const router = useRouter()

onMounted(async () => {
  try {
    await dbService.init()
  } catch (error) {
    console.error('Failed to initialize database:', error)
    return
  }

  const saved = await dbService.getCredentials()
  if (saved) {
    try {
      const config = {
        username: saved.username,
        password: saved.password
      }
      await invoke('connect_to_bancho', { config })
      return
    } catch (error) {
      console.error('Failed to connect with saved credentials:', error)
    }
  }

  router.push('/login')
})
</script>