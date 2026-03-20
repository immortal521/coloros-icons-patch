<template>
  <md-card>
    <h3>配置</h3>
    <md-select label="Channel" v-model="local.channel">
      <md-list-item value="stable">stable</md-list-item>
      <md-list-item value="beta">beta</md-list-item>
    </md-select>

    <md-text-field label="Runtime Dir" v-model="local.runtime_dir" />
    <md-text-field label="Temp Dir" v-model="local.temp_dir" />
    <md-text-field label="Target Dir" v-model="local.target_dir" />

    <md-button @click="save" raised>保存</md-button>
  </md-card>
</template>

<script setup lang="ts">
import { reactive } from 'vue'

const props = defineProps<{ config: any }>()
const local = reactive({ ...props.config })

const save = () => {}
</script>
