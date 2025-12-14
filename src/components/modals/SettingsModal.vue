<template>
  <div
    class="fixed inset-0 bg-black/80 flex items-center justify-center p-4 z-50"
    @click="$emit('close')"
  >
    <div
      class="bg-gray-800 rounded-2xl p-6 w-full max-w-md border border-gray-700 shadow-2xl"
      @click.stop
    >
      <!-- Header -->
      <div class="flex items-center justify-between mb-6">
        <h2 class="text-xl font-semibold text-white">
          Settings
        </h2>
        <button
          class="p-2 rounded-lg hover:bg-gray-700 transition-colors"
          @click="emit('close')"
        >
          <svg
            class="w-5 h-5 text-gray-400"
            fill="none"
            stroke="currentColor"
            viewBox="0 0 24 24"
          >
            <path
              stroke-linecap="round"
              stroke-linejoin="round"
              stroke-width="2"
              d="M6 18L18 6M6 6l12 12"
            />
          </svg>
        </button>
      </div>

      <!-- Settings Content -->
      <div class="space-y-6">
        <!-- Account Section -->
        <div>
          <h3 class="text-lg font-medium text-white mb-4">
            Account
          </h3>
          <div class="space-y-2">
            <!-- Current User Info -->
            <div class="p-4 bg-gray-700 rounded-lg">
              <div class="flex items-center space-x-3">
                <div class="w-12 h-12 bg-gradient-to-br from-pink-500 to-purple-600 rounded-full flex items-center justify-center">
                  <span class="text-lg font-medium text-white">
                    {{ globalState.user?.charAt(0).toUpperCase() || '?' }}
                  </span>
                </div>
                <div>
                  <div class="font-medium text-white">
                    {{ globalState.user || 'Not logged in' }}
                  </div>
                  <div class="flex items-center space-x-2">
                    <span
                      class="rounded-full size-2"
                      :class="{
                        'bg-green-500': globalState.isConnected,
                        'bg-red-500': !globalState.isConnected
                      }"
                    />
                    <div class="text-sm text-gray-400">
                      {{ globalState.isConnected ? "Connected to Bancho" : "Offline" }}
                    </div>
                  </div>

                  <div class="flex items-center space-x-2">
                    <span
                      class="rounded-full size-2"
                      :class="{
                        'bg-green-500': globalState.isConnectedOsu,
                        'bg-red-500': !globalState.isConnectedOsu
                      }"
                    />
                    <div
                      class="text-sm text-gray-400"
                      :class="{
                        'cursor-pointer hover:text-red-500': globalState.isConnectedOsu
                      }"
                      @click="removeOsuConnect"
                    >
                      {{ globalState.isConnectedOsu ? "osu! Account Connected" : "osu! Account Not Connected" }}
                    </div>
                  </div>
                </div>
              </div>
            </div>

            <!-- Account Actions -->
            <div class="space-y-2">
              <ConnectOsuBtn v-if="!globalState.isConnectedOsu" />

              <button
                class="w-full flex items-center justify-center space-x-2 px-4 py-3 bg-red-600 hover:bg-red-700 text-white rounded-lg transition-colors"
                @click="emit('logout')"
              >
                <svg
                  class="w-5 h-5"
                  fill="none"
                  stroke="currentColor"
                  viewBox="0 0 24 24"
                >
                  <path
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    stroke-width="2"
                    d="M17 16l4-4m0 0l-4-4m4 4H7m6 4v1a3 3 0 01-3 3H6a3 3 0 01-3-3V7a3 3 0 013-3h4a3 3 0 013 3v1"
                  />
                </svg>
                <span>Logout</span>
              </button>
            </div>
          </div>
        </div>
      </div>

      <!-- Footer -->
      <div class="mt-8 pt-6 border-t border-gray-700">
        <div class="flex items-center justify-between text-sm text-gray-400">
          <span>osu!Reffer v{{ appVersion }}</span>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { getVersion } from '@tauri-apps/api/app'
import { globalState } from '@/stores/global'
import ConnectOsuBtn from '../ConnectOsuBtn.vue'
import { dbService } from '@/services/database'

const emit = defineEmits<{
  close: []
  logout: []
}>()

const appVersion = ref('')

onMounted(async () => {
  try {
    appVersion.value = await getVersion()
  }
  catch (error) {
    console.error('Failed to get app version:', error)
  }
})

const removeOsuConnect = () => {
  if (!globalState.user || !globalState.isConnectedOsu) return
  try {
    if (confirm('Are you sure you want to disconnect your osu! account?')) {
      dbService.deleteOauthToken(globalState.user)
      globalState.isConnectedOsu = false
    }
  }
  catch (error) {
    console.error('Error removing osu! connection:', error)
  }
}
</script>
