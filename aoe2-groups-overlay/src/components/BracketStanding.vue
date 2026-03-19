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
        v-for="group in bracket.groups"
        :key="group.name"
        :group="group"
      />
    </div>
  </div>
</template>

<style scoped>
.groups-container {
  display: grid;
  grid-template-columns: repeat(2, 1fr);
}

.bracket-title {
  text-align: center;
}
</style>
