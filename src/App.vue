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
} from 'echarts/components'
import { CanvasRenderer } from 'echarts/renderers';
import { invoke } from '@tauri-apps/api';
import { open } from '@tauri-apps/api/dialog';

// 基本都是抄的
use([
  TitleComponent,
  TooltipComponent,
  ToolboxComponent,
  LegendComponent,
  GridComponent,
  DatasetComponent,
  DataZoomComponent,
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

// 以下配置基本都是抄的
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
    // `type` 设为 `time` 时经常会有各种莫名其妙的问题，所以此处使用 `category`
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
const onFinish = async (state: FormState) => {
  console.log({values: state});

  const data = await invoke('handle_submit', state as any);
  // const data = await invoke('handle_submit', { ratedCapacity: state.ratedCapacity, isPrimaryLoad: state.isPrimaryLoad, ratio: state.ratio, filepath: state.filepath });
  console.log({data});
};

const openFile = async () => {
  const filepath = await open();
  if (filepath) {
    if (typeof(filepath) == 'string') {
      formState.filepath = filepath;
    } else {
      formState.filepath = filepath[0];
    }
  }
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
        <a-space>
          <a-input
            id="file-selector"
            v-model:value="formState.filepath"
            placeholder="文件路径"
          />
          <a-button @click="openFile">...</a-button>
        </a-space>
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

  /* 令图表占满剩余高度 */
  display: flex;
  flex-grow: 1;
}
</style>
