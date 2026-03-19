import { ref } from 'vue'
import { defineStore } from 'pinia'

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

export interface BracketConfig {
  name: string
  groupRanges: string[]
}

export interface TournamentConfig {
  sheetId: string
  brackets: BracketConfig[]
}

const TOURNAMENTS: Record<string, TournamentConfig> = {
  tcc2: {
    sheetId: '1uRMlH-VQpuB9Q2_2xnp70JqR1712jgJVu70cpiMMy5o',
    brackets: [
      {
        name: 'Champions',
        groupRanges: ['G4:P9', 'V4:AE9', 'G23:P28', 'V23:AE28'],
      },
      {
        name: 'Monks',
        groupRanges: ['G4:P9', 'V4:AE9', 'G23:P28', 'V23:AE28'],
      },
      {
        name: 'Mangonels',
        groupRanges: ['G4:P9', 'V4:AE9', 'G23:P28', 'V23:AE28'],
      },
      {
        name: 'Knights',
        groupRanges: ['G4:P9', 'V4:AE9', 'G23:P28', 'V23:AE28'],
      },
      {
        name: 'Pikemen',
        groupRanges: ['G4:P9', 'V4:AE9', 'G23:P28', 'V23:AE28'],
      },
      {
        name: 'Longswords',
        groupRanges: ['G4:P9', 'V4:AE9', 'G23:P28', 'V23:AE28'],
      },
      {
        name: 'Crossbows',
        groupRanges: ['G4:P9', 'V4:AE9', 'G23:P28', 'V23:AE28'],
      },
      {
        name: 'Archers',
        groupRanges: ['G4:P9', 'V4:AE9', 'G23:P28', 'V23:AE28'],
      },
      {
        name: 'Militia',
        groupRanges: ['G4:P9', 'V4:AE9', 'G23:P28', 'V23:AE28'],
      },
    ],
  },
}

export function getTournamentConfig(tournamentName: string): TournamentConfig | undefined {
  return TOURNAMENTS[tournamentName]
}

export const useStandingsStore = defineStore('standings', () => {
  const brackets = ref<BracketStanding[]>([])
  const isLoading = ref(false)
  const error = ref<string | null>(null)

  const fetchStandings = async (sheetId: string, bracketsConfig: BracketConfig[]) => {
    isLoading.value = true
    error.value = null
    try {
      const apiKey = import.meta.env.VITE_GOOGLE_SHEETS_API_KEY
      if (!apiKey) {
        throw new Error('Google Sheets API key is missing')
      }

      const ranges = bracketsConfig.flatMap((bracket) =>
        bracket.groupRanges.map((range) => `'${bracket.name}'!${range}`)
      )
      const rangesQuery = ranges.map((r) => `ranges=${encodeURIComponent(r)}`).join('&')
      const url = `https://sheets.googleapis.com/v4/spreadsheets/${sheetId}/values:batchGet?${rangesQuery}&key=${apiKey}`

      const response = await fetch(url)

      if (!response.ok) {
        throw new Error(`Failed to fetch sheet data: ${response.statusText}`)
      }

      const data = await response.json()

      if (!data.valueRanges) {
        throw new Error('Invalid data format received from Google Sheets API')
      }

      const parsedBrackets: BracketStanding[] = []
      let rangeIndex = 0

      for (const bracketConfig of bracketsConfig) {
        const parsedGroups: GroupStanding[] = []
        let groupIndex = 0

        for (let i = 0; i < bracketConfig.groupRanges.length; i++) {
          const rangeData = data.valueRanges[rangeIndex++]
          const values = rangeData?.values
          if (!values || values.length === 0) {
            continue // Skip empty ranges
          }

          const groupName = `Group ${String.fromCharCode(65 + groupIndex)}`
          groupIndex++

          const players: PlayerStanding[] = []
          for (let j = 0; j < values.length; j++) {
            const row = values[j]

            // Fixed indices based on layout:
            // 0: rank, 1: name, 2: blank, 3: map diff, 4: sets won, 5: dash, 6: sets lost, 7: maps won, 8: dash, 9: maps lost
            const rankVal = row[0]
            const nameVal = row[1]

            // Skip if no name or if name is clearly a header like "Player" or "Name"
            if (!nameVal || typeof nameVal !== 'string') continue
            const nameStr = nameVal.trim()
            if (
              nameStr === '' ||
              nameStr.toLowerCase() === 'name' ||
              nameStr.toLowerCase() === 'player'
            )
              continue

            const setsWon = parseInt(row[4], 10) || 0
            const setsLost = parseInt(row[6], 10) || 0
            const mapsWon = parseInt(row[7], 10) || 0
            const mapsLost = parseInt(row[9], 10) || 0

            players.push({
              name: nameStr,
              rank: parseInt(rankVal, 10) || players.length + 1,
              setsWon,
              setsLost,
              mapsWon,
              mapsLost,
            })
          }

          parsedGroups.push({ name: groupName, players })
        }
        parsedBrackets.push({ name: bracketConfig.name, groups: parsedGroups })
      }

      brackets.value = parsedBrackets
    } catch (err: unknown) {
      if (err instanceof Error) {
        error.value = err.message
      } else {
        error.value = 'An unknown error occurred'
      }
      throw err // Rethrow to allow component to handle it if needed
    } finally {
      isLoading.value = false
    }
  }

  const fetchTournament = async (tournamentId: string) => {
    const config = getTournamentConfig(tournamentId)
    if (!config) {
      error.value = `Tournament configuration not found for: ${tournamentId}`
      return
    }
    await fetchStandings(config.sheetId, config.brackets)
  }

  return { brackets, isLoading, error, fetchStandings, fetchTournament }
})
