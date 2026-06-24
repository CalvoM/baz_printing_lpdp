<script setup lang="ts">
import type { Protocol } from '~/composables/usePrinterConfig'

const model = defineModel<Protocol>({ required: true })

const items = [
  { value: 'ipp', label: 'IPP', description: 'Internet Printing Protocol' },
  { value: 'lpd', label: 'LPD', description: 'Legacy — for older printers (RFC 1179)' },
]
</script>

<template>
  <URadioGroup
    v-model="model"
    legend="Protocol"
    :items="items"
    orientation="horizontal"
  />
</template>
