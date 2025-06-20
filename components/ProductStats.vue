<template>
  <div class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 gap-2">
    <div
      v-for="(stat, key) in stats"
      :key="key"
      class="bg-gray-50 rounded-lg p-3 shadow flex flex-col"
    >
      <div class="font-semibold text-gray-700 truncate">{{ stat.name }}</div>
      <div class="text-xs text-gray-500">{{ stat.quantity }} sản phẩm</div>
      <div class="mt-auto text-right font-semibold">
        {{ formatCurrency(stat.revenue) }}
      </div>
    </div>
  </div>
</template>

<script setup>
const props = defineProps({
  stats: {
    type: Object,
    default: () => ({}),
  },
})

const formatCurrency = (val) =>
  new Intl.NumberFormat('vi-VN', {
    style: 'currency',
    currency: 'VND',
    currencyDisplay: 'narrowSymbol',
  })
    .format(val)
    .replace(/[₫\s]/g, '')
</script>
