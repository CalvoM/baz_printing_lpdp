<script setup lang="ts">
import { ref } from 'vue'
import { useTauri } from '~/composables/useTauri'

interface PrintJob {
  rank: string
  owner: string
  jobNumber: string
  host: string
  filename: string
  size: string
}

const props = defineProps<{
  host: string
  port: number
  queue: string
}>()

const { lpdQueryQueue } = useTauri()

const username = ref<string>('')
const jobNumber = ref<string>('')
const output = ref<string>('')
const loading = ref(false)
const error = ref<string | null>(null)

const canQuery = computed(() => !!props.host && !!props.queue)

const parsedQueue = computed(() => {
  if (!output.value) return null
  const lines = output.value.split('\n').map(l => l.trimEnd())
  const statusLine = lines.find(l => l.trim() !== '') ?? ''
  const afterStatus = lines.slice(lines.indexOf(statusLine) + 1)

  const ownerRe = /^(\S+):\s+(\S+)\s+\[job\s+(\d+)\s+(\S+)\]$/
  const fileRe = /^\s{2,}(\S.*?)\s{2,}(\d+\s+bytes)\s*$/
  const jobs: PrintJob[] = []

  let i = 0
  while (i < afterStatus.length) {
    const m = ownerRe.exec(afterStatus[i].trim())
    if (m) {
      const [, owner, rank, jobNum, host] = m
      let j = i + 1
      while (j < afterStatus.length && afterStatus[j].trim() === '') j++
      const fm = fileRe.exec(afterStatus[j] ?? '')
      jobs.push({
        rank,
        owner,
        jobNumber: jobNum,
        host,
        filename: fm ? fm[1].trim() : '(unknown)',
        size: fm ? fm[2] : '',
      })
      i = j + 1
    } else {
      i++
    }
  }
  return { statusLine, jobs }
})

const statusIsReady = computed(() => {
  const s = parsedQueue.value?.statusLine ?? ''
  return s !== '' && !/not\s+ready/i.test(s)
})

async function viewQueue() {
  loading.value = true
  error.value = null
  output.value = ''
  try {
    output.value = await lpdQueryQueue(
      props.host,
      props.port,
      props.queue,
      username.value || undefined,
      jobNumber.value || undefined
    )
  } catch (e: any) {
    error.value = e?.message ?? String(e)
  } finally {
    loading.value = false
  }
}
</script>

<template>
  <div class="space-y-3">
    <div class="flex gap-3 items-end">
      <UFormField label="Username" class="flex-1">
        <UInput v-model="username" placeholder="Optional" class="w-full" />
      </UFormField>
      <UFormField label="Job #" class="w-32">
        <UInput v-model="jobNumber" placeholder="Optional" />
      </UFormField>
      <UButton
        :loading="loading"
        :disabled="!canQuery"
        color="primary"
        @click="viewQueue"
      >
        View Queue
      </UButton>
    </div>

    <p v-if="!canQuery" class="text-sm text-gray-400">
      Configure host and queue name above to query status.
    </p>
    <p v-if="error" class="text-sm text-red-500">{{ error }}</p>

    <div v-if="parsedQueue" class="flex items-center gap-2 px-3 py-2 rounded-md">
      <UIcon
        :name="statusIsReady ? 'i-lucide-check-circle' : 'i-lucide-alert-circle'"
        :class="statusIsReady ? 'text-emerald-500' : 'text-red-500'"
        class="size-4 shrink-0"
      />
      <span class="text-sm font-medium" :class="statusIsReady ? 'text-emerald-400' : 'text-red-400'">
        {{ parsedQueue.statusLine }}
      </span>
    </div>

    <ul v-if="parsedQueue && parsedQueue.jobs.length > 0" class="space-y-2">
      <li
        v-for="job in parsedQueue.jobs"
        :key="job.jobNumber"
        class="flex items-start gap-3 rounded-md px-3 py-2"
      >
        <span class="mt-0.5 inline-flex items-center justify-center size-7 rounded-full text-gray-300 text-xs font-bold shrink-0">
          {{ job.rank }}
        </span>
        <div class="min-w-0 flex-1">
          <p class="text-sm font-medium truncate" :title="job.filename">{{ job.filename }}</p>
          <p class="text-xs text-gray-400 mt-0.5">
            {{ job.size }}<span class="mx-1 text-gray-600">·</span>Job #{{ job.jobNumber }}<span class="mx-1 text-gray-600">·</span>{{ job.owner }}
          </p>
        </div>
      </li>
    </ul>

    <p v-else-if="parsedQueue && parsedQueue.jobs.length === 0" class="text-sm text-gray-400 text-center py-3">
      No jobs in queue.
    </p>
  </div>
</template>
