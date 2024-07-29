<script setup lang="ts">
import { computed, ref } from 'vue';
import { invoke } from '@tauri-apps/api';
import { message } from 'ant-design-vue';
import MainForm, { FormState } from './components/MainForm.vue';
import SettingsForm from './components/SettingsForm.vue';
import FigureZone from './components/FigureZone.vue';
import { useSettingsStore } from './stores/settings.ts';
import { storeToRefs } from 'pinia';

interface Response {
  p: unknown[]; // 功率绘图数据
  w: unknown[]; // 能耗绘图数据
}

/** 图表是否为加载中 */
const spinning = ref(false);

/** 侧边栏是否打开，注：不要使用 `open` 作为变量名，会与 tauri-api 中的函数同名 */
const isDrawerOpen = ref(false);

// 后端处理返回的全部功率、能耗数据
const powerRecords = ref<unknown[]>([]);
const workRecords = ref<unknown[]>([]);

const settingsStore = useSettingsStore();
const { smooth, month } = storeToRefs(settingsStore);

// 的经过筛选的数据，用于绘图
const powerData = computed(() => {
  if (powerRecords.value.length === 0) {
    return [];
  }

  if (month.value === 'all') {
    return powerRecords.value;
  }

  return powerRecords.value.filter((r) => (r as any).t.slice(5, 7) === month.value);
});
const workData = computed(() => {
  if (workRecords.value.length === 0) {
    return [];
  }

  if (month.value === 'all') {
    return workRecords.value;
  }

  return workRecords.value.filter((r) => (r as any).d.slice(5, 7) === month.value);
});

// 获取表单数据并发往后端，后端返回绘制图表所需数据，然后利用数据绘图
const requestFigureData = async (state: FormState) => {
  // 清除原有数据，相比直接覆盖显得较为美观
  powerRecords.value = [];
  workRecords.value = [];

  spinning.value = true;

  try {
    const response: Response = await invoke('build_datasets', state as any);
    powerRecords.value = response.p;
    workRecords.value = response.w;
  } catch (e) {
    const msg = `数据处理发生错误：${e as string}`;
    console.error(msg);
    message.error(msg);
  }

  spinning.value = false;
};
</script>

<template>
  <MainForm
    @submit="requestFigureData"
    @requireOpenDrawer="() => isDrawerOpen = true"
  />

  <FigureZone
    :power-data="powerData"
    :work-data="workData"
    :smooth="smooth"
    :spinning="spinning"
  />

  <ADrawer v-model:open="isDrawerOpen" placement="left">
    <SettingsForm />
  </ADrawer>
</template>
