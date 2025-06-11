<script setup lang="ts">
type Source = 'U' | 'E' | 'S' | 'F';

const props = defineProps<{
  isTimerOn: boolean;
  startTime: number | null;
  startedBy: Source | null;
  status: string;
  elapsedTime: string;
  startTimer: (source: Source) => void;
}>();
</script>

<template>
  <div class="row">
    <img src="https://cdn-icons-png.flaticon.com/128/3522/3522691.png" class="logo vite" @click="props.startTimer('U')" />
    <img src="https://cdn-icons-png.flaticon.com/128/3523/3523059.png" class="logo vue" @click="props.startTimer('E')" />
    <img src="https://cdn-icons-png.flaticon.com/128/3522/3522643.png" class="logo vue" @click="props.startTimer('S')" />
    <img src="https://cdn-icons-png.flaticon.com/128/3522/3522317.png" class="logo vite" @click="props.startTimer('F')" />
  </div>
  <p class="word">{{ props.status }}</p>
  <p class="word">Elapsed Time: {{ props.elapsedTime }}</p>
</template>

<style scoped>
.row {
  display: flex;
  justify-content: center;
}
.word {
  font-size: 1.5rem;
  font-family: 'Courier New', Courier, monospace;
  font-weight: 800;
  color: #1f1e33;
  margin-top: 1rem;
  text-align: center;
}
.logo {
  height: 6em;
  padding: 1.5em;
  transition: 0.75s;
  pointer-events: auto;
}
.logo.vite:hover {
  filter: drop-shadow(0 0 2em #430a67);
}
.logo.vue:hover {
  filter: drop-shadow(0 0 2em #431060);
}
</style>
