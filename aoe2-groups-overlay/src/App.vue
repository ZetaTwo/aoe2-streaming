<script setup lang="ts">
import { onMounted } from 'vue'
import { useStandingsStore } from '@/stores/standings'

const standingsStore = useStandingsStore()

onMounted(() => {
  const urlParams = new URLSearchParams(window.location.search)
  const tournamentId = urlParams.get('tournament')
  if (tournamentId) {
    standingsStore.fetchTournament(tournamentId)
  }
})
</script>

<template>
  <h1>You did it!</h1>
  <p>
    Visit <a href="https://vuejs.org/" target="_blank" rel="noopener">vuejs.org</a> to read the
    documentation
  </p>
</template>

<style scoped></style>
