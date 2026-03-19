import { setActivePinia, createPinia } from 'pinia'
import { describe, it, expect, beforeEach, vi } from 'vitest'
import { useStandingsStore } from '../standings'

describe('Standings Store', () => {
  beforeEach(() => {
    setActivePinia(createPinia())
    vi.stubEnv('VITE_GOOGLE_SHEETS_API_KEY', 'test_key')
  })

  it('fetches and parses standings correctly', async () => {
    const store = useStandingsStore()

    // Mock global fetch
    // Format: rank, name, (blank), map diff, sets won, (dash), sets lost, maps won, (dash), maps lost
    global.fetch = vi.fn().mockResolvedValue({
      ok: true,
      json: () =>
        Promise.resolve({
          valueRanges: [
            {
              range: "'Group A'!A1:J3",
              majorDimension: 'ROWS',
              values: [
                [
                  'Rank',
                  'Player',
                  '',
                  'Map Diff',
                  'Sets W',
                  '-',
                  'Sets L',
                  'Maps W',
                  '-',
                  'Maps L',
                ],
                ['1', 'TheViper', '', '+3', '2', '-', '0', '4', '-', '1'],
                ['2', 'Hera', '', '-1', '1', '-', '1', '2', '-', '3'],
              ],
            },
          ],
        }),
    })

    await store.fetchStandings('mock_sheet_id', [{ name: 'Group A', groupRanges: ['A1:J3'] }])

    expect(global.fetch).toHaveBeenCalled()
    expect(store.isLoading).toBe(false)
    expect(store.error).toBe(null)
    expect(store.brackets.length).toBe(1)
    expect(store.brackets[0]!.name).toBe('Group A')
    expect(store.brackets[0]!.groups.length).toBe(1)

    const group = store.brackets[0]!.groups[0]!
    expect(group.name).toBe('Group A')
    expect(group.players.length).toBe(2)

    expect(group.players[0]!.name).toBe('TheViper')
    expect(group.players[0]!.setsWon).toBe(2)
    expect(group.players[0]!.mapsLost).toBe(1)
    expect(group.players[0]!.rank).toBe(1)
  })

  it('handles fetch errors', async () => {
    const store = useStandingsStore()

    global.fetch = vi.fn().mockResolvedValue({
      ok: false,
      statusText: 'Not Found',
    })

    await expect(store.fetchStandings('mock_sheet_id', [{ name: 'Group A', groupRanges: ['A1:J3'] }])).rejects.toThrow(
      'Failed to fetch',
    )

    expect(store.error).toContain('Failed to fetch')
    expect(store.isLoading).toBe(false)
  })

  it('handles missing API key', async () => {
    vi.stubEnv('VITE_GOOGLE_SHEETS_API_KEY', '')
    const store = useStandingsStore()

    await expect(store.fetchStandings('mock_sheet_id', [{ name: 'Group A', groupRanges: ['A1:J3'] }])).rejects.toThrow(
      'Google Sheets API key is missing',
    )

    expect(store.error).toContain('missing')
  })

  it('fetches a tournament configuration successfully', async () => {
    const store = useStandingsStore()

    global.fetch = vi.fn().mockResolvedValue({
      ok: true,
      json: () =>
        Promise.resolve({
          valueRanges: [
            {
              range: "'Champions'!G4:P9",
              majorDimension: 'ROWS',
              values: [
                [
                  'Rank',
                  'Player',
                  '',
                  'Map Diff',
                  'Sets W',
                  '-',
                  'Sets L',
                  'Maps W',
                  '-',
                  'Maps L',
                ],
                ['1', 'Tatoh', '', '+3', '2', '-', '0', '4', '-', '1'],
              ],
            },
          ],
        }),
    })

    await store.fetchTournament('tcc2')

    expect(global.fetch).toHaveBeenCalled()
    expect(store.isLoading).toBe(false)
    expect(store.error).toBe(null)
    expect(store.brackets.length).toBe(9)
    
    const bracket = store.brackets[0]!
    expect(bracket.name).toBe('Champions')
    expect(bracket.groups.length).toBe(1)

    const group = bracket.groups[0]!
    expect(group.name).toBe('Group A')
    expect(group.players[0]!.name).toBe('Tatoh')
  })

  it('handles unknown tournament id', async () => {
    const store = useStandingsStore()

    await store.fetchTournament('unknown-tournament')
    expect(store.error).toContain('Tournament configuration not found for: unknown-tournament')
  })
})
