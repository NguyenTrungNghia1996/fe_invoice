<script setup>
const { RestApi } = useApi()


const invoice = ref({ items: [], note: '' })
const selectedProduct = ref(null)
const productOptions = ref([])
const productQuantity = ref(1)
const submitting = ref(false)
const printData = ref(null)
const storeInfo = ref(null)

const itemColumns = [
  { title: 'Sản phẩm', dataIndex: 'name', key: 'name', ellipsis: true},
  { title: 'Số lượng', key: 'quantity', width: 120 },
  { title: 'Đơn giá', key: 'price', align: 'right', width: 120 , ellipsis: true},
  { title: 'Thành tiền', key: 'total', align: 'right', width: 120, ellipsis: true},
  { title: 'Hành Động', key: 'action', width: 100, align: 'center' }
]

const fetchProducts = async () => {
  try {
    const { data } = await RestApi.products.list()
    productOptions.value = data.value?.data?.products || []
  } catch (err) {
    message.error('Không thể tải sản phẩm')
  }
}

// Loại bỏ dấu tiếng Việt
function removeVietnameseTones(str) {
  return str.normalize('NFD')
    .replace(/[\u0300-\u036f]/g, '')
    .replace(/đ/g, 'd').replace(/Đ/g, 'D')
    .toLowerCase()
}

// Lọc sản phẩm không dấu
const filterProductOption = (input, option) => {
  return removeVietnameseTones(option.label).includes(removeVietnameseTones(input))
}

// Dữ liệu hiển thị trong select
const selectOptions = computed(() =>
  productOptions.value.map(p => ({
    label: `${p.name} - ${formatCurrency(p.price)}`,
    value: p.id
  }))
)

const handleAddProduct = () => {
  const product = productOptions.value.find(p => p.id === selectedProduct.value)
  if (!product) return

  const existing = invoice.value.items.find(item => item.productId === product.id)
  if (existing) {
    existing.quantity += productQuantity.value
  } else {
    invoice.value.items.push({
      productId: product.id,
      name: product.name,
      price: product.price,
      quantity: productQuantity.value
    })
  }

  selectedProduct.value = null
  productQuantity.value = 1
}

const removeItem = (index) => {
  invoice.value.items.splice(index, 1)
}

const calculateTotal = (items) => {
  return items.reduce((sum, item) => sum + item.price * item.quantity, 0)
}

const resetForm = () => {
  invoice.value = { items: [], note: '' }
  selectedProduct.value = null
  productQuantity.value = 1
  printData.value = null
}

const submitInvoice = async () => {
  if (!invoice.value.items.length) {
    message.warning('Vui lòng thêm sản phẩm')
    return
  }

  submitting.value = true
  try {
    const { data } = await RestApi.invoices.create({
      body: {
        items: invoice.value.items,
        note: invoice.value.note
      }
    })

    if (data.value?.status === 'success') {
      printData.value = data.value.data
      const { data: settingRes } = await RestApi.setting.get()
      storeInfo.value = settingRes.value?.data || {}
      message.success('Tạo hóa đơn thành công!')
      printInvoice()
      resetForm()
    } else {
      throw new Error('Tạo hóa đơn thất bại')
    }
  } catch (err) {
    message.error(err.message || 'Đã xảy ra lỗi')
  } finally {
    submitting.value = false
  }
}

const printInvoice = () => {
  if (!printData.value) return
  const html = generatePrintableHtml(printData.value, storeInfo.value)
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

const generatePrintableHtml = (invoice, store = {}) => {
  const items = invoice.items.map(item => `
    <div class="row">
      <span>${item.name} x${item.quantity}</span>
      <span>${formatCurrency(item.price * item.quantity)}</span>
    </div>
  `).join('')

  const createdAt = new Date(invoice.createdAt).toLocaleString('vi-VN')

return `
  ${store.logoUrl ? `<div style="text-align:center;"><img src="${store.logoUrl}" style="max-height: 80px; margin-bottom: 10px;" /></div>` : ''}
  <div style="text-align:center; font-weight:bold;">${store.storeName || 'CỬA HÀNG'}</div>
  ${store.address ? `<div style="text-align:center;">Địa chỉ: ${store.address}</div>` : ''}
  ${store.phone ? `<div style="text-align:center;">Điện thoại: ${store.phone}</div>` : ''}
  <hr />
  <div style="text-align:center; font-weight:bold;">HÓA ĐƠN BÁN HÀNG</div>
  <div style="text-align:center;">Số HĐ: <b>${invoice.code}</b></div>
  <div style="text-align:center;">Ngày ${createdAt}</div>
  <hr />
  <table style="width: 100%; border-collapse: collapse; table-layout: fixed;">
    <thead>
      <tr>
        <th style="text-align:left; width: 40%;">Tên SP</th>
        <th style="text-align:right; width: 20%;">Đơn giá</th>
        <th style="text-align:right; width: 15%;">SL</th>
        <th style="text-align:right; width: 25%;">Thành tiền</th>
      </tr>
    </thead>
    <tbody>
      ${invoice.items.map(item => `
        <tr>
          <td style="word-break: break-word;">${item.name}</td>
          <td style="text-align:right;">${formatCurrency(item.price)}</td>
          <td style="text-align:right;">${item.quantity}</td>
          <td style="text-align:right;">${formatCurrency(item.price * item.quantity)}</td>
        </tr>
      `).join('')}
    </tbody>
  </table>
  <hr />
  <div style="display: flex; justify-content: space-between; font-weight: bold;">
    <span>Tổng thanh toán:</span>
    <span>${formatCurrency(calculateTotal(invoice.items))}</span>
  </div>
  ${invoice.note ? `<div>Ghi chú: ${invoice.note}</div>` : ''}
  <div style="text-align:center; margin-top:10px;">Cảm ơn quý khách!</div>
`
}

const formatCurrency = (val) => {
  return new Intl.NumberFormat('vi-VN', {
    style: 'currency',
    currency: 'VND'
  }).format(val)
}

fetchProducts()
</script>

<template>
  <div class="min-h-screen bg-white p-4">
    <div class="mb-6">
      <h1 class="text-xl font-bold text-gray-800">Tạo hóa đơn mới</h1>
      <p class="text-gray-600 text-sm">Thêm sản phẩm và thông tin vào hóa đơn</p>
    </div>

    <div class="bg-white p-4 rounded-lg shadow-sm mb-4">
      <a-form layout="vertical" @submit.prevent>
        <a-form-item label="Chọn sản phẩm">
          <div class="flex gap-2">
            <a-select
              v-model:value="selectedProduct"
              show-search
              :options="selectOptions"
              :filter-option="filterProductOption"
              placeholder="Tìm sản phẩm"
              style="width: 100%"
              @keydown.enter.prevent="handleAddProduct"
            />
            <a-button type="primary" @click="handleAddProduct" :disabled="!selectedProduct">Thêm</a-button>
          </div>
        </a-form-item>

        <a-form-item label="Số lượng" v-if="selectedProduct">
          <a-input-number v-model:value="productQuantity" :min="1" style="width: 100%" />
        </a-form-item>

        <a-table
          class="mt-6"
          :columns="itemColumns"
          :data-source="invoice.items"
          size="small"
          :pagination="false"
          bordered
        >
          <template #bodyCell="{ column, record, index }">
            <template v-if="column.key === 'quantity'">
              <a-input-number v-model:value="record.quantity" :min="1" />
            </template>
            <template v-if="column.key === 'price'">
              {{ formatCurrency(record.price) }}
            </template>
            <template v-if="column.key === 'total'">
              {{ formatCurrency(record.price * record.quantity) }}
            </template>
            <template v-if="column.key === 'action'">
              <a-button type="text" danger @click="removeItem(index)">Xóa</a-button>
            </template>
          </template>
        </a-table>

        <div class="text-right mt-4 font-semibold">
          Tổng cộng: {{ formatCurrency(calculateTotal(invoice.items)) }}
        </div>

        <a-form-item label="Ghi chú" class="mt-4">
          <a-textarea v-model:value="invoice.note" :rows="3" />
        </a-form-item>

        <div class="flex justify-end gap-2 mt-6">
          <a-button @click="resetForm">Hủy bỏ</a-button>
          <a-button
            @click="submitInvoice"
            type="primary"
            :loading="submitting"
            :disabled="!invoice.items.length"
          >
            Tạo hóa đơn
          </a-button>
        </div>
      </a-form>
    </div>

    <iframe id="print-frame" style="display:none;"></iframe>
  </div>
</template>
