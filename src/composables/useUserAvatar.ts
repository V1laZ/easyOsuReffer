import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { avatarCache, pendingRequests } from '@/main'
import { dbService } from '@/services/database'
import { globalState } from '@/stores/global'

export function useUserAvatar(username: string) {
  const avatarUrl = ref<string | null>(null)

  const fetchAvatar = async () => {
    if (!username || username === 'BanchoBot') {
      avatarUrl.value = null
      return
    }

    if (avatarCache.has(username)) {
      avatarUrl.value = avatarCache.get(username)!
      return
    }

    const accessToken = await dbService.getAccessToken(globalState.user || '')
    if (!accessToken) {
      avatarUrl.value = null
      return
    }

    if (pendingRequests.has(username)) {
      try {
        const url = await pendingRequests.get(username)!.promise
        avatarUrl.value = url
      }
      catch (err) {
        console.error(`Failed to load avatar for ${username}:`, err)
      }
      return
    }

    let resolveRequest: (url: string) => void
    let rejectRequest: (error: unknown) => void

    const promise = new Promise<string>((resolve, reject) => {
      resolveRequest = resolve
      rejectRequest = reject
    })

    pendingRequests.set(username, {
      promise,
      resolve: resolveRequest!,
      reject: rejectRequest!,
    })

    try {
      const userData = await invoke<{ avatar_url?: string }>('fetch_user_data', {
        username,
        accessToken,
      })

      const url = userData.avatar_url

      if (url) {
        avatarCache.set(username, url)
        avatarUrl.value = url

        pendingRequests.get(username)?.resolve(url)
      }
      else {
        throw new Error('No avatar URL in response')
      }
    }
    catch (err) {
      console.error(`Failed to load avatar for ${username}:`, err)

      pendingRequests.get(username)?.reject(err)
    }
    finally {
      pendingRequests.delete(username)
    }
  }

  return {
    avatarUrl,
    fetchAvatar,
  }
}
