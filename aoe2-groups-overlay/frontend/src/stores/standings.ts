import { ref } from 'vue'
import { defineStore } from 'pinia'
import { mockBrackets } from '@/mocks/standings'

export interface PlayerStanding {
  rank: number
  name: string
  setsWon: number
  setsLost: number
  mapsWon: number
  mapsLost: number
}

export interface GroupStanding {
  name: string
  players: PlayerStanding[]
}

export interface BracketStanding {
  name: string
  groups: GroupStanding[]
}

interface TournamentResponse {
  brackets: BracketStanding[]
}

export const useStandingsStore = defineStore('standings', () => {
  const brackets = ref<BracketStanding[]>([])
  const isLoading = ref(false)
  const error = ref<string | null>(null)

  const fetchTournament = async (tournamentId: string) => {
    isLoading.value = true
    error.value = null
    brackets.value = []
    try {
      if (import.meta.env.VITE_USE_MOCK_DATA === 'true') {
        brackets.value = mockBrackets
        return
      }

      const baseUrl = import.meta.env.VITE_STANDINGS_PROXY_URL
      if (!baseUrl) {
        throw new Error('VITE_STANDINGS_PROXY_URL is not configured')
      }
      const url = `${baseUrl.replace(/\/$/, '')}/tournaments/${encodeURIComponent(tournamentId)}`
      const response = await fetch(url)

      if (response.status === 404) {
        throw new Error(`Tournament not found: ${tournamentId}`)
      }
      if (!response.ok) {
        throw new Error(`Failed to fetch standings: ${response.statusText}`)
      }

      const data = (await response.json()) as TournamentResponse
      brackets.value = data.brackets ?? []
    } catch (err: unknown) {
      error.value = err instanceof Error ? err.message : 'An unknown error occurred'
    } finally {
      isLoading.value = false
    }
  }

  return { brackets, isLoading, error, fetchTournament }
})
