<script setup lang="ts">
import { ref, reactive, toRefs } from 'vue';
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

const props = defineProps<{
  powerData: unknown[],
  workData: unknown[],
  smooth: boolean,
  spinning: boolean,
}>();
const { powerData, workData, smooth } = toRefs(props);

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

/** 当前显示的标签页的 key */
const activeKey = ref('1');

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
      id: 'dataZoomXSlider',
      type: 'slider',
      xAxisIndex: [0],
    },
    {
      id: 'dataZoomYSlider',
      type: 'slider',
      yAxisIndex: [0],
    },
    {
      id: 'dataZoomXInside',
      type: 'inside',
      xAxisIndex: [0],
    },
    {
      id: 'dataZoomYInside',
      type: 'inside',
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
      smooth: smooth,
      connectNulls: true,
    },
    {
      type: 'line',
      name: '早峰瞬时功率',
      smooth: smooth,
      connectNulls: true,
    },
    {
      type: 'line',
      name: '午谷瞬时功率',
      smooth: smooth,
      connectNulls: true,
    },
    {
      type: 'line',
      name: '午峰瞬时功率',
        smooth: smooth,
        connectNulls: true,
    },
    {
      type: 'line',
      name: '晚谷余量',
      smooth: smooth,
      connectNulls: true,
    },
    {
      type: 'line',
      name: '午谷余量',
      smooth: smooth,
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
      id: 'dataZoomXSlider',
      type: 'slider',
      xAxisIndex: [0],
    },
    {
      id: 'dataZoomYSlider',
      type: 'slider',
      yAxisIndex: [0],
    },
    {
      id: 'dataZoomXInside',
      type: 'inside',
      xAxisIndex: [0],
    },
    {
      id: 'dataZoomYInside',
      type: 'inside',
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
      smooth: smooth,
      connectNulls: true,
    },
    {
      type: 'line',
      name: '午峰能耗',
      smooth: smooth,
      connectNulls: true,
    },
  ],
  dataset: {
    dimensions: ['d', 'm', 'n'],
    source: workData,
  },
});
</script>

<template>
  <ATabs centered v-model:activeKey="activeKey">

    <ATabPane key="1" tab="功率图">

      <ASpin :spinning="spinning">
        <VChart class="chart" :option="powerOption" autoresize />
      </ASpin>

    </ATabPane>

    <ATabPane key="2" tab="能耗图">

      <ASpin :spinning="spinning">
        <VChart class="chart" :option="workOption" autoresize />
      </ASpin>

    </ATabPane>

  </ATabs>
</template>

<style scoped>
.chart {
  width: 100%;
  height: calc(100vh - 360px);
}
</style>
