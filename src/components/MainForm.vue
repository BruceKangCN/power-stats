<script setup lang="ts">
import { reactive, defineEmits} from 'vue';
import { open } from '@tauri-apps/api/dialog';

defineEmits(['submit', 'requireOpenDrawer']);

export interface FormState {
  ratedCapacity: number; // 额定容量
  isPrimaryLoad: boolean; // 是否为一次负荷
  factor?: number; // 倍率
  filepath: string; // 数据文件路径
}

const formState = reactive<FormState>({
  ratedCapacity: 0.0,
  isPrimaryLoad: true,
  factor: 1.0,
  filepath: '',
});

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
  <AForm
    :model="formState"
    name="form"
    :label-col="{ span: 6 }"
    :wrapper-col="{ span: 16 }"
    autocomplete="off"
    @finish="$emit('submit', formState)"
  >

    <AFormItem
      label="变压器额定容量"
      name="ratedCapacity"
      :rules="[{ required: true, message: '请输入额定容量' }]"
    >
      <AInputNumber v-model:value="formState.ratedCapacity" />
    </AFormItem>

    <AFormItem
      label="是否为一次负荷"
      name="isPrimaryLoad"
      :rules="[{ required: true, message: '请选择是否为一次负荷' }]"
    >
      <ASwitch v-model:checked="formState.isPrimaryLoad" />
    </AFormItem>

    <AFormItem
      label="倍率"
      name="factor"
      :rules="[{ required: formState.isPrimaryLoad, message: '一次负荷需填写倍率' }]"
    >
      <AInputNumber v-model:value="formState.factor" />
    </AFormItem>

    <AFormItem
      label="数据源"
      name="filepath"
      :rules="[{ required: true, message: '请选择数据文件' }]"
    >
      <ASpace>
        <AInput
          id="file-selector"
          v-model:value="formState.filepath"
          placeholder="文件路径"
        />
        <AButton @click="openFile">...</AButton>
      </ASpace>
    </AFormItem>

    <AFormItem
      :wrapper-col="{ offset: 6, span: 16}"
    >
      <AButton
        @click="$emit('requireOpenDrawer')"
        style="margin-right: 10px"
      >
        选项
      </AButton>
      <AButton type="primary" html-type="submit">确认</AButton>
    </AFormItem>

  </AForm>
</template>
