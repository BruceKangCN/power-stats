<script setup lang="ts">
import { reactive, ref } from 'vue';
import { LineChart } from 'echarts/charts';
import VChart from 'vue-echarts';
import { use } from 'echarts/core';
import {
  TitleComponent,
  TooltipComponent,
  ToolboxComponent,
  LegendComponent,
  GridComponent,
  DatasetComponent,
  DataZoomComponent,
  DataZoomInsideComponent,
  DataZoomSliderComponent,
} from 'echarts/components'
import { CanvasRenderer } from 'echarts/renderers';

use([
  TitleComponent,
  TooltipComponent,
  ToolboxComponent,
  LegendComponent,
  GridComponent,
  DatasetComponent,
  DataZoomComponent,
  DataZoomInsideComponent,
  DataZoomSliderComponent,
  LineChart,
  CanvasRenderer,
]);

interface FormState {
  ratedCapacity: number; // 额定容量
  isPrimaryLoad: boolean; // 是否为一次负荷
  ratio?: number; // 倍率
  filepath: string; // 数据文件路径
}

const formState = reactive<FormState>({
  ratedCapacity: 0.0,
  isPrimaryLoad: true,
  ratio: 1.0,
  filepath: '',
})

const option = ref({
  legend: {},
  tooltip: {},
  toolbox: {
    feature: {
      saveAsImage: {
        name: 'figure',
        excludeComponents: ['tooltip', 'toolbox', 'dataZoom'],
      },
      dataZoom: {},
    },
  },
  dataZoom: [
    {
      id: 'dataZoomX',
      type: 'slider',
      xAxisIndex: [0],
    },
    {
      id: 'dataZoomY',
      type: 'slider',
      yAxisIndex: [0],
    },
  ],
  xAxis: {
    type: 'category',
  },
  yAxis: {
    type: 'value',
  },
  series: [
    {
      type: 'line',
      smooth: true,
      connectNulls: true,
    },
    {
      type: 'line',
      smooth: true,
      connectNulls: true,
    },
  ],
  dataset: {
    dimensions: ['time', 'user1', 'user2'],
    source: [
      { time: '2019-10-01T00:00:00', user1: 1.7, user2: 3.1 },
      { time: '2019-10-01T01:00:00', user1: 3.4, user2: null },
      { time: '2019-10-01T02:00:00', user1: 2.5, user2: 2.7 },
      { time: '2019-10-01T03:00:00', user1: 3.3, user2: null },
    ],
  },
});

// TODO: 获取表单数据并发往后端，返回绘制图表所需数据
const onFinish = (values: FormState) => {
  console.log({values});
};
</script>

<template>
  <a-flex
    gap="small"
    vertical
    style="height: 100vh;"
  >
    <a-form
      :model="formState"
      name="form"
      :label-col="{ span: 6 }"
      :wrapper-col="{ span: 16 }"
      autocomplete="off"
      @finish="onFinish"
    >
      <a-form-item
        label="变压器额定容量"
        name="ratedCapacity"
        :rules="[{ required: true, message: '请输入额定容量' }]"
      >
        <a-input-number v-model:value="formState.ratedCapacity" />
      </a-form-item>

      <a-form-item
        label="是否为一次负荷"
        name="isPrimaryLoad"
        :rules="[{ required: true, message: '请选择是否为一次负荷' }]"
      >
        <a-switch v-model:checked="formState.isPrimaryLoad" />
      </a-form-item>

      <a-form-item
        label="倍率"
        name="ratio"
        :rules="[{ required: formState.isPrimaryLoad, message: '一次负荷需填写倍率' }]"
      >
        <a-input-number v-model:value="formState.ratio" />
      </a-form-item>

      <a-form-item
        label="数据源"
        name="filepath"
        :rules="[{ required: true, message: '请选择数据文件' }]"
      >
        <a-input
          id="file-selector"
          v-model:value="formState.filepath"
          type="file"
          placeholder="文件路径"
        />
      </a-form-item>

      <a-form-item
        :wrapper-col="{ offset: 6, span: 16}"
      >
        <a-button type="primary" html-type="submit">确认</a-button>
      </a-form-item>
    </a-form>

    <a-divider />

    <v-chart class="chart" :option="option" autoresize />
  </a-flex>
</template>

<style scoped>
#file-selector::file-selector-button {
  color: white;
  background-color: #1677FF;
  border: none;
  border-radius: 0.5rem;
}

.chart {
  width: 100%;
  display: flex;
  flex-grow: 1;
}
</style>
