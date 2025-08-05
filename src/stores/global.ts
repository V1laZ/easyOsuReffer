import { reactive } from 'vue'

export const globalState = reactive({
  user: null as string | null,
  userId: null as number | null,
  isConnected: false,
  isConnectedOsu: false,
})

export const modalsState = reactive({
  showOAuthCallback: false,
})
