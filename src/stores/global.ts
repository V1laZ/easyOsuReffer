import { reactive } from 'vue'

export const globalState = reactive({
  user: null as string | null,
  isConnected: false
})
