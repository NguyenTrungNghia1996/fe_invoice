<template>
  <div class="min-h-screen bg-white p-4">
    <div class="mb-6">
      <h1 class="text-xl font-bold text-gray-800">Báo cáo hóa đơn</h1>
      <p class="text-gray-600 text-sm">Thống kê và danh sách hóa đơn</p>
    </div>

    <div class="bg-white p-3 rounded-lg shadow-sm mb-4">
      <div class="flex flex-col md:flex-row md:items-center gap-2">
        <a-input-search
          v-model:value="param.code"
          placeholder="Mã hóa đơn"
          enter-button
          allow-clear
          class="w-full md:w-60"
          @search="onSearch"
        />
        <a-range-picker
          v-model:value="dateRange"
          format="DD/MM/YYYY"
          @change="handleDateChange"
        />
        <a-select v-model:value="param.shift" class="w-32" placeholder="Ca">
          <a-select-option value="">Tất cả</a-select-option>
          <a-select-option value="morning">Sáng</a-select-option>
          <a-select-option value="afternoon">Chiều</a-select-option>
        </a-select>
        <a-select v-model:value="param.deleted" class="w-32" placeholder="Trạng thái">
          <a-select-option value="">Tất cả</a-select-option>
          <a-select-option :value="false">Chưa xoá</a-select-option>
          <a-select-option :value="true">Đã xoá</a-select-option>
        </a-select>
        <a-button type="primary" @click="showAllInvoices">Hiển thị toàn bộ</a-button>
      </div>
    </div>

    <div class="grid grid-cols-1 md:grid-cols-3 gap-4 mb-4">
      <div class="bg-white p-4 rounded-lg shadow-sm border-l-4 border-blue-500">
        <div class="text-gray-500 text-sm">Tổng số hóa đơn</div>
        <div class="text-2xl font-bold">{{ summary.totalInvoices }}</div>
      </div>
      <div class="bg-white p-4 rounded-lg shadow-sm border-l-4 border-green-500">
        <div class="text-gray-500 text-sm">Tổng doanh thu</div>
        <div class="text-2xl font-bold">{{ formatCurrency(summary.totalAmount) }}</div>
      </div>
      <div class="bg-white p-4 rounded-lg shadow-sm border-l-4 border-purple-500">
        <div class="text-gray-500 text-sm">Số lượng sản phẩm</div>
        <div class="text-2xl font-bold">{{ summary.totalQuantity }}</div>
      </div>
    </div>

    <div class="bg-white p-4 rounded-lg shadow-sm mb-4">
      <div class="font-semibold text-gray-700 mb-2">Thống kê sản phẩm:</div>
      <ProductStats :stats="summary.productStats" />
    </div>

    <a-table
      :columns="columns"
      :data-source="invoices"
      :loading="loading"
      :pagination="pagination"
      row-key="id"
      size="small"
      @change="handleTableChange"
      bordered
    >
      <template #bodyCell="{ column, record }">
        <template v-if="column.key === 'createdAt'">
          {{ formatDateTime(record.createdAt) }}
        </template>
        <template v-if="column.key === 'deletedAt'">
          {{ record.deletedAt ? formatDateTime(record.deletedAt) : '' }}
        </template>
        <template v-if="column.key === 'createdBy'">
          {{ record.createdBy?.username || '' }}
        </template>
        <template v-if="column.key === 'deletedBy'">
          {{ record.deletedBy?.username || '' }}
        </template>
        <template v-if="column.key === 'items'">
          <div class="flex flex-col gap-1">
            <div v-for="(item, index) in record.items" :key="index" class="text-sm">
              {{ item.name }} (x{{ item.quantity }}) - {{ formatCurrency(item.price * item.quantity) }}
            </div>
          </div>
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
  page: 1,
  limit: 10,
  from: dayjs().format('DD/MM/YYYY'),
  to: dayjs().format('DD/MM/YYYY'),
  code: '',
  shift: '',
  deleted: ''
})

const dateRange = ref([
  dayjs(param.value.from, 'DD/MM/YYYY'),
  dayjs(param.value.to, 'DD/MM/YYYY')
])

const invoices = ref([])
const summary = ref({
  totalInvoices: 0,
  totalAmount: 0,
  totalQuantity: 0,
  productStats: {}
})
const loading = ref(false)

const columns = [
  { title: 'Mã hóa đơn', dataIndex: 'code', key: 'code', width: '180px' },
  { title: 'Thời gian tạo', dataIndex: 'createdAt', key: 'createdAt', width: '170px' },
  { title: 'Thời gian xoá', dataIndex: 'deletedAt', key: 'deletedAt', width: '170px' },
  { title: 'Người tạo', key: 'createdBy', dataIndex: 'createdBy', width: '120px' },
  { title: 'Người xoá', key: 'deletedBy', width: '120px' },
  { title: 'Sản phẩm', key: 'items' },
  { title: 'Tổng tiền', key: 'total', align: 'right', width: '150px' }
]

const pagination = computed(() => ({
  current: param.value.page,
  pageSize: param.value.limit,
  total: summary.value.totalInvoices,
  showSizeChanger: true,
  pageSizeOptions: ['10', '20', '50', '100'],
  showTotal: total => `Tổng ${total} hóa đơn`,
  size: 'small'
}))

const fetchInvoices = async (args) => {
  loading.value = true
  try {
    const { data } = await RestApi.invoices.list({ params: args })
    invoices.value = data.value?.data?.invoices || []
    summary.value = {
      totalInvoices: data.value?.data?.total || 0,
      totalAmount: data.value?.data?.totalAmount || 0,
      totalQuantity: Object.values(data.value?.data?.productStats || {}).reduce((s, p) => s + p.quantity, 0),
      productStats: data.value?.data?.productStats || {}
    }
  } finally {
    loading.value = false
  }
}

watchEffect(() => fetchInvoices({ ...param.value }))

const handleTableChange = async pager => {
  param.value.page = pager.current
  param.value.limit = pager.pageSize
  await fetchInvoices({ ...param.value })
}

const handleDateChange = async dates => {
  if (dates?.length === 2) {
    param.value.from = dates[0].format('DD/MM/YYYY')
    param.value.to = dates[1].format('DD/MM/YYYY')
    param.value.page = 1
    await fetchInvoices({ ...param.value })
  }
}

const onSearch = async () => {
  param.value.page = 1
  await fetchInvoices({ ...param.value })
}

const showAllInvoices = async () => {
  param.value.page = 1
  param.value.limit = summary.value.totalInvoices || 0
  await fetchInvoices({ ...param.value })
}

const calculateInvoiceTotal = invoice => invoice.items.reduce((sum, item) => sum + item.price * item.quantity, 0)

const formatCurrency = val => new Intl.NumberFormat('vi-VN', {
  style: 'currency',
  currency: 'VND',
  currencyDisplay: 'narrowSymbol'
}).format(val).replace(/[₫\s]/g, '')

const formatDate = val => dayjs(val).format('DD/MM/YYYY')
const formatDateTime = val => dayjs(val).format('HH:mm DD/MM/YYYY')
</script>
