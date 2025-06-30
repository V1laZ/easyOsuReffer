<template>
  <div class="fixed inset-0 bg-black bg-opacity-50 backdrop-blur-sm flex items-center justify-center p-4 z-50" @click="$emit('close')">
    <div class="bg-gray-800 rounded-2xl p-6 w-full max-w-md border border-gray-700 shadow-2xl" @click.stop>
      <!-- Header -->
      <div class="flex items-center justify-between mb-6">
        <h2 class="text-xl font-semibold text-white">Settings</h2>
        <button 
          @click="$emit('close')"
          class="p-2 rounded-lg hover:bg-gray-700 transition-colors"
        >
          <svg class="w-5 h-5 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
          </svg>
        </button>
      </div>

      <!-- Settings Content -->
      <div class="space-y-6">
        <!-- Account Section -->
        <div>
          <h3 class="text-lg font-medium text-white mb-4">Account</h3>
          <div class="space-y-4">
            <!-- Current User Info -->
            <div class="p-4 bg-gray-700 rounded-lg">
              <div class="flex items-center space-x-3">
                <div class="w-12 h-12 bg-gradient-to-br from-pink-500 to-purple-600 rounded-full flex items-center justify-center">
                  <span class="text-lg font-medium text-white">
                    {{ currentUser?.charAt(0).toUpperCase() || '?' }}
                  </span>
                </div>
                <div>
                  <div class="font-medium text-white">{{ currentUser || 'Not logged in' }}</div>
                  <div class="text-sm text-gray-400">Connected to osu! Bancho</div>
                </div>
              </div>
            </div>

            <!-- Account Actions -->
            <div class="space-y-2">
              <button
                @click="handleLogout"
                class="w-full flex items-center justify-center space-x-2 px-4 py-3 bg-red-600 hover:bg-red-700 text-white rounded-lg transition-colors"
              >
                <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 16l4-4m0 0l-4-4m4 4H7m6 4v1a3 3 0 01-3 3H6a3 3 0 01-3-3V7a3 3 0 013-3h4a3 3 0 013 3v1" />
                </svg>
                <span>Logout</span>
              </button>

              <button
                @click="handleClearCredentials"
                class="w-full flex items-center justify-center space-x-2 px-4 py-3 bg-gray-600 hover:bg-gray-700 text-white rounded-lg transition-colors"
              >
                <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
                </svg>
                <span>Clear Saved Credentials</span>
              </button>
            </div>
          </div>
        </div>

        <!-- Appearance Section -->
        <div>
          <h3 class="text-lg font-medium text-white mb-4">Appearance</h3>
          <div class="space-y-4">
            <!-- Theme Selection -->
            <div>
              <label class="block text-sm font-medium text-gray-300 mb-2">
                Theme
              </label>
              <select 
                v-model="settings.theme"
                class="w-full px-3 py-2 bg-gray-700 border border-gray-600 rounded-lg text-white focus:outline-none focus:ring-2 focus:ring-pink-500 focus:border-transparent"
              >
                <option value="dark">Dark</option>
                <option value="light" disabled>Light (Coming Soon)</option>
                <option value="osu" disabled>osu! Theme (Coming Soon)</option>
              </select>
            </div>

            <!-- Font Size -->
            <div>
              <label class="block text-sm font-medium text-gray-300 mb-2">
                Font Size
              </label>
              <div class="flex items-center space-x-3">
                <span class="text-sm text-gray-400">Small</span>
                <input
                  v-model="settings.fontSize"
                  type="range"
                  min="12"
                  max="18"
                  step="1"
                  class="flex-1 h-2 bg-gray-700 rounded-lg appearance-none cursor-pointer"
                />
                <span class="text-sm text-gray-400">Large</span>
              </div>
              <div class="text-xs text-gray-400 mt-1">{{ settings.fontSize }}px</div>
            </div>

            <!-- Show Timestamps -->
            <div class="flex items-center justify-between">
              <label class="text-sm font-medium text-gray-300">
                Show Message Timestamps
              </label>
              <input
                v-model="settings.showTimestamps"
                type="checkbox"
                class="w-4 h-4 text-pink-500 bg-gray-700 border-gray-600 rounded focus:ring-pink-500 focus:ring-2"
              />
            </div>

            <!-- Compact Mode -->
            <div class="flex items-center justify-between">
              <label class="text-sm font-medium text-gray-300">
                Compact Message View
              </label>
              <input
                v-model="settings.compactMode"
                type="checkbox"
                class="w-4 h-4 text-pink-500 bg-gray-700 border-gray-600 rounded focus:ring-pink-500 focus:ring-2"
              />
            </div>
          </div>
        </div>

        <!-- Notifications Section -->
        <div>
          <h3 class="text-lg font-medium text-white mb-4">Notifications</h3>
          <div class="space-y-4">
            <!-- Sound Notifications -->
            <div class="flex items-center justify-between">
              <label class="text-sm font-medium text-gray-300">
                Sound Notifications
              </label>
              <input
                v-model="settings.soundNotifications"
                type="checkbox"
                class="w-4 h-4 text-pink-500 bg-gray-700 border-gray-600 rounded focus:ring-pink-500 focus:ring-2"
              />
            </div>

            <!-- Desktop Notifications -->
            <div class="flex items-center justify-between">
              <label class="text-sm font-medium text-gray-300">
                Desktop Notifications
              </label>
              <input
                v-model="settings.desktopNotifications"
                type="checkbox"
                class="w-4 h-4 text-pink-500 bg-gray-700 border-gray-600 rounded focus:ring-pink-500 focus:ring-2"
              />
            </div>

            <!-- Mention Notifications -->
            <div class="flex items-center justify-between">
              <label class="text-sm font-medium text-gray-300">
                Notify on @mentions
              </label>
              <input
                v-model="settings.mentionNotifications"
                type="checkbox"
                class="w-4 h-4 text-pink-500 bg-gray-700 border-gray-600 rounded focus:ring-pink-500 focus:ring-2"
              />
            </div>
          </div>
        </div>
      </div>

      <!-- Footer -->
      <div class="mt-8 pt-6 border-t border-gray-700">
        <div class="flex items-center justify-between text-sm text-gray-400">
          <span>easyOsuReffer v1.0.0</span>
          <button
            @click="resetSettings"
            class="text-pink-400 hover:text-pink-300 transition-colors"
          >
            Reset to defaults
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive } from 'vue'
import { dbService } from '../../services/database'

interface Settings {
  theme: 'dark' | 'light' | 'osu'
  fontSize: number
  showTimestamps: boolean
  compactMode: boolean
  soundNotifications: boolean
  desktopNotifications: boolean
  mentionNotifications: boolean
}

interface Emits {
  (e: 'close'): void
  (e: 'logout'): void
}

const emit = defineEmits<Emits>()

const currentUser = ref<string | null>('TestUser') // TODO: Get from actual state

const settings = reactive<Settings>({
  theme: 'dark',
  fontSize: 14,
  showTimestamps: true,
  compactMode: false,
  soundNotifications: true,
  desktopNotifications: false,
  mentionNotifications: true,
})

const handleLogout = () => {
  emit('logout')
}

const handleClearCredentials = async () => {
  if (confirm('Are you sure you want to clear saved credentials? You will need to login again next time.')) {
    try {
      await dbService.deleteCredentials()
      alert('Saved credentials cleared successfully')
    } catch (error) {
      console.error('Failed to clear credentials:', error)
      alert('Failed to clear credentials')
    }
  }
}

const resetSettings = () => {
  if (confirm('Reset all settings to default values?')) {
    settings.theme = 'dark'
    settings.fontSize = 14
    settings.showTimestamps = true
    settings.compactMode = false
    settings.soundNotifications = true
    settings.desktopNotifications = false
    settings.mentionNotifications = true
  }
}

// TODO: Save settings to local storage or database
// TODO: Apply settings to the application
</script>
