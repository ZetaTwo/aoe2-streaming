import { setActivePinia, createPinia } from 'pinia'
import { describe, it, expect, beforeEach, vi } from 'vitest'
import { useStandingsStore } from '../standings'

describe('Standings Store', () => {
  beforeEach(() => {
    setActivePinia(createPinia())
    vi.stubEnv('VITE_STANDINGS_PROXY_URL', 'http://proxy.test')
  })

  it('fetches a tournament from the proxy and stores brackets', async () => {
    const store = useStandingsStore()

    global.fetch = vi.fn().mockResolvedValue({
      ok: true,
      status: 200,
      json: () =>
        Promise.resolve({
          brackets: [
            {
              name: 'Champions',
              groups: [
                {
                  name: 'Group A',
                  players: [
                    {
                      rank: 1,
                      name: 'TheViper',
                      setsWon: 2,
                      setsLost: 0,
                      mapsWon: 4,
                      mapsLost: 1,
                    },
                  ],
                },
              ],
            },
          ],
        }),
    })

    await store.fetchTournament('tcc2')

    expect(global.fetch).toHaveBeenCalledWith('http://proxy.test/tournaments/tcc2')
    expect(store.isLoading).toBe(false)
    expect(store.error).toBe(null)
    expect(store.brackets.length).toBe(1)

    const bracket = store.brackets[0]!
    expect(bracket.name).toBe('Champions')
    expect(bracket.groups[0]!.players[0]!.name).toBe('TheViper')
    expect(bracket.groups[0]!.players[0]!.setsWon).toBe(2)
    expect(bracket.groups[0]!.players[0]!.mapsLost).toBe(1)
  })

  it('trims a trailing slash on the proxy URL', async () => {
    vi.stubEnv('VITE_STANDINGS_PROXY_URL', 'http://proxy.test/')
    const store = useStandingsStore()

    global.fetch = vi.fn().mockResolvedValue({
      ok: true,
      status: 200,
      json: () => Promise.resolve({ brackets: [] }),
    })

    await store.fetchTournament('ttlc2')

    expect(global.fetch).toHaveBeenCalledWith('http://proxy.test/tournaments/ttlc2')
  })

  it('surfaces 404 as an unknown-tournament error', async () => {
    const store = useStandingsStore()

    global.fetch = vi.fn().mockResolvedValue({
      ok: false,
      status: 404,
      statusText: 'Not Found',
    })

    await store.fetchTournament('does-not-exist')

    expect(store.error).toContain('does-not-exist')
    expect(store.isLoading).toBe(false)
    expect(store.brackets).toEqual([])
  })

  it('surfaces non-404 errors with the proxy status text', async () => {
    const store = useStandingsStore()

    global.fetch = vi.fn().mockResolvedValue({
      ok: false,
      status: 502,
      statusText: 'Bad Gateway',
    })

    await store.fetchTournament('tcc2')

    expect(store.error).toContain('Bad Gateway')
    expect(store.isLoading).toBe(false)
  })

  it('errors when the proxy URL is not configured', async () => {
    vi.stubEnv('VITE_STANDINGS_PROXY_URL', '')
    const store = useStandingsStore()

    await store.fetchTournament('tcc2')

    expect(store.error).toContain('VITE_STANDINGS_PROXY_URL is not configured')
  })
})
