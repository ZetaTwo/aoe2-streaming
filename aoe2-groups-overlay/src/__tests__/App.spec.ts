import { describe, it, expect, beforeEach, vi } from 'vitest'
import { nextTick } from 'vue'
import { mount } from '@vue/test-utils'
import { createPinia, setActivePinia } from 'pinia'
import App from '../App.vue'
import { useStandingsStore } from '@/stores/standings'

describe('App', () => {
  beforeEach(() => {
    setActivePinia(createPinia())
    vi.stubGlobal('location', { search: '' })
  })

  it('shows loading message while fetching', async () => {
    const store = useStandingsStore()
    store.isLoading = true

    const wrapper = mount(App)
    expect(wrapper.find('.loading-message').exists()).toBe(true)
    expect(wrapper.find('.brackets-container').exists()).toBe(false)
  })

  it('shows error message on failure', async () => {
    const store = useStandingsStore()
    store.error = 'Failed to fetch standings'

    const wrapper = mount(App)
    expect(wrapper.find('.error').text()).toContain('Failed to fetch standings')
    expect(wrapper.find('.brackets-container').exists()).toBe(false)
  })

  it('renders all brackets when no bracket filter is set', async () => {
    const store = useStandingsStore()
    store.brackets = [
      { name: 'Alpha', groups: [] },
      { name: 'Beta', groups: [] },
    ]

    const wrapper = mount(App)
    const bracketTitles = wrapper.findAll('.bracket-title')
    expect(bracketTitles).toHaveLength(2)
    expect(bracketTitles[0]!.text()).toContain('Alpha')
    expect(bracketTitles[1]!.text()).toContain('Beta')
  })

  it('filters brackets by name when bracket query param is set', async () => {
    vi.stubGlobal('location', { search: '?bracket=Beta' })
    const store = useStandingsStore()
    store.brackets = [
      { name: 'Alpha', groups: [] },
      { name: 'Beta', groups: [] },
    ]

    const wrapper = mount(App)
    await nextTick()
    const bracketTitles = wrapper.findAll('.bracket-title')
    expect(bracketTitles).toHaveLength(1)
    expect(bracketTitles[0]!.text()).toContain('Beta')
  })

  it('calls fetchTournament on mount when tournament param is present', async () => {
    vi.stubGlobal('location', { search: '?tournament=tcc2' })
    const store = useStandingsStore()
    store.fetchTournament = vi.fn().mockResolvedValue(undefined)

    mount(App)
    expect(store.fetchTournament).toHaveBeenCalledWith('tcc2')
  })

  it('does not call fetchTournament when no tournament param is present', async () => {
    const store = useStandingsStore()
    store.fetchTournament = vi.fn()

    mount(App)
    expect(store.fetchTournament).not.toHaveBeenCalled()
  })
})
