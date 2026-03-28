<script setup lang="ts">
import type { PlayerStanding } from '@/stores/standings'

defineProps<{
  player: PlayerStanding
  playerIndex: number
}>()
</script>

<template>
  <tr class="player-standing-row" :class="`player-${playerIndex}`">
    <td class="player-standing-rank">{{ player.rank }}</td>
    <td class="player-standing-name">{{ player.name }}</td>
    <td class="player-standing-sets">{{ player.setsWon }} - {{ player.setsLost }}</td>
    <td class="player-standing-maps">{{ player.mapsWon }} - {{ player.mapsLost }}</td>
  </tr>
</template>
