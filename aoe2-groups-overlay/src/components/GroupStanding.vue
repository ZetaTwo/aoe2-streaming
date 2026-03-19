<script setup lang="ts">
import type { GroupStanding } from '@/stores/standings'
import PlayerStandingRow from './PlayerStanding.vue'

defineProps<{
  group: GroupStanding
}>()
</script>

<template>
  <div class="group-standing">
    <h3 class="group-title">{{ group.name }}</h3>
    <table class="group-table">
      <thead class="group-table-head">
        <tr class="group-table-header-row">
          <th class="group-table-header-rank">Rank</th>
          <th class="group-table-header-player">Player</th>
          <th class="group-table-header-sets">Sets</th>
          <th class="group-table-header-maps">Maps</th>
        </tr>
      </thead>
      <tbody class="group-table-body">
        <PlayerStandingRow v-for="player in group.players" :key="player.name" :player="player" />
      </tbody>
    </table>
  </div>
</template>

<style scoped>
.group-standing {
  display: flex;
  align-items: center;
  flex-direction: column;
}

table {
  border-collapse: separate;
  border-spacing: 0;
  border: 2px solid #555;
  border-radius: 8px;
  overflow: hidden;
  background-color: #222;
  box-shadow: 0 4px 15px rgba(0, 0, 0, 0.5);
}

th {
  background-color: #333;
}

th,
:deep(td) {
  text-align: center;
  padding: 0.5rem 1.25rem;
  border-bottom: 2px solid #555;
  border-right: 2px solid #555;
}

th:last-child,
:deep(td:last-child) {
  border-right: none;
}

:deep(tr:last-child td) {
  border-bottom: none;
}

.group-table-header-player,
:deep(.player-standing-name) {
  text-align: left;
}
</style>
