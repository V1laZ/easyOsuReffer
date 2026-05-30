import { ref } from 'vue'
import { dbService } from '@/services/database'
import type { Mappool } from '@/types'

const mappools = ref<Mappool[]>([])

export function useMappools() {
  const loadMappools = async () => {
    try {
      mappools.value = await dbService.getMappools()
    }
    catch (error) {
      console.error('Failed to load mappools:', error)
    }
  }

  return { mappools, loadMappools }
}
