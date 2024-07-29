import { defineStore } from 'pinia';
import { ref } from 'vue';

export const useSettingsStore = defineStore('settings', () => {
  const smooth = ref(true); // 曲线是否进行平滑处理
  const month = ref('all'); // 筛选的用于绘图的月份

  return { smooth, month };
});
