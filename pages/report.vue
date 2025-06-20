<template>
  <div class="min-h-screen bg-white p-4">
    <div class="mb-6">
      <h1 class="text-xl font-bold text-gray-800">Báo cáo hóa đơn</h1>
    </div>
    <div class="bg-white p-3 rounded-lg shadow-sm mb-4">
      <div class="flex flex-col md:flex-row md:items-end gap-2">
        <a-range-picker v-model:value="dateRange" format="DD/MM/YYYY" @change="handleDateChange" />
        <a-select v-model:value="shift" :options="shiftOptions" class="w-40" placeholder="Ca" />
      </div>
    </div>
    <a-table :columns="columns" :data-source="invoices" :loading="loading" :pagination="pagination" row-key="id" bordered @change="handleTableChange">
      <template #bodyCell="{ column, record }">
        <template v-if="column.key === 'createdAt'">
          {{ formatDate(record.createdAt) }}
        </template>
        <template v-if="column.key === 'total'">
          {{ formatCurrency(calculateInvoiceTotal(record)) }}
        </template>
      </template>
    </a-table>
  </div>
</template>

<script setup>
import dayjs from 'dayjs'
const { RestApi } = useApi()

const param = ref({
  from: dayjs().format('DD/MM/YYYY'),
  to: dayjs().format('DD/MM/YYYY'),
  page: 1,
  limit: 10,
  deleted: false,
  shift: 'morning'
})

const dateRange = ref([dayjs(param.value.from, 'DD/MM/YYYY'), dayjs(param.value.to, 'DD/MM/YYYY')])
const shift = ref('morning')
const shiftOptions = [
  { label: 'Ca sáng', value: 'morning' },
  { label: 'Ca chiều', value: 'afternoon' },
  { label: 'Ca tối', value: 'evening' }
]

const invoices = ref([])
const total = ref(0)
const loading = ref(false)

const columns = [
  { title: 'Mã hóa đơn', dataIndex: 'code', key: 'code', width: '180px' },
  { title: 'Ngày tạo', dataIndex: 'createdAt', key: 'createdAt', width: '150px' },
  { title: 'Tổng tiền', key: 'total', align: 'right', width: '150px' }
]

const pagination = computed(() => ({
  current: param.value.page,
  pageSize: param.value.limit,
  total: total.value,
  showSizeChanger: true,
  pageSizeOptions: ['10', '20', '50', '100'],
  size: 'small'
}))

const fetchData = async () => {
  loading.value = true
  try {
    const { data } = await RestApi.invoices.list({ params: param.value })
    invoices.value = data.value?.data?.invoices || []
    total.value = data.value?.data?.total || 0
  } finally {
    loading.value = false
  }
}

const handleDateChange = async dates => {
  if (dates?.length === 2) {
    param.value.from = dates[0].format('DD/MM/YYYY')
    param.value.to = dates[1].format('DD/MM/YYYY')
    param.value.page = 1
    await fetchData()
  }
}

const handleTableChange = async pager => {
  param.value.page = pager.current
  param.value.limit = pager.pageSize
  await fetchData()
}

watch(shift, async val => {
  param.value.shift = val
  param.value.page = 1
  await fetchData()
})

const calculateInvoiceTotal = invoice => invoice.items.reduce((sum, i) => sum + i.price * i.quantity, 0)
const formatCurrency = val => new Intl.NumberFormat('vi-VN', { style: 'currency', currency: 'VND', currencyDisplay: 'narrowSymbol' }).format(val).replace(/[₫\s]/g, '')
const formatDate = val => dayjs(val).format('DD/MM/YYYY')

await fetchData()
</script>
