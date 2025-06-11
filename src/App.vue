<script setup lang="ts">
import { ref, computed, onUnmounted } from 'vue';
import NavBar from './components/NavBar.vue';
import TimerView from './components/TimerView.vue';
import DayView from './components/DayView.vue';
import WeekView from './components/WeekView.vue';
import MonthView from './components/MonthView.vue';
import { appDataDir } from '@tauri-apps/api/path';
import { invoke } from '@tauri-apps/api/core';

const currentPage = ref<'timer' | 'day' | 'week' | 'month'>('timer');
type Source = 'U' | 'E' | 'S' | 'F'; 

function setPage(page: string) {
  currentPage.value = page as any;
}

const isTimerOn = ref(false);
const startTime = ref<number | null>(null);
const startedBy = ref<Source | null>(null);

const status = ref('');
const elapsedTime = ref(0);
let timerId: ReturnType<typeof setInterval> | null = null;

async function startTimer(source: Source) {
  if (isTimerOn.value) {
    isTimerOn.value = false;
    if (timerId !== null) {
      clearInterval(timerId);
      timerId = null;
    }

    const currentTime = Date.now();
    const totalElapsed = startTime.value ? Math.floor((currentTime - startTime.value) / 1000) : 0;
    startTime.value = null;

    const dir = (await appDataDir()) + 'my-app-data';
    if (startedBy.value) {
      try {
        await invoke('log_session_csv', {
          activity: startedBy.value,
          duration: totalElapsed,
          directory: dir
        });
      } catch (e) {
        console.error('Failed to log session:', e);
      }
    }

    status.value = `Done ${startedBy.value}. Total elapsed time is : ${totalElapsed} seconds.`;
    startedBy.value = null;

  } else {
    isTimerOn.value = true;
    startTime.value = Date.now();
    startedBy.value = source;
    elapsedTime.value = 0;
    status.value = `Current Task: ${source}`;
    timerId = setInterval(() => {
      if (startTime.value) {
        elapsedTime.value = Math.floor((Date.now() - startTime.value) / 1000);
      }
    }, 1000);
  }
}

const formattedElapsedTime = computed(() => {
  const total = elapsedTime.value;
  const hours = Math.floor(total / 3600);
  const minutes = Math.floor((total % 3600) / 60);
  const seconds = total % 60;
  const pad = (n: number) => String(n).padStart(2, '0');
  return `${pad(hours)}h ${pad(minutes)}m ${pad(seconds)}s`;
});

onUnmounted(() => {
  if (timerId !== null) {
    clearInterval(timerId);
    timerId = null;
  }
});
</script>

<template>
  <main class="container">
    <NavBar :currentPage="currentPage" :setPage="setPage" />

    <div v-if="currentPage === 'timer'">
      <TimerView 
        :isTimerOn="isTimerOn"
        :startTime="startTime"
        :startedBy="startedBy"
        :status="status"
        :elapsedTime="formattedElapsedTime"
        :startTimer="startTimer"
      />
    </div>
    <div v-else-if="currentPage === 'day'">
      <DayView />
    </div>
    <div v-else-if="currentPage === 'week'">
      <WeekView />
    </div>
    <div v-else-if="currentPage === 'month'">
      <MonthView />
    </div>
    <div v-else>
      <p style="text-align: center; font-size: 2rem;">This is {{ currentPage }}</p>
    </div>
  </main>
</template>

<style>
.container {
  margin: 0;
  display: flex;
  flex-direction: column;
  justify-content: center;
  text-align: center;
  background-color: rgba(149, 142, 167, 0.725);
  min-height: 100vh;
  overflow: hidden;
  padding-top: 5%;
}
#app {
  height: 100%;
}
</style>
