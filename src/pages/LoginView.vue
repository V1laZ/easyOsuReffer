<template>
  <div class="bg-gradient-to-br h-screen from-gray-900 via-purple-900 to-pink-900 flex items-center justify-center px-4 py-8">
    <div class="w-full max-w-md">
      <!-- App Logo/Title -->
      <div class="text-center mb-8">
        <div class="inline-flex items-center justify-center w-20 h-20 bg-pink-500 rounded-full mb-4 shadow-lg">
          <svg class="w-10 h-10 text-white" fill="currentColor" viewBox="0 0 24 24">
            <path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm-2 15l-5-5 1.41-1.41L10 14.17l7.59-7.59L19 8l-9 9z"/>
          </svg>
        </div>
        <h1 class="text-3xl font-bold text-white mb-2">osu! Refferer</h1>
        <p class="text-pink-200 text-sm">Ref osu! matches or just chat with others!</p>
      </div>

      <!-- Login Form -->
      <div class="bg-gray-800/80 backdrop-blur-sm rounded-2xl p-6 shadow-2xl border border-pink-500/20">
        <form @submit.prevent="handleLogin" class="space-y-6">
          <!-- Username Field -->
          <div class="space-y-2">
            <label for="username" class="block text-sm font-medium text-pink-200">
              osu! Username
            </label>
            <div class="relative">
              <input
                id="username"
                v-model="loginForm.username"
                type="text"
                required
                class="w-full px-4 py-3 bg-gray-700/50 border border-gray-600 rounded-xl text-white placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-pink-500 focus:border-transparent transition-all duration-200"
                placeholder="Enter your osu! username"
                :disabled="isConnecting"
              />
              <div class="absolute inset-y-0 right-0 flex items-center pr-3">
                <svg class="w-5 h-5 text-gray-400" fill="currentColor" viewBox="0 0 24 24">
                  <path d="M12 12c2.21 0 4-1.79 4-4s-1.79-4-4-4-4 1.79-4 4 1.79 4 4 4zm0 2c-2.67 0-8 1.34-8 4v2h16v-2c0-2.66-5.33-4-8-4z"/>
                </svg>
              </div>
            </div>
          </div>

          <!-- Password Field -->
          <div class="space-y-2">
            <label for="password" class="block text-sm font-medium text-pink-200">
              IRC Password
            </label>
            <div class="relative">
              <input
                id="password"
                v-model="loginForm.password"
                :type="showPassword ? 'text' : 'password'"
                required
                class="w-full px-4 py-3 bg-gray-700/50 border border-gray-600 rounded-xl text-white placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-pink-500 focus:border-transparent transition-all duration-200"
                placeholder="Enter your IRC password"
                :disabled="isConnecting"
              />
              <button
                type="button"
                @click="showPassword = !showPassword"
                class="absolute inset-y-0 right-0 flex items-center pr-3"
                :disabled="isConnecting"
              >
                <svg v-if="showPassword" class="w-5 h-5 text-gray-400 hover:text-pink-400 transition-colors" fill="currentColor" viewBox="0 0 24 24">
                  <path d="M12 7c2.76 0 5 2.24 5 5 0 .65-.13 1.26-.36 1.83l2.92 2.92c1.51-1.26 2.7-2.89 3.43-4.75-1.73-4.39-6-7.5-11-7.5-1.4 0-2.74.25-3.98.7l2.16 2.16C10.74 7.13 11.35 7 12 7zM2 4.27l2.28 2.28.46.46C3.08 8.3 1.78 10.02 1 12c1.73 4.39 6 7.5 11 7.5 1.55 0 3.03-.3 4.38-.84l.42.42L19.73 22 21 20.73 3.27 3 2 4.27zM7.53 9.8l1.55 1.55c-.05.21-.08.43-.08.65 0 1.66 1.34 3 3 3 .22 0 .44-.03.65-.08l1.55 1.55c-.67.33-1.41.53-2.2.53-2.76 0-5-2.24-5-5 0-.79.2-1.53.53-2.2zm4.31-.78l3.15 3.15.02-.16c0-1.66-1.34-3-3-3l-.17.01z"/>
                </svg>
                <svg v-else class="w-5 h-5 text-gray-400 hover:text-pink-400 transition-colors" fill="currentColor" viewBox="0 0 24 24">
                  <path d="M12 4.5C7 4.5 2.73 7.61 1 12c1.73 4.39 6 7.5 11 7.5s9.27-3.11 11-7.5c-1.73-4.39-6-7.5-11-7.5zM12 17c-2.76 0-5-2.24-5-5s2.24-5 5-5 5 2.24 5 5-2.24 5-5 5zm0-8c-1.66 0-3 1.34-3 3s1.34 3 3 3 3-1.34 3-3-1.34-3-3-3z"/>
                </svg>
              </button>
            </div>
          </div>

          <!-- Remember Me -->
          <div class="flex items-center">
            <input
              id="remember"
              v-model="loginForm.rememberMe"
              type="checkbox"
              class="w-4 h-4 text-pink-500 bg-gray-700 border-gray-600 rounded focus:ring-pink-500 focus:ring-2"
              :disabled="isConnecting"
            />
            <label for="remember" class="ml-2 text-sm text-pink-200">
              Remember my credentials
            </label>
          </div>

          <!-- Login Button -->
          <button
            type="submit"
            :disabled="isConnecting || !loginForm.username.trim() || !loginForm.password.trim()"
            class="w-full bg-gradient-to-r from-pink-500 to-purple-600 hover:from-pink-600 hover:to-purple-700 disabled:from-gray-600 disabled:to-gray-700 text-white font-medium py-3 px-4 rounded-xl transition-all duration-200 transform hover:scale-[1.02] disabled:hover:scale-100 disabled:cursor-not-allowed flex items-center justify-center space-x-2"
          >
            <svg v-if="isConnecting" class="animate-spin -ml-1 mr-3 h-5 w-5 text-white" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
              <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
              <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
            </svg>
            <span>{{ isConnecting ? 'Connecting...' : 'Connect to Bancho' }}</span>
          </button>
        </form>

        <!-- Connection Status -->
        <div v-if="connectionStatus" class="mt-4 p-3 rounded-lg" :class="connectionStatus.type === 'error' ? 'bg-red-900/50 border border-red-500/50' : 'bg-green-900/50 border border-green-500/50'">
          <div class="flex items-center">
            <svg v-if="connectionStatus.type === 'error'" class="w-5 h-5 text-red-400 mr-2" fill="currentColor" viewBox="0 0 24 24">
              <path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm-2 15l-5-5 1.41-1.41L10 14.17l7.59-7.59L19 8l-9 9z"/>
            </svg>
            <svg v-else class="w-5 h-5 text-green-400 mr-2" fill="currentColor" viewBox="0 0 24 24">
              <path d="M9 16.17L4.83 12l-1.42 1.41L9 19 21 7l-1.41-1.41z"/>
            </svg>
            <span class="text-sm" :class="connectionStatus.type === 'error' ? 'text-red-300' : 'text-green-300'">
              {{ connectionStatus.message }}
            </span>
          </div>
        </div>

        <!-- Help Links -->
        <div class="mt-6 pt-4 border-t border-gray-600">
          <div class="text-center space-y-2">
            <a role="button" href="#" @click.prevent="showHelp = true" class="text-pink-400 hover:text-pink-300 text-sm transition-colors">
              How to get IRC password?
            </a>
          </div>
        </div>
      </div>
    </div>

    <!-- Help Modal -->
    <div v-if="showHelp" class="fixed inset-0 bg-black/50 backdrop-blur-sm flex items-center justify-center p-4 z-50" @click="showHelp = false">
      <div class="bg-gray-800 rounded-2xl p-6 max-w-md w-full border border-pink-500/20" @click.stop>
        <h3 class="text-lg font-semibold text-white mb-4">Getting your IRC Password</h3>
        <div class="space-y-3 text-sm text-gray-300">
          <p>1. Go to your <strong class="text-pink-400">osu! account settings</strong></p>
          <p>2. Scroll down to the <strong class="text-pink-400">Legacy API</strong> section</p>
          <p>3. Copy the <strong class="text-pink-400">irc server password</strong></p>
          <p>4. Use this password in the IRC Password field</p>
        </div>
        <button @click="showHelp = false" class="mt-4 w-full bg-pink-500 hover:bg-pink-600 text-white py-2 rounded-xl transition-colors">
          Got it!
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { useRouter } from 'vue-router'
import { invoke } from '@tauri-apps/api/core'
import { dbService } from '../services/database'
import { globalState } from '../stores/global'

const router = useRouter()

// Form state
const loginForm = ref({
  username: '',
  password: '',
  rememberMe: false
})

// UI state
const showPassword = ref(false)
const isConnecting = ref(false)
const connectionStatus = ref<ConnectionStatus | null>(null)
const showHelp = ref(false)

const handleLogin = async () => {
  if (!loginForm.value.username.trim() || !loginForm.value.password.trim()) {
    connectionStatus.value = {
      type: 'error',
      message: 'Please fill in all fields'
    }
    return
  }

  isConnecting.value = true
  connectionStatus.value = null

  try {
    const config = {
      username: loginForm.value.username.trim(),
      password: loginForm.value.password.trim()
    }

    await invoke('connect_to_bancho', { config })
    
    // Save credentials if remember me is checked
    if (loginForm.value.rememberMe) {
      await dbService.saveCredentials(config.username, config.password)
    } else {
      await dbService.deleteCredentials()
    }

    globalState.user = config.username
    globalState.isConnected = true

    router.push('/')
  } catch (error) {
    console.error('Connection failed:', error)
    connectionStatus.value = {
      type: 'error',
      message: `Connection failed: ${error}`
    }
  } finally {
    isConnecting.value = false
  }
}
</script>