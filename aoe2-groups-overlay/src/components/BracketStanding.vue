<script setup lang="ts">
import type { BracketStanding as BracketStandingType } from '@/stores/standings'
import GroupStandingTable from './GroupStanding.vue'

defineProps<{
  bracket: BracketStandingType
}>()
</script>

<template>
  <div class="bracket-standing">
    <h2 class="bracket-title">{{ bracket.name }} Bracket</h2>
    <div class="groups-container">
      <GroupStandingTable
        v-for="(group, index) in bracket.groups"
        :key="group.name"
        :group="group"
        :groupIndex="index + 1"
      />
    </div>
  </div>
</template>

<style scoped>
.groups-container {
  display: grid;
  grid-template-columns: repeat(2, max-content);
  justify-content: center;
  gap: 3rem;
}

.groups-container > :nth-child(odd) {
  justify-self: end;
}

.groups-container > :nth-child(even) {
  justify-self: start;
}

.bracket-title {
  text-align: center;
  background-color: #222;
  border: 2px solid #555;
  border-radius: 8px;
  box-shadow: 0 4px 15px rgba(0, 0, 0, 0.5);
  padding: 0.5rem 2rem;
  width: max-content;
  margin: 0 auto 2rem;
}
</style>
