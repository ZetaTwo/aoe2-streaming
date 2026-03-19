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
  border: 1px solid gray;
  border-radius: 8px;
  overflow: hidden;
}

th,
:deep(td) {
  text-align: center;
  padding: 0.5rem 1.25rem;
  border-bottom: 1px solid gray;
  border-right: 1px solid gray;
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
