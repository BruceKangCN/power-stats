<script setup lang="ts">
import { defineEmits, ref } from 'vue';
import type { SelectProps } from 'ant-design-vue';
import { useSettingsStore } from '../stores/settings.ts';

const emit = defineEmits(['update']);

const store = useSettingsStore();

const monthOptions = ref<SelectProps['options']>([
  {
    value: 'all',
    label: '全部',
  },
  {
    value: '01',
    label: '1',
  },
  {
    value: '02',
    label: '2',
  },
  {
    value: '03',
    label: '3',
  },
  {
    value: '04',
    label: '4',
  },
  {
    value: '05',
    label: '5',
  },
  {
    value: '06',
    label: '6',
  },
  {
    value: '07',
    label: '7',
  },
  {
    value: '08',
    label: '8',
  },
  {
    value: '09',
    label: '9',
  },
  {
    value: '10',
    label: '10',
  },
  {
    value: '11',
    label: '11',
  },
  {
    value: '12',
    label: '12',
  },
]);
</script>

<template>
    <a-form name="figure-settings">

      <a-form-item
        label="曲线平滑"
        name="smooth"
      >
        <a-switch v-model:checked="store.smooth" />
      </a-form-item>

      <a-form-item
        label="月份"
        name="month"
      >
        <a-select
          v-model:value="store.month"
          :options="monthOptions"
        />
      </a-form-item>

    </a-form>
</template>
