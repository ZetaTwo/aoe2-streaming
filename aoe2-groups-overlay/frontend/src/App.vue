<script setup lang="ts">
import { onMounted, ref, computed } from 'vue'
import { useStandingsStore } from '@/stores/standings'
import BracketStanding from '@/components/BracketStanding.vue'

const standingsStore = useStandingsStore()
const selectedBracket = ref<string | null>(null)

onMounted(() => {
  const urlParams = new URLSearchParams(window.location.search)
  const tournamentId = urlParams.get('tournament')
  selectedBracket.value = urlParams.get('bracket')

  if (tournamentId) {
    standingsStore.fetchTournament(tournamentId)
  }
})

const displayedBrackets = computed(() => {
  if (!selectedBracket.value) return standingsStore.brackets
  return standingsStore.brackets.filter((b) => b.name === selectedBracket.value)
})
</script>

<template>
  <main class="app-main">
    <div v-if="standingsStore.isLoading" class="loading-message">Loading standings...</div>
    <div v-else-if="standingsStore.error" class="error">{{ standingsStore.error }}</div>
    <div v-else class="brackets-container">
      <BracketStanding
        v-for="bracket in displayedBrackets"
        :key="bracket.name"
        :bracket="bracket"
      />
    </div>
  </main>
</template>

<style scoped>
main {
  font-family:
    system-ui,
    -apple-system,
    'Segoe UI',
    Roboto,
    Helvetica,
    Arial,
    sans-serif;
  font-size: 1.2rem;
  font-weight: bold;
  color: #e0e0e0;
}
</style>
