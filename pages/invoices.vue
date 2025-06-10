<template>
  <div class="min-h-screen bg-white p-4">
    <div class="">
      <!-- Header -->
      <div class="mb-6">
        <h1 class="text-xl font-bold text-gray-800">Quản lý hóa đơn</h1>
        <p class="text-gray-600 text-sm">Danh sách và thống kê các hóa đơn bán hàng</p>
      </div>

      <!-- Toolbar -->
      <div class="bg-white p-3 rounded-lg shadow-sm mb-4">
        <div class="flex flex-col md:flex-row md:items-center justify-between gap-2">
          <div class="flex flex-col md:flex-row gap-2 w-full">
            <a-range-picker
              v-model:value="dateRange"
              format="DD/MM/YYYY"
              class="w-full"
              @change="handleDateChange"
            />
            <a-input-search
              v-model:value="search_text"
              placeholder="Tìm kiếm theo mã hóa đơn..."
              enter-button
              allow-clear
              class="w-full"
              @search="onSearch"
            />
          </div>

          <div class="flex items-center gap-2">
            <a-popconfirm 
              title="Bạn chắc chắn muốn xoá các hóa đơn đã chọn?" 
              ok-text="Xoá" 
              cancel-text="Huỷ" 
              @confirm="handleDeleteSelected"
              :disabled="!selectedRowKeys.length"
            >
              <a-button danger :disabled="!selectedRowKeys.length" class="flex items-center gap-1">
                Xoá đã chọn ({{ selectedRowKeys.length }})
              </a-button>
            </a-popconfirm>
          </div>
        </div>
      </div>

      <!-- Stats Summary -->
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

      <!-- Table -->
      <div class="bg-white">
        <a-table 
          :columns="columns" 
          :data-source="invoices" 
          :loading="loading" 
          :pagination="pagination"
          :row-selection="{ selectedRowKeys, onChange: onSelectChange }"
          row-key="id" 
          size="small" 
          @change="handleTableChange"
          bordered
        >
          <template #bodyCell="{ column, record }">
            <template v-if="column.key === 'createdAt'">
              {{ formatDate(record.createdAt) }}
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

            <template v-if="column.key === 'actions'">
              <div class="flex gap-1">
                <a-button type="text" size="small" @click="viewDetail(record)">
                  Chi tiết
                </a-button>
                <a-popconfirm 
                  title="Bạn chắc chắn muốn xoá?" 
                  ok-text="Xoá" 
                  cancel-text="Huỷ" 
                  @confirm="() => handleDelete(record.id)"
                >
                  <a-button type="text" size="small" danger class="hover:bg-red-50 px-1">
                    Xoá
                  </a-button>
                </a-popconfirm>
              </div>
            </template>
          </template>
        </a-table>
      </div>
    </div>

    <!-- Detail Modal -->
    <a-modal 
      v-model:visible="detailVisible" 
      :title="'Chi tiết hóa đơn ' + selectedInvoice?.code" 
      width="700px"
      :footer="null"
    >
      <div v-if="selectedInvoice" class="space-y-4">
        <div class="flex justify-between items-center">
          <div>
            <div class="font-semibold">Mã hóa đơn:</div>
            <div>{{ selectedInvoice.code }}</div>
            <div class="font-semibold mt-2">Ngày tạo:</div>
            <div>{{ formatDateTime(selectedInvoice.createdAt) }}</div>
          </div>
          <div class="text-right">
            <div class="font-semibold">Tổng cộng:</div>
            <div class="text-xl font-bold">{{ formatCurrency(calculateInvoiceTotal(selectedInvoice)) }}</div>
          </div>
        </div>

        <a-divider />

        <div>
          <div class="font-semibold mb-2">Sản phẩm:</div>
          <a-table 
            :columns="detailColumns" 
            :data-source="selectedInvoice.items" 
            size="small"
            :pagination="false"
          >
            <template #bodyCell="{ column, record }">
              <template v-if="column.key === 'price'">
                {{ formatCurrency(record.price) }}
              </template>
              <template v-if="column.key === 'total'">
                {{ formatCurrency(record.price * record.quantity) }}
              </template>
            </template>
          </a-table>
        </div>

        <div v-if="selectedInvoice.note">
          <div class="font-semibold">Ghi chú:</div>
          <div>{{ selectedInvoice.note }}</div>
        </div>
      </div>
    </a-modal>
  </div>
</template>

<script setup>
import dayjs from 'dayjs'
const { RestApi } = useApi()
const param = ref({ 
  page: 1, 
  limit: 10,
  from: dayjs().startOf('month').format('DD/MM/YYYY'),
  to: dayjs().endOf('month').format('DD/MM/YYYY'),
  code: ''
})
const dateRange = ref([
  dayjs(param.value.from, 'DD/MM/YYYY'),
  dayjs(param.value.to, 'DD/MM/YYYY')
])
const search_text = ref("")
const invoices = ref([])
const summary = ref({
  totalInvoices: 0,
  totalAmount: 0,
  totalQuantity: 0
})
const loading = ref(false)
const detailVisible = ref(false)
const selectedInvoice = ref(null)
const selectedRowKeys = ref([])

// Columns
const columns = [
  {
    title: 'Mã hóa đơn',
    dataIndex: 'code',
    key: 'code',
    width: '180px'
  },
  {
    title: 'Ngày tạo',
    dataIndex: 'createdAt',
    key: 'createdAt',
    width: '150px'
  },
  {
    title: 'Sản phẩm',
    key: 'items'
  },
  {
    title: 'Tổng tiền',
    key: 'total',
    align: 'right',
    width: '150px'
  },
  {
    title: 'Hành động',
    key: 'actions',
    width: '120px',
    align: 'center'
  }
]

const detailColumns = [
  {
    title: 'Tên sản phẩm',
    dataIndex: 'name',
    key: 'name'
  },
  {
    title: 'Số lượng',
    dataIndex: 'quantity',
    key: 'quantity',
    align: 'center'
  },
  {
    title: 'Đơn giá',
    key: 'price',
    align: 'right'
  },
  {
    title: 'Thành tiền',
    key: 'total',
    align: 'right'
  }
]

// Computed
const pagination = computed(() => ({
  current: param.value.page,
  pageSize: param.value.limit,
  total: summary.value.totalInvoices,
  showSizeChanger: true,
  pageSizeOptions: ['1','10', '20', '50', '100'],
  showTotal: (total) => `Tổng ${total} hóa đơn`,
  size: 'small'
}))

// Methods
const fetchInvoices = async (param_source = null) => {
  loading.value = true
  try {
    const params = param_source || param.value
    const { data } = await RestApi.invoices.list({ params })
    invoices.value = data.value?.data?.invoices || []
    summary.value = {
      totalInvoices: data.value?.data?.total || 0,
      totalAmount: data.value?.data?.totalAmount || 0,
      totalQuantity: Object.values(data.value?.data?.productStats || {}).reduce((sum, item) => sum + item.quantity, 0)
    }
  } finally {
    loading.value = false
  }
}

const handleTableChange = async (paginator) => {
  param.value.page = paginator.current
  param.value.limit = paginator.pageSize
  await fetchInvoices({...param.value})
}

const handleDateChange = async (dates) => {
  if (dates && dates.length === 2) {
    param.value.from = dates[0].format('DD/MM/YYYY')
    param.value.to = dates[1].format('DD/MM/YYYY')
    param.value.page = 1
    await fetchInvoices({...param.value})
  }
}

const onSearch = async () => {
  param.value.page = 1
  param.value.code = search_text.value
  await fetchInvoices({...param.value})
}

const onSelectChange = (selectedKeys) => {
  selectedRowKeys.value = selectedKeys
}

const calculateInvoiceTotal = (invoice) => {
  return invoice.items.reduce((sum, item) => sum + (item.price * item.quantity), 0)
}

const viewDetail = (invoice) => {
  selectedInvoice.value = invoice
  detailVisible.value = true
}

const handleDelete = async (id) => {
  try {
    await RestApi.invoices.delete({ params: { id } })
    message.success('Xoá hóa đơn thành công!')
    await fetchInvoices({...param.value})
    selectedRowKeys.value = selectedRowKeys.value.filter(key => key !== id)
  } catch (error) {
    message.error('Xoá hóa đơn không thành công!')
  }
}

const handleDeleteSelected = async () => {
  if (!selectedRowKeys.value.length) return

  try {
    await RestApi.invoices.delete({ params: { id: selectedRowKeys.value.join(',') } })
    message.success(`Đã xoá ${selectedRowKeys.value.length} hóa đơn`)
    selectedRowKeys.value = []
    await fetchInvoices({...param.value})
  } catch (error) {
    message.error('Xoá hóa đơn không thành công!')
  }
}
const formatCurrency = (value) => {
  return new Intl.NumberFormat('vi-VN', { 
    style: 'currency', 
    currency: 'VND' 
  }).format(value)
}

const formatDate = (dateString) => {
  return dayjs(dateString).format('DD/MM/YYYY')
}
const formatDateTime = (dateString) => {
  return dayjs(dateString).format('HH:mm DD/MM/YYYY')
}
await fetchInvoices({...param.value})
</script>