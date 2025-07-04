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
        <h2 class="text-xl font-semibold text-white">Settings</h2>
        <button 
          @click="emit('close')"
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
                  <div class="font-medium text-white">{{ globalState.user || 'Not logged in' }}</div>
                  <div class="flex items-center space-x-2">
                    <span 
                      class="rounded-full size-2" 
                      :class="{
                        'bg-green-500': globalState.isConnected,
                        'bg-red-500': !globalState.isConnected
                    }"></span>
                    <div class="text-sm text-gray-400">{{ globalState.isConnected ? "Connected to Bancho" : "Offline" }}</div>
                  </div>

                  <div class="flex items-center space-x-2">
                    <span 
                      class="rounded-full size-2" 
                      :class="{
                        'bg-green-500': globalState.isConnectedOsu,
                        'bg-red-500': !globalState.isConnectedOsu
                    }"></span>
                    <div class="text-sm text-gray-400">{{ globalState.isConnectedOsu ? "osu! Account Connected" : "osu! Account Not Connected" }}</div>
                  </div>
                </div>
              </div>
            </div>

            <!-- Account Actions -->
            <div class="space-y-2">
              <button
                v-if="!globalState.isConnectedOsu"
                @click="open('https://osureffer.vilaz.dev/login')"
                class="w-full flex items-center justify-center space-x-2 px-4 py-3 bg-pink-600 hover:bg-pink-700 text-white rounded-lg transition-colors"
                >
                <svg class="size-6" xmlns="http://www.w3.org/2000/svg" width="32" height="32" viewBox="0 0 24 24"><!-- Icon from Simple Icons by Simple Icons Collaborators - https://github.com/simple-icons/simple-icons/blob/develop/LICENSE.md --><path fill="currentColor" d="M7.698 10.362a1.94 1.94 0 0 0-.7-.516q-.421-.189-.988-.189c-.567 0-.704.063-.982.189s-.51.298-.692.516q-.273.328-.413.777q-.139.448-.139.96q0 .511.14.952q.139.44.412.767q.274.329.692.512t.982.184c.565 0 .707-.062.988-.184q.422-.184.7-.512q.279-.327.413-.767q.135-.44.135-.952a3.3 3.3 0 0 0-.135-.96a2.1 2.1 0 0 0-.413-.777m-.965 2.81q-.22.372-.723.372q-.494 0-.713-.372q-.22-.373-.22-1.073c0-.7.073-.824.22-1.073q.22-.372.713-.372q.503 0 .723.372q.22.373.22 1.073t-.22 1.073m11.89-.83l-.09-4.39a4.5 4.5 0 0 1 .69-.054q.351 0 .701.054l-.09 4.39q-.315.053-.601.053a3.5 3.5 0 0 1-.61-.054m1.319 1.4q0 .332-.054.664a4 4 0 0 1-.655.054a4 4 0 0 1-.664-.054a4 4 0 0 1-.054-.655q0-.323.054-.665a4 4 0 0 1 .655-.054q.323 0 .664.054q.054.341.054.656m-3.223-4.03q.315 0 .638.053v4.461q-.288.099-.759.193a5.3 5.3 0 0 1-1.863.023a1.7 1.7 0 0 1-.74-.305q-.32-.234-.507-.683q-.189-.449-.189-1.193V9.765a4 4 0 0 1 .638-.054q.313 0 .637.054v2.46q0 .367.058.606a.9.9 0 0 0 .18.377a.66.66 0 0 0 .3.197q.18.058.422.058q.332 0 .557-.062V9.765a4 4 0 0 1 .628-.054m-4.362 2.683q.08.225.08.548a1.4 1.4 0 0 1-.542 1.117q-.265.212-.642.333q-.378.12-.853.12a5 5 0 0 1-.395-.013a3 3 0 0 1-.346-.045a4 4 0 0 1-.327-.076a4 4 0 0 1-.35-.116a2.6 2.6 0 0 1 .085-.49a3 3 0 0 1 .175-.48q.296.117.561.175q.265.06.552.059q.126 0 .274-.023a1 1 0 0 0 .274-.08a.7.7 0 0 0 .21-.153a.35.35 0 0 0 .086-.247q0-.216-.13-.31a1.3 1.3 0 0 0-.364-.166l-.556-.162q-.503-.143-.786-.426q-.282-.283-.283-.848q0-.682.49-1.068q.489-.386 1.332-.386q.35 0 .692.062q.341.063.691.189a2.5 2.5 0 0 1-.09.485a2.3 2.3 0 0 1-.17.44a4 4 0 0 0-.476-.158a2.2 2.2 0 0 0-.548-.067q-.305 0-.476.094a.32.32 0 0 0-.17.301q0 .197.121.278t.346.153l.511.153q.252.072.454.175t.345.255t.225.377M12 0C5.373 0 0 5.373 0 12s5.373 12 12 12c6.628 0 12-5.373 12-12S18.628 0 12 0m0 22.8C6.035 22.8 1.2 17.965 1.2 12S6.035 1.2 12 1.2S22.8 6.035 22.8 12S17.965 22.8 12 22.8"/></svg>
                <span>Connect osu! Account</span>
              </button>

              <button
                @click="emit('logout')"
                class="w-full flex items-center justify-center space-x-2 px-4 py-3 bg-red-600 hover:bg-red-700 text-white rounded-lg transition-colors"
              >
                <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 16l4-4m0 0l-4-4m4 4H7m6 4v1a3 3 0 01-3 3H6a3 3 0 01-3-3V7a3 3 0 013-3h4a3 3 0 013 3v1" />
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
          <span>osu! Reffer v1.0.0</span>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { globalState } from '../../stores/global';
import { open } from '@tauri-apps/plugin-shell';

const emit = defineEmits<{
  close: [],
  logout: []
}>()
</script>
