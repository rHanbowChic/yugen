<script setup lang="ts">
import type { HTMLAttributes } from "vue"
import { computed } from "vue"
import { SliderRange, SliderRoot, SliderThumb, SliderTrack } from "reka-ui"
import { cn } from "@/lib/utils"

const props = withDefaults(
  defineProps<{
    modelValue?: number
    min?: number
    max?: number
    step?: number
    class?: HTMLAttributes["class"]
  }>(),
  {
    modelValue: 0,
    min: 0,
    max: 100,
    step: 1,
  },
)

const emit = defineEmits<{
  "update:modelValue": [value: number]
}>()

const modelAsArray = computed<number[]>({
  get: () => [props.modelValue],
  set: ([value]) => {
    if (value !== undefined) {
      emit("update:modelValue", value)
    }
  },
})
</script>

<template>
  <SliderRoot
    v-model="modelAsArray"
    :min="props.min"
    :max="props.max"
    :step="props.step"
    :class="
      cn(
        'relative flex w-full touch-none select-none items-center',
        props.class,
      )
    "
  >
    <SliderTrack class="bg-primary/20 relative h-2 w-full grow overflow-hidden rounded-full">
      <SliderRange class="bg-primary absolute h-full" />
    </SliderTrack>
    <SliderThumb
      class="border-primary bg-background focus-visible:ring-ring block size-5 rounded-full border-2 shadow transition-colors focus-visible:outline-none focus-visible:ring-3 disabled:pointer-events-none disabled:opacity-50"
    />
  </SliderRoot>
</template>
