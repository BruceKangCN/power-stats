<script setup lang="ts">
import { computed, reactive, ref } from 'vue';
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
import { message } from 'ant-design-vue';

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
  factor?: number; // 倍率
  filepath: string; // 数据文件路径
}

interface Response {
  p: unknown[]; // 功率绘图数据
  w: unknown[]; // 能耗绘图数据
}

const formState = reactive<FormState>({
  ratedCapacity: 0.0,
  isPrimaryLoad: true,
  factor: 1.0,
  filepath: '',
})

/** 当前显示的标签页的 key */
const activeKey = ref('1');

/** 图表是否为加载中 */
const spinning = ref(false);

// 后端处理返回的全部功率、能耗数据
const powerRecords = ref<unknown[]>([]);
const workRecords = ref<unknown[]>([]);

// TODO: 添加筛选器，根据月份筛选
// 的经过筛选的数据，用于绘图
const powerData = computed(() => powerRecords.value);
const workData = computed(() => workRecords.value);

// /**
//  * 用于格式化图表信息中的数字的函数
//  *
//  * @deprecated 某些情况下会导致图表 tooltip 无法正常显示
//  */
// const numberFormatter = (value: number) => value.toFixed(4);

const powerOption = reactive({
  legend: {},
  tooltip: {
    trigger: 'axis',
  },
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
      name: '晚谷瞬时功率',
      smooth: true,
      connectNulls: true,
    },
    {
      type: 'line',
      name: '早峰瞬时功率',
      smooth: true,
      connectNulls: true,
    },
    {
      type: 'line',
      name: '午谷瞬时功率',
      smooth: true,
      connectNulls: true,
    },
    {
      type: 'line',
      name: '午峰瞬时功率',
      smooth: true,
      connectNulls: true,
    },
    {
      type: 'line',
      name: '晚谷余量',
      smooth: true,
      connectNulls: true,
    },
    {
      type: 'line',
      name: '午谷余量',
      smooth: true,
      connectNulls: true,
    },
  ],
  dataset: {
    dimensions: ['t', 'eo', 'mp', 'no', 'np', 'er', 'nr'],
    source: powerData,
  },
});
const workOption = reactive({
  legend: {},
  tooltip: {
    trigger: 'axis',
  },
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
      name: '早峰能耗',
      smooth: true,
      connectNulls: true,
    },
    {
      type: 'line',
      name: '午峰能耗',
      smooth: true,
      connectNulls: true,
    },
  ],
  dataset: {
    dimensions: ['d', 'm', 'n'],
    source: workData,
  },
});

// 获取表单数据并发往后端，后端返回绘制图表所需数据，然后利用数据绘图
const onFinish = async (state: FormState) => {
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
  <!-- begin: 表单区域 -->
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
      name="factor"
      :rules="[{ required: formState.isPrimaryLoad, message: '一次负荷需填写倍率' }]"
    >
      <a-input-number v-model:value="formState.factor" />
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
  <!-- end: 表单区域 -->

  <!-- begin: 图表展示区域，以标签页分割 -->
  <a-tabs centered v-model:activeKey="activeKey">

    <a-tab-pane key="1" tab="功率图">

      <a-spin :spinning="spinning">
        <v-chart class="chart" :option="powerOption" autoresize />
      </a-spin>

    </a-tab-pane>

    <a-tab-pane key="2" tab="能耗图">

      <a-spin :spinning="spinning">
        <v-chart class="chart" :option="workOption" autoresize />
      </a-spin>

    </a-tab-pane>

  </a-tabs>
  <!-- end: 图表展示区域 -->
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
  height: calc(100vh - 360px);
}
</style>
