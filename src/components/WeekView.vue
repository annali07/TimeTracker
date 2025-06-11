<script setup lang="ts">
import { ref, onMounted, computed } from 'vue';
import { appDataDir } from '@tauri-apps/api/path';
import { invoke } from '@tauri-apps/api/core';

// Narrow sources type
const sources = ['U', 'E', 'S', 'F'] as const;
type Source = typeof sources[number];

// Fully typesafe backend data structure
type DayBlock = Record<Source, number>;
type WeekData = Record<string, DayBlock>;

// Reactive ref to hold loaded data
const weekData = ref<WeekData>({});

// Days of the week (fixed)
const days = ['Monday', 'Tuesday', 'Wednesday', 'Thursday', 'Friday', 'Saturday', 'Sunday'];

// Color map per activity
const colorMap: Record<Source, string> = {
  U: '#778da9',
  E: '#b0c4b1',
  S: '#edafb8',
  F: '#fcd5ce',
};

// Load backend data on mount
onMounted(async () => {
  const dir = (await appDataDir()) + 'my-app-data';
  try {
    const result = await invoke<WeekData>('get_weekly_summary_csv', { directory: dir });
    weekData.value = result;
  } catch (e) {
    console.error("Failed to load weekly summary", e);
  }
});

// Compute total duration for each day (used later)
function total(day: string): number {
  return sources.reduce((sum, src) => {
    return sum + (weekData.value?.[day]?.[src] ?? 0);
  }, 0);
}
function formatTime(sec: number): string {
  const h = Math.floor(sec / 3600);
  const m = Math.floor((sec % 3600) / 60);
  const s = sec % 60;
  return [h && `${h}h`, (h || m) && `${m}m`, `${s}s`].filter(Boolean).join(' ');
}
// Fully precompute segments for all days (very TS-safe)
const barSegments = computed(() => {
  return days.map(day => {
    const dayTotal = sources.reduce((sum, src) => sum + (weekData.value?.[day]?.[src] ?? 0), 0);
    return {
      day,
      segments: sources.map(src => {
        const value = weekData.value?.[day]?.[src] ?? 0;
        const height = dayTotal ? (value / dayTotal) * 170 : 0;
        return { src, value, height };
      }),
    };
  });
});
</script>

<template>
  <div class="week-container">
    
    <h2>Weekly Breakdown</h2>

    <div class="chart">
      <div v-for="bar in barSegments" :key="bar.day" class="day-column">
        <div class="stack-bar-container">
        <div class="stack-bar">
          <template v-if="total(bar.day) > 0">
            <div
              v-for="segment in bar.segments"
              :key="segment.src"
              class="segment"
              :style="{
                height: segment.height + 'px',
                backgroundColor: colorMap[segment.src]
              }"
            >
              <p v-if="segment.value > 0">
                 {{ formatTime(segment.value) }} 
              </p>
            </div>
          </template>
        </div>


             <!-- Hover Info Box -->
          <div class="info-box">
            <div v-for="segment in bar.segments" :key="segment.src" class="info-line">
              <span class="info-dot" :style="{ backgroundColor: colorMap[segment.src] }"></span>
              {{ segment.src }}: {{ formatTime(segment.value) }}
            </div>
          </div>
        </div>


        <div class="day-label">{{ bar.day.slice(0, 3) }}</div>
      </div>
    </div>
    </div>
</template>

<style scoped>
.week-container {
  padding: 1rem;
  text-align: center;
  font-family: Inter, sans-serif;
  color: #111;
}

h2 {
  font-size: 1.8rem;
  margin-bottom: 0rem;
  margin-top: -2rem;
  padding-bottom: 3rem;
}

.chart {
  display: flex;
  justify-content: space-around;
  align-items: flex-end;
  margin-top: 2rem;
  height: 150px;
}

.day-column {
  display: flex;
  flex-direction: column;
  align-items: center;
}

.stack-bar {
  width: 3.5rem;
  height: 170px;
  display: flex;
  flex-direction: column;
  border-radius: 0.4rem;
  overflow: hidden;
  background: #eee;
  border: 1px solid #ccc;
}

.segment {
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 0.8rem;
  color: white;
  text-shadow: 0 0 2px #000;
}

.segment-label {
  padding: 0 0.2rem;
}

.day-label {
  margin-top: 0.5rem;
  font-size: 0.9rem;
  font-weight: 600;
}
.stack-bar-container {
  position: relative;
  width: 3.5rem;
}
.stack-bar {
  height: 170px;
  display: flex;
  flex-direction: column;
  border-radius: 0.4rem;
  overflow: hidden;
  background: #eee;
  border: 1px solid #ccc;
}

.info-box {
  position: absolute;
  top: -10px; /* adjust as needed */
  left: 50%;
  transform: translateX(-50%);
  background: rgba(50, 50, 50, 0.69);
  color: white;
  padding: 0.5rem;
  border-radius: 0.4rem;
  font-size: 0.75rem;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.356);
  opacity: 0;
  pointer-events: none;
  transition: opacity 0.2s ease;
  z-index: 10;
  white-space: nowrap;
}

.stack-bar-container:hover .info-box {
  opacity: 1;
  pointer-events: auto;
}

.info-line {
  display: flex;
  align-items: center;
  margin-bottom: 0.2rem;
}

.info-dot {
  width: 0.7rem;
  height: 0.7rem;
  border-radius: 50%;
  margin-right: 0.4rem;
}

</style>
