<template>
  <div class="min-h-screen bg-white p-4">
    <div class="">
      <!-- Header -->
      <div class="mb-6">
        <h1 class="text-xl font-bold text-gray-800">Quản lý sản phẩm</h1>
        <p class="text-gray-600 text-sm">Danh sách và quản lý các sản phẩm của bạn</p>
      </div>

      <!-- Toolbar -->
      <div class="bg-white p-3 rounded-lg shadow-sm mb-4">
        <div class="flex flex-col md:flex-row md:items-center justify-between gap-2">
          <a-input-search v-model:value="search_text" placeholder="Tìm kiếm sản phẩm..." enter-button allow-clear class="w-full md:w-80" @search="onSearch" />

          <div class="flex items-center gap-2">
            <a-popconfirm title="Bạn chắc chắn muốn xoá?" ok-text="Xoá" cancel-text="Huỷ" @confirm="handleDelete">
              <a-button danger :disabled="!selectedRowKeys.length" class="flex items-center gap-1">
                Xoá đã chọn
              </a-button>
            </a-popconfirm>
            <a-button type="primary" @click="openAddModal" class="flex items-center gap-1">
              Thêm sản phẩm
            </a-button>
          </div>
        </div>
      </div>

      <!-- Table -->
      <div class="bg-white ">
        <a-table :columns="columns" :data-source="products" :loading="loading" :pagination="pagination" :row-selection="{ selectedRowKeys, onChange: onSelectChange }" row-key="id" size="small" @change="handleTableChange" bordered>
          <template #bodyCell="{ column, record }">
            <template v-if="column.key === 'price'">
              {{ formatCurrency(record.price) }}
            </template>

            <template v-if="column.key === 'actions'">
              <div class="flex gap-1">
                <a-button type="text" size="small" @click="openEditModal(record)">
                  Sửa
                </a-button>

                <a-popconfirm title="Bạn chắc chắn muốn xoá?" ok-text="Xoá" cancel-text="Huỷ" @confirm="() => handleDeleteOne(record.id)">
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

    <!-- Add/Edit Modal -->
    <a-modal v-model:visible="modalVisible" :title="editingProduct ? 'Sửa sản phẩm' : 'Thêm sản phẩm mới'" @ok="submitForm" @cancel="resetForm" :confirm-loading="modalLoading" width="500px" :destroy-on-close="true">
      <a-form layout="vertical" :model="form">
        <a-form-item label="Tên sản phẩm" name="name" :rules="[{ required: true, message: 'Vui lòng nhập tên sản phẩm' }]">
          <a-input v-model:value="form.name" placeholder="Nhập tên sản phẩm" />
        </a-form-item>

        <a-form-item label="Giá (VND)" name="price" :rules="[{ required: true, message: 'Vui lòng nhập giá sản phẩm' }]">
          <a-input-number v-model:value="form.price" :min="0" :formatter="value => `${value}`.replace(/\B(?=(\d{3})+(?!\d))/g, ',')" :parser="value => value.replace(/\$\s?|(,*)/g, '')" style="width: 100%" />
        </a-form-item>
      </a-form>

      <template #footer>
        <a-button key="back" @click="resetForm" size="small">Huỷ</a-button>
        <a-button key="submit" type="primary" :loading="modalLoading" @click="submitForm" size="small">
          {{ editingProduct ? 'Cập nhật' : 'Thêm mới' }}
        </a-button>
      </template>
    </a-modal>
  </div>
</template>

<script setup>
const { RestApi } = useApi()
// State
const param = ref({ page: 1, limit: 10, search: '' })
const search_text = ref("")
const products = ref([])
const total = ref(0)
const loading = ref(false)
const modalLoading = ref(false)
const selectedRowKeys = ref([])

// Modal
const modalVisible = ref(false)
const editingProduct = ref(null)
const form = ref({ name: '', price: 0 })

// Columns
const columns = [
  {
    title: 'Tên sản phẩm',
    dataIndex: 'name',
    key: 'name',
    ellipsis: true
  },
  {
    title: 'Giá',
    dataIndex: 'price',
    key: 'price',
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

// Computed
const pagination = computed(() => ({
  current: param.value.page,
  pageSize: param.value.limit,
  total: total.value,
  showSizeChanger: true,
  pageSizeOptions: ['1', '10', '20', '50', '100'],
  showTotal: (total) => `Tổng ${total} sản phẩm`,
  size: 'small'
}))

// Methods
const fetchProducts = async (param_soure) => {
  loading.value = true
  try {
    const { data } = await RestApi.products.list({ params: param_soure })
    products.value = data.value?.data?.products || []
    total.value = data.value?.data?.total || 0
  } finally {
    loading.value = false
  }
}

const handleTableChange = async (paginator) => {
  param.value.page = paginator.current
  param.value.limit = paginator.pageSize
  await fetchProducts({ ...param.value })
}

const onSearch = async () => {
  param.value.page = 1
  param.value.search = search_text.value
  await fetchProducts({ ...param.value })
}

const onSelectChange = (selectedKeys) => {
  selectedRowKeys.value = selectedKeys
}

const openAddModal = () => {
  editingProduct.value = null
  form.value = { name: '', price: 0 }
  modalVisible.value = true
}

const openEditModal = (record) => {
  editingProduct.value = record
  Object.assign(form.value, {
    id: record.id,
    name: record.name,
    price: record.price || ''
  })
  modalVisible.value = true
}

const submitForm = async () => {
  modalLoading.value = true
  try {
    if (editingProduct.value) {
      const { data, error } = await RestApi.products.update({ body: { ...form.value } })
      if (data.value?.status === 'success') {
        message.success(data.value.message || 'Thêm mới thành công')
      } else {
        throw new Error(error.value?.data?.message || 'Thêm mới không thành công')
      }
      message.success('Cập nhật sản phẩm thành công!')
    } else {
      if (form.value) {
        delete form.value.id
      }
      const { data, error } = await RestApi.products.create({ body: { ...form.value } })
      if (data.value?.status === 'success') {
        message.success(data.value.message || 'Thêm mới thành công')
      } else {
        throw new Error(error.value?.data?.message || 'Thêm mới không thành công')
      }
    }
  } catch (error) {
    message.error(error.message || error.response?.data?.message || 'Đã xảy ra lỗi khi lưu thông tin')
  }
  finally {
    modalVisible.value = false
    await fetchProducts({ ...param.value })
    modalLoading.value = false
  }
}

const handleDelete = async () => {
  if (!selectedRowKeys.value.length) return

  modalLoading.value = true
  try {
    const { data, error } = await RestApi.products.delete({ params: { id: selectedRowKeys.value.join(',') } })
    if (data.value?.status === 'success') {
      message.success(`Đã xoá ${selectedRowKeys.value.length} sản phẩm`)
      selectedRowKeys.value = []
    } else {
      throw new Error(error.value?.data?.message || 'Thêm mới không thành công')
    }
  } catch (error) {
    message.error(error.message || error.response?.data?.message || 'Đã xảy ra lỗi khi lưu thông tin')
  }
  finally {
    await fetchProducts({ ...param.value })
    modalLoading.value = false
  }
}

const handleDeleteOne = async (id) => {
  modalLoading.value = true
  try {
    const { data, error } = await RestApi.products.delete({ params: { id: id } })
    if (data.value?.status === 'success') {
      message.success('Xoá sản phẩm thành công!')
    } else {
      throw new Error(error.value?.data?.message || 'Thêm mới không thành công')
    }
  } catch (error) {
    message.error(error.message || error.response?.data?.message || 'Đã xảy ra lỗi khi lưu thông tin')
  } finally {
    await fetchProducts({ ...param.value })
    modalLoading.value = false
  }
}

const resetForm = () => {
  modalVisible.value = false
  form.value = { name: '', price: 0 }
  editingProduct.value = null
}

const formatCurrency = (value) => {
  return new Intl.NumberFormat('vi-VN', {
    style: 'currency',
    currency: 'VND'
  }).format(value)
}

await fetchProducts({ ...param.value })
</script>