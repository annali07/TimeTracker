<script setup lang="ts">
import { ref, onMounted, onBeforeUnmount, computed } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { appDataDir } from '@tauri-apps/api/path';

const colorMap: Record<string, string> = {
  U: '#1f1e33',
  E: '#1f1e33',
  S: '#1f1e33',
  F: '#1f1e33'
};

const monthlyTotals = ref<Record<string, number>>({});
const currentTime = ref('');
let clockInterval: ReturnType<typeof setInterval>;

function updateClock() {
  const now = new Date();
  const month = now.toLocaleDateString(undefined, { year: 'numeric', month: 'long' });
  currentTime.value = `Month of ${month}`;
}

onMounted(async () => {
  try {
    const dir = (await appDataDir()) + 'my-app-data';
    monthlyTotals.value = await invoke<Record<string, number>>('get_monthly_summary_csv', { directory: dir });
  } catch {
    monthlyTotals.value = {};
  }
  updateClock();
  clockInterval = setInterval(updateClock, 1000);
});

onBeforeUnmount(() => clearInterval(clockInterval));

const maxTotal = computed(() => Math.max(...Object.values(monthlyTotals.value), 0));

function formatTime(sec: number): string {
  const h = Math.floor(sec / 3600);
  const m = Math.floor((sec % 3600) / 60);
  const s = sec % 60;
  return [h && `${h}h`, (h || m) && `${m}m`, `${s}s`].filter(Boolean).join(' ');
}
</script>


<template>
    <div class="day-container">
      <h2>{{  currentTime }}</h2>
  
      <div v-for="src of ['U','E','S','F']" :key="src" class="bar-row">
        <span class="label">{{ src }}</span>
        <div class="bar-bg">
          <div
            class="bar-fill"
            :style="{
              width: maxTotal > 0 ? ((monthlyTotals[src] || 0) * 100 / maxTotal) + '%' : '0%',
              backgroundColor: colorMap[src]
            }"
          />aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa
        </div>
        <span class="label-right">{{ formatTime(monthlyTotals[src] || 0) }}</span>
      </div>
  
    </div>
  </template>
  

<style scoped>
.day-container {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  /* padding: 2rem; */
  font-family: 'Inter', sans-serif;
  color: #1a1a1a;
}

h2 {
  font-size: 1.8rem;
  
  margin-top: -0.5rem;
}

ul {
  list-style: none;
  padding: 0;
  /* margin: 0 0 2rem 0; */
}

li {
  font-size: 1.4rem;
  margin: 0.5rem 0;
}

li span {
  font-weight: bold;
  margin-right: 0.5rem;
}

.day-container {
  font-family: Inter, sans-serif;
  color: #111;
  width: 100%;
  max-width: 500px;
  margin: auto;
}

.bar-row {
  display: flex;
  align-items: center;
  margin: 0.6rem 0;
}

.label {
  width: 1.5em;
  font-weight: bold;
}

.bar-bg {
  flex: 1;
  height: 1em;
  background: #eeeeee61;
  margin: 0 0.5em;
  border-radius: 0.25em;
  overflow: hidden;
  width: 100%;
}

.bar-fill {
  height: 100%;
  background: #4a90e2;
  transition: width 0.5s ease;
  min-width: max-content;
  border-radius: 0.25em;
}

.label-right {
  width: 4em;
  text-align: right;
  font-size: 0.9rem;
}

.clock {
  margin-top: 1.5rem;
  font-size: 0.9rem;
  color: #555;
}

</style>

