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
            <a-popconfirm v-if="userStore.role === 'admin'" title="Bạn chắc chắn muốn xoá các hóa đơn đã chọn?" ok-text="Xoá" cancel-text="Huỷ" @confirm="handleDeleteSelected" :disabled="!selectedRowKeys.length">
              <a-button danger :disabled="!selectedRowKeys.length" class="flex items-center gap-1">
                Xoá đã chọn ({{ selectedRowKeys.length }})
              </a-button>
            </a-popconfirm>
            <input type="file" accept="application/json" ref="invoiceFile" class="hidden" @change="importInvoices" />
            <!-- <a-button @click="triggerInvoiceImport">Nhập</a-button>
          <a-button @click="exportInvoices">Xuất</a-button> -->
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
        <!-- <a-table :columns="columns" :data-source="invoices" :loading="loading" :pagination="pagination" :row-selection="{ selectedRowKeys, onChange: onSelectChange }" row-key="id" size="small" @change="handleTableChange" bordered> -->
        <a-table :columns="columns" :data-source="invoices" :loading="loading" :pagination="false" row-key="id" size="small" @change="handleTableChange" bordered>
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
import dayjs from 'dayjs'
const { RestApi } = useApi()
const userStore = useUserStore()

const param = ref({
  // page: 1,
  // limit: 10,
  from: dayjs().format('DD/MM/YYYY'),
  to: dayjs().format('DD/MM/YYYY'),
  code: '',
  deleted:false
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
  // current: param.value.page,
  // pageSize: param.value.limit,
  total: summary.value.totalInvoices,
  showSizeChanger: true,
  pageSizeOptions: ['1', '10', '20', '50', '100'],
  showTotal: (total) => `Tổng ${total} hóa đơn`,
  size: 'small'
}))

const fetchInvoices = async (paramSource) => {
  loading.value = true
  try {
    const { data } = await RestApi.invoices.list({ params: paramSource })
    invoices.value = data.value?.data?.invoices || []
    selectedRowKeys.value = invoices.value.map(inv => inv.id)
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
  // param.value.page = pager.current
  // param.value.limit = pager.pageSize == 1 ? summary.value.totalInvoices : pager.pageSize
  await fetchInvoices({ ...param.value })
}

const handleDateChange = async (dates) => {
  if (dates?.length === 2) {
    param.value.from = dates[0].format('DD/MM/YYYY')
    param.value.to = dates[1].format('DD/MM/YYYY')
    // param.value.page = 1
    await fetchInvoices({ ...param.value })
  }
}

const onSearch = async () => {
  // param.value.page = 1
  param.value.code = search_text.value
  await fetchInvoices({ ...param.value })
}


const calculateInvoiceTotal = (invoice) =>
  invoice.items.reduce((sum, item) => sum + item.price * item.quantity, 0)

const viewDetail = (invoice) => {
  selectedInvoice.value = invoice
  detailVisible.value = true
}


const handleDeleteSelected = async () => {
  if (!selectedRowKeys.value.length) return

  const chunkSize = 10
  const total = selectedRowKeys.value.length
  const chunks = []

  // Chia thành các mảng nhỏ 10 phần tử
  for (let i = 0; i < total; i += chunkSize) {
    chunks.push(selectedRowKeys.value.slice(i, i + chunkSize))
  }

  try {
    for (const chunk of chunks) {
      await RestApi.invoices.delete({ params: { id: chunk.join(',') } })
    }
    message.success(`Đã xoá ${total} hóa đơn thành công`)
    selectedRowKeys.value = []
    await fetchInvoices({ ...param.value })
  } catch (e) {
    message.error('Không thể xoá hàng loạt!')
  }
}

const invoiceFile = ref(null)
const triggerInvoiceImport = () => {
  invoiceFile.value?.click()
}

const importInvoices = async (e) => {
  const file = e.target.files[0]
  if (!file) return
  try {
    const text = await file.text()
    const json = JSON.parse(text)
    const { data } = await RestApi.invoices.import({ body: json })
    if (data.value?.status === 'success') {
      message.success('Nhập hóa đơn thành công!')
      await fetchInvoices({ ...param.value })
    } else {
      throw new Error(data.value?.message || 'Nhập thất bại')
    }
  } catch (err) {
    message.error(err.message || 'Nhập thất bại')
  } finally {
    e.target.value = ''
  }
}

const exportInvoices = async () => {
  try {
    const { data } = await RestApi.invoices.export()
    const blob = new Blob([JSON.stringify(data.value?.data || data.value, null, 2)], { type: 'application/json' })
    const url = URL.createObjectURL(blob)
    const link = document.createElement('a')
    link.href = url
    link.download = 'invoices.json'
    link.click()
    URL.revokeObjectURL(url)
  } catch (err) {
    message.error('Xuất thất bại')
  }
}

const formatCurrency = (val) => {
  return new Intl.NumberFormat('vi-VN', {
    style: 'currency',
    currency: 'VND',
    currencyDisplay: 'narrowSymbol' // sẽ là ₫
  })
    .format(val)
    .replace(/[₫\s]/g, ''); // xóa ₫ và khoảng trắng
}
const formatDate = (val) =>
  dayjs(val).format('DD/MM/YYYY')

const formatDateTime = (val) =>
  dayjs(val).format('HH:mm DD/MM/YYYY')

const generatePrintableHtml = (invoice, store = {}) => {
  const createdAt = new Date(invoice.createdAt).toLocaleString('vi-VN')

  return `
  <div style="font-family: monospace; font-size: 16px; width: 100%;">
    ${store.logoUrl ? `<div style="text-align:center;"><img src="${store.logoUrl}" style="max-height: 60px; margin-bottom: 5px;" /></div>` : ''}
    <div style="text-align:center; font-weight:bold;">${store.storeName || 'CỬA HÀNG'}</div>
    ${store.address ? `<div style="text-align:center;">Địa chỉ: ${store.address}</div>` : ''}
    ${store.phone ? `<div style="text-align:center;">Điện thoại: ${store.phone}</div>` : ''}
    <hr/>
    <div style="text-align:center; font-weight:bold; margin: 5px 0;">HÓA ĐƠN BÁN HÀNG</div>
    <div style="text-align:center;">Số HĐ: <b>${invoice.code}</b></div>
    <div style="text-align:center; padding-bottom: 20px;">Ngày: ${createdAt}</div>

    <table style="width:100%; font-size:16px; border-collapse: collapse;" border="1">
      <thead>
        <tr>
          <th style="text-align:left;  padding:3px;">Tên SP</th>
          <th style="text-align:center;  padding:3px;">ĐG</th>
          <th style="text-align:center;  padding:3px;">SL</th>
          <th style="text-align:right;  padding:3px;">TT</th>
        </tr>
      </thead>
      <tbody>
        ${invoice.items.map((item, index) => `
          <tr>
            <td style="word-break: break-word; padding:3px;">${item.name}</td>
            <td style="text-align:center; white-space:nowrap; padding:3px;">${formatCurrency(item.price)}</td>
            <td style="text-align:center; padding:3px;">${item.quantity}</td>
            <td style="text-align:right; white-space:nowrap; padding:3px;">${formatCurrency(item.price * item.quantity)}</td>
          </tr>
        `).join('')}
      </tbody>
    </table>
    <div style="text-align:right; font-weight:bold; white-space:nowrap; font-size:16px;">
      Tổng cộng: ${formatCurrency(calculateInvoiceTotal(invoice))}
    </div>
    ${invoice.note ? `<div style="font-size:16px;">Ghi chú: ${invoice.note}</div>` : ''}
    <div style="font-weight:bold; white-space:nowrap;text-align:center; margin-top:10px; font-size:16px; padding-top: 10px;">Cảm ơn quý khách!</div>
  </div>
`;

}

const printInvoice = async (invoice) => {
  const settingRes = await RestApi.setting.get()
  const setting = settingRes?.data?.value?.data || {}

  const html = generatePrintableHtml(invoice, setting)

  const iframe = document.getElementById('print-frame')
  const doc = iframe.contentWindow.document
  doc.open()
  doc.write(`
  <html>
    <head>
      <title>Hóa đơn</title>
      <style>
        @page { margin: 0 }
        body {
          font-family: monospace;
          font-size: 13px;
          padding: 10px;
          margin: 0;
        }
        hr {
          border: none;
          border-top: 1px dashed black;
          margin: 10px 0;
        }
        .text-center { text-align: center }
        .bold { font-weight: bold }
        .row { display: flex; justify-content: space-between }
        table {
          width: 100%;
          border-collapse: collapse;
          table-layout: fixed;
        }
        th, td {
          word-break: break-word;
          padding: 2px 0;
        }
        th:nth-child(1), td:nth-child(1) { width: 40%; text-align: left; }
        th:nth-child(2), td:nth-child(2) { width: 20%; text-align: right; }
        th:nth-child(3), td:nth-child(3) { width: 15%; text-align: right; }
        th:nth-child(4), td:nth-child(4) { width: 25%; text-align: right; }
      </style>
    </head>
    <body onload="window.print()">
      ${html}
    </body>
  </html>
  `)
  doc.close()
}
const exportToExcel = async () => {
  const mod = await import('xlsx')
  const XLSX = mod.default || mod

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
