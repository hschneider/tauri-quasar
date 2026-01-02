<template>
  <q-page style="padding: 0 15px 15px 15px">
    <h1>Headline</h1>
    <h2>Subheadline</h2>
    <p>{{ lorem }}</p>
    <p>{{ lorem }}</p>
    <q-btn class="btn-md btn-black" label="Ping the Rust-Backend" color="primary" @click="CallBackend" unelevated/>

    <TestTable style="margin: 15px -15px 15px -15px"/>

    <q-card dark bordered class="bg-grey-3" style="color: #333; max-width: 300px">
      <q-card-section>
        <div class="text-h6">Our Changing Planet</div>
        <div class="text-subtitle2">by John Doe</div>
      </q-card-section>

      <q-separator inset />

      <q-card-section>
        {{ lorem }}
      </q-card-section>
    </q-card>
  </q-page>
</template>

<script setup>
import TestTable from 'components/TestTable.vue'
import { message } from '@tauri-apps/plugin-dialog'
import { invoke } from '@tauri-apps/api/core'

const lorem =
  'Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat.'

async function CallBackend() {
  let res = await invoke('ping')
  await message(res, {
    title: 'Greeting',
    kind: 'info',
  })
}
</script>
