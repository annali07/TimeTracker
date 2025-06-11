// ==========================
// File: src/components/NavBar.vue
// ==========================

<script setup lang="ts">
import { defineProps } from 'vue';

const props = defineProps<{
  currentPage: string;
  setPage: (page: string) => void;
}>();

const tabOffset: Record<string, string> = {
  timer: '0%',
  day: '25%',
  week: '50%',
  month: '75%'
};
</script>

<template>
  <nav class="top-nav">
    <div class="tab-container">
      <div class="tab-highlight" :style="{ left: tabOffset[props.currentPage] }"></div>
      <button :class="{ active: currentPage === 'timer' }" @click="props.setPage('timer')">Timer</button>
      <button :class="{ active: currentPage === 'day' }" @click="props.setPage('day')">Day</button>
      <button :class="{ active: currentPage === 'week' }" @click="props.setPage('week')">Week</button>
      <button :class="{ active: currentPage === 'month' }" @click="props.setPage('month')">Month</button>
    </div>
  </nav>
</template>

<style scoped>
.top-nav {
  display: flex;
  justify-content: center;
  /* margin-top: 1rem; */
}
.tab-container {
  position: relative;
  display: flex;
  background-color: #1f1e33;
  border-radius: 8px;
  overflow: hidden;
  width: 80%;
  max-width: 600px;
  height: 40px;
  margin-top: -70px;
}
.tab-container button {
  flex: 1;
  background: none;
  border: none;
  color: rgba(215, 203, 238, 0.656);
  font-weight: 600;
  font-size: 1rem;
  z-index: 1;
  cursor: pointer;
  padding: 0.5rem 1rem;
}
.tab-container button.active {
  color: #1f1e33;
}
.tab-highlight {
  position: absolute;
  top: 0;
  bottom: 0;
  width: 25%;
  background-color: #ffffffd7;
  border-radius: 8px;
  transition: left 0.3s ease;
  z-index: 0;
}
</style>