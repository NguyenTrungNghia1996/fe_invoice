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
            <a-input-search v-model:value="search_text" placeholder="Tìm kiếm theo mã hóa đơn..." enter-button allow-clear class="w-full" @search="onSearch" />
            <a-range-picker v-model:value="dateRange" format="DD/MM/YYYY" @change="handleDateChange" />
          </div>

          <div class="flex items-center gap-2">
          <a-popconfirm
            v-if="userStore.role === 'admin'"
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
            <a-button type="primary" @click="exportToExcel">Xuất excel</a-button>
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

      <!-- Product Stats -->
      <div class="bg-white p-4 rounded-lg shadow-sm mb-4">
        <div class="font-semibold text-gray-700 mb-2">Thống kê sản phẩm:</div>
        <ul class="text-sm space-y-1">
          <li v-for="(stat, key) in summary.productStats" :key="key">
            • {{ stat.name }}: {{ stat.quantity }} sản phẩm – {{ formatCurrency(stat.revenue) }}
          </li>
        </ul>
      </div>

      <!-- Table -->
      <div class="bg-white">
        <a-table :columns="columns" :data-source="invoices" :loading="loading" :pagination="pagination" :row-selection="{ selectedRowKeys, onChange: onSelectChange }" row-key="id" size="small" @change="handleTableChange" bordered>
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
                <a-button type="text" size="small" @click="printInvoice(record)">
                  In lại
                </a-button>
                <a-popconfirm
                  v-if="userStore.role === 'admin'"
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
    <a-modal v-model:open="detailVisible" :title="'Chi tiết hóa đơn ' + selectedInvoice?.code" width="700px" :footer="null">
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
          <a-table :columns="detailColumns" :data-source="selectedInvoice.items" size="small" :pagination="false">
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
    <iframe id="print-frame" style="display:none;"></iframe>
  </div>
</template>

<script setup>
import * as XLSX from 'xlsx'
import dayjs from 'dayjs'
const { RestApi } = useApi()
const userStore = useUserStore()

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
  totalQuantity: 0,
  productStats: {}
})
const loading = ref(false)
const detailVisible = ref(false)
const selectedInvoice = ref(null)
const selectedRowKeys = ref([])

const columns = [
  { title: 'Mã hóa đơn', dataIndex: 'code', key: 'code', width: '180px' },
  { title: 'Ngày tạo', dataIndex: 'createdAt', key: 'createdAt', width: '150px' },
  { title: 'Sản phẩm', key: 'items' },
  { title: 'Tổng tiền', key: 'total', align: 'right', width: '150px' },
  { title: 'Hành động', key: 'actions', width: '150px', align: 'center' }
]

const detailColumns = [
  { title: 'Tên sản phẩm', dataIndex: 'name', key: 'name' },
  { title: 'Số lượng', dataIndex: 'quantity', key: 'quantity', align: 'center' },
  { title: 'Đơn giá', key: 'price', align: 'right' },
  { title: 'Thành tiền', key: 'total', align: 'right' }
]

const pagination = computed(() => ({
  current: param.value.page,
  pageSize: param.value.limit,
  total: summary.value.totalInvoices,
  showSizeChanger: true,
  pageSizeOptions: ['10', '20', '50', '100'],
  showTotal: (total) => `Tổng ${total} hóa đơn`,
  size: 'small'
}))

const fetchInvoices = async (paramSource = null) => {
  loading.value = true
  try {
    const params = paramSource || param.value
    const { data } = await RestApi.invoices.list({ params })
    invoices.value = data.value?.data?.invoices || []
    summary.value = {
      totalInvoices: data.value?.data?.total || 0,
      totalAmount: data.value?.data?.totalAmount || 0,
      totalQuantity: Object.values(data.value?.data?.productStats || {}).reduce((sum, p) => sum + p.quantity, 0),
      productStats: data.value?.data?.productStats || {}
    }
  } finally {
    loading.value = false
  }
}

const handleTableChange = async (pager) => {
  param.value.page = pager.current
  param.value.limit = pager.pageSize
  await fetchInvoices({ ...param.value })
}

const handleDateChange = async (dates) => {
  if (dates?.length === 2) {
    param.value.from = dates[0].format('DD/MM/YYYY')
    param.value.to = dates[1].format('DD/MM/YYYY')
    param.value.page = 1
    await fetchInvoices({ ...param.value })
  }
}

const onSearch = async () => {
  param.value.page = 1
  param.value.code = search_text.value
  await fetchInvoices({ ...param.value })
}

const onSelectChange = (keys) => {
  selectedRowKeys.value = keys
}

const calculateInvoiceTotal = (invoice) =>
  invoice.items.reduce((sum, item) => sum + item.price * item.quantity, 0)

const viewDetail = (invoice) => {
  selectedInvoice.value = invoice
  detailVisible.value = true
}

const handleDelete = async (id) => {
  try {
    await RestApi.invoices.delete({ params: { id } })
    message.success('Xoá thành công!')
    selectedRowKeys.value = selectedRowKeys.value.filter(k => k !== id)
    await fetchInvoices({ ...param.value })
  } catch (e) {
    message.error('Không thể xoá!')
  }
}

const handleDeleteSelected = async () => {
  if (!selectedRowKeys.value.length) return
  try {
    await RestApi.invoices.delete({ params: { id: selectedRowKeys.value.join(',') } })
    message.success(`Đã xoá ${selectedRowKeys.value.length} hóa đơn`)
    selectedRowKeys.value = []
    await fetchInvoices({ ...param.value })
  } catch (e) {
    message.error('Không thể xoá hàng loạt!')
  }
}

const formatCurrency = (value) =>
  new Intl.NumberFormat('vi-VN', { style: 'currency', currency: 'VND' }).format(value)

const formatDate = (val) =>
  dayjs(val).format('DD/MM/YYYY')

const formatDateTime = (val) =>
  dayjs(val).format('HH:mm DD/MM/YYYY')

const printInvoice = async (invoice) => {
  const settingRes = await RestApi.setting.get()
  const setting = settingRes?.data?.value?.data || {}

  const html = `
<html>
  <head>
    <title>In hóa đơn</title>
    <style>
      @page { margin: 0 }
      html, body {
        margin: 0;
        font-family: monospace;
        font-size: 13px;
        padding: 10px;
      }
      hr {
        border: none;
        border-top: 1px dashed black;
        margin: 10px 0;
      }
      .text-center { text-align: center }
      .bold { font-weight: bold }
      .row { display: flex; justify-content: space-between }
    </style>
  </head>
  <body onload="window.print()">
    ${setting.logoUrl ? `<div class="text-center"><img src="${setting.logoUrl}" style="max-width:60px;margin:4px auto;"/></div>` : ''}
    <div class="text-center bold">${setting.storeName || 'CỬA HÀNG'}</div>
    <div class="text-center">Địa chỉ: ${setting.address || ''}</div>
    <div class="text-center">Điện Thoại: ${setting.phone || ''}</div>
    <hr />
    <div class="text-center bold">HÓA ĐƠN BÁN HÀNG</div>
    <div class="text-center">Hóa Đơn: ${invoice.code}</div>
    <div class="text-center">Ngày: ${formatDateTime(invoice.createdAt)}</div>
    <hr />
    <table style="width: 100%; border-collapse: collapse; table-layout: fixed;">
      
      <thead>
        <tr>
        <th style="text-align:left; width: 40%;">Tên SP</th>
        <th style="text-align:right; width: 10%;">SL</th>
        <th style="text-align:right; width: 25%;">Đơn giá</th>
        <th style="text-align:right; width: 25%;">Thành tiền</th>
      </tr>
      </thead>
      <tbody>
        ${invoice.items.map(i => `
          <tr>
            <td style="word-break: break-word;">${i.name}</td>
            <td style="text-align:right; word-break: break-word;">${i.quantity}</td>
            <td style="text-align:right; word-break: break-word;">${formatCurrency(i.price)}</td>
            <td style="text-align:right; word-break: break-word;">${formatCurrency(i.quantity * i.price)}</td>
          </tr>
        `).join('')}
      </tbody>
    </table>
    <hr />
    <div class="row bold">
      <span>Tổng thanh toán:</span>
      <span>${formatCurrency(calculateInvoiceTotal(invoice))}</span>
    </div>
    ${invoice.note ? `<div>Ghi chú: ${invoice.note}</div>` : ''}
    <div class="text-center" style="margin-top:10px;">Cảm ơn quý khách!</div>
  </body>
</html>
`;



  const iframe = document.getElementById('print-frame')
  const doc = iframe.contentWindow.document
  doc.open()
  doc.write(html)
  doc.close()
}
const exportToExcel = () => {

  const rows = []
  invoices.value.forEach((inv, i) => {
    rows.push({
      'STT': i + 1,
      'Mã hóa đơn': inv.code,
      'Ngày tạo': formatDateTime(inv.createdAt),
      'Sản phẩm': '',
      'Số lượng': '',
      'Đơn giá': '',
      'Thành tiền': '',
      'Tổng tiền': formatCurrency(calculateInvoiceTotal(inv)),
      'Ghi chú': inv.note || ''
    })

    // Các sản phẩm bên trong hóa đơn
    inv.items.forEach(item => {
      rows.push({
        'STT': '',
        'Mã hóa đơn': '',
        'Ngày tạo': '',
        'Sản phẩm': item.name,
        'Số lượng': item.quantity,
        'Đơn giá': item.price,
        'Thành tiền': item.price * item.quantity,
        'Tổng tiền': '',
        'Ghi chú': ''
      })
    })
  })

  const worksheet = XLSX.utils.json_to_sheet(rows, { skipHeader: false })
  const workbook = XLSX.utils.book_new()
  XLSX.utils.book_append_sheet(workbook, worksheet, 'Danh sách hóa đơn')
  XLSX.writeFile(workbook, 'Danh_sach_hoa_don.xlsx')
}
await fetchInvoices({ ...param.value })
</script>