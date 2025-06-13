<template>
  <div class="min-h-screen bg-white p-4">
    <div>
      <div class="mb-6">
        <h1 class="text-xl font-bold text-gray-800">Quản lý người dùng</h1>
        <p class="text-gray-600 text-sm">Danh sách tài khoản hệ thống</p>
      </div>

      <div class="bg-white p-3 rounded-lg shadow-sm mb-4">
        <div class="flex flex-col md:flex-row md:items-center justify-between gap-2">
          <a-input-search v-model:value="search_text" placeholder="Tìm kiếm..." enter-button allow-clear class="w-full md:w-80" @search="onSearch" />
          <div class="flex items-center gap-2">
            <a-popconfirm
              v-if="userStore.role === 'admin'"
              title="Bạn chắc chắn muốn xoá?"
              ok-text="Xoá"
              cancel-text="Huỷ"
              @confirm="handleDelete"
            >
              <a-button danger :disabled="!selectedRowKeys.length" class="flex items-center gap-1">Xoá đã chọn</a-button>
            </a-popconfirm>
            <a-button v-if="userStore.role === 'admin'" type="primary" @click="openAddModal" class="flex items-center gap-1">Thêm người dùng</a-button>
          </div>
        </div>
      </div>

      <div class="bg-white">
        <a-table
          :columns="columns"
          :data-source="users"
          :loading="loading"
          :pagination="pagination"
          :row-selection="{ selectedRowKeys, onChange: onSelectChange }"
          row-key="id"
          size="small"
          @change="handleTableChange"
          bordered
        >
          <template #bodyCell="{ column, record }">
            <template v-if="column.key === 'actions'">
              <div class="flex gap-1">
                <a-button type="text" size="small" @click="openEditModal(record)">Sửa</a-button>
                <a-popconfirm
                  v-if="userStore.role === 'admin'"
                  title="Bạn chắc chắn muốn xoá?"
                  ok-text="Xoá"
                  cancel-text="Huỷ"
                  @confirm="() => handleDeleteOne(record.id)"
                >
                  <a-button type="text" size="small" danger class="hover:bg-red-50 px-1">Xoá</a-button>
                </a-popconfirm>
              </div>
            </template>
          </template>
        </a-table>
      </div>
    </div>

    <a-modal v-model:visible="modalVisible" :title="editingUser ? 'Sửa người dùng' : 'Thêm người dùng mới'" @ok="submitForm" @cancel="resetForm" :confirm-loading="modalLoading" width="500px" :destroy-on-close="true">
      <a-form layout="vertical" :model="form">
        <a-form-item label="Tên đăng nhập" name="username" :rules="[{ required: true, message: 'Vui lòng nhập tên đăng nhập' }]">
          <a-input v-model:value="form.username" placeholder="Nhập tên đăng nhập" />
        </a-form-item>
        <a-form-item label="Họ tên" name="hoten" :rules="[{ required: true, message: 'Vui lòng nhập họ tên' }]">
          <a-input v-model:value="form.hoten" placeholder="Nhập họ tên" />
        </a-form-item>
        <a-form-item v-if="!editingUser" label="Mật khẩu" name="password" :rules="[{ required: true, message: 'Vui lòng nhập mật khẩu' }]">
          <a-input-password v-model:value="form.password" placeholder="Nhập mật khẩu" />
        </a-form-item>
        <a-form-item label="Role" name="role" :rules="[{ required: true, message: 'Vui lòng nhập role' }]">
          <a-input v-model:value="form.role" placeholder="admin hoặc user" />
        </a-form-item>
      </a-form>
      <template #footer>
        <a-button key="back" @click="resetForm">Huỷ</a-button>
        <a-button key="submit" type="primary" :loading="modalLoading" @click="submitForm">{{ editingUser ? 'Cập nhật' : 'Thêm mới' }}</a-button>
      </template>
    </a-modal>
  </div>
</template>

<script setup>
const { RestApi } = useApi()
const userStore = useUserStore()
if (userStore.role !== 'admin') {
  await navigateTo('/create_invoices')
}

const param = ref({ page: 1, limit: 10, search: '' })
const search_text = ref('')
const users = ref([])
const total = ref(0)
const loading = ref(false)
const modalLoading = ref(false)
const selectedRowKeys = ref([])

const modalVisible = ref(false)
const editingUser = ref(null)
const form = ref({ username: '', hoten: '', password: '', role: '' })

const columns = [
  { title: 'Tên đăng nhập', dataIndex: 'username', key: 'username', ellipsis: true },
  { title: 'Họ tên', dataIndex: 'hoten', key: 'hoten', ellipsis: true },
  { title: 'Role', dataIndex: 'role', key: 'role', width: '120px', align: 'center' },
  { title: 'Hành động', key: 'actions', width: '120px', align: 'center' }
]

const pagination = computed(() => ({
  current: param.value.page,
  pageSize: param.value.limit,
  total: total.value,
  showSizeChanger: true,
  pageSizeOptions: ['10', '20', '50', '100'],
  showTotal: t => `Tổng ${t} người dùng`,
  size: 'small'
}))

const fetchUsers = async (paramSource) => {
  loading.value = true
  try {
    const { data } = await RestApi.users.list({ params: paramSource })
    users.value = data.value?.data?.users || []
    total.value = data.value?.data?.total || 0
  } finally {
    loading.value = false
  }
}

const handleTableChange = async (pager) => {
  param.value.page = pager.current
  param.value.limit = pager.pageSize
  await fetchUsers({ ...param.value })
}

const onSearch = async () => {
  param.value.page = 1
  param.value.search = search_text.value
  await fetchUsers({ ...param.value })
}

const onSelectChange = keys => {
  selectedRowKeys.value = keys
}

const openAddModal = () => {
  editingUser.value = null
  form.value = { username: '', hoten: '', password: '', role: '' }
  modalVisible.value = true
}

const openEditModal = (record) => {
  editingUser.value = record
  form.value = { id: record.id, username: record.username, hoten: record.hoten, role: record.role }
  modalVisible.value = true
}

const submitForm = async () => {
  modalLoading.value = true
  try {
    if (editingUser.value) {
      const { data } = await RestApi.users.update({ body: { ...form.value } })
      if (data.value?.status === 'success') {
        message.success('Cập nhật thành công')
      } else {
        throw new Error('Không thể cập nhật')
      }
    } else {
      const body = { ...form.value }
      delete body.id
      const { data } = await RestApi.users.create({ body })
      if (data.value?.status === 'success') {
        message.success('Thêm mới thành công')
      } else {
        throw new Error('Không thể thêm mới')
      }
    }
  } catch (err) {
    message.error(err.message || 'Đã xảy ra lỗi')
  } finally {
    modalVisible.value = false
    await fetchUsers({ ...param.value })
    modalLoading.value = false
  }
}

const handleDelete = async () => {
  if (!selectedRowKeys.value.length) return
  modalLoading.value = true
  try {
    const { data } = await RestApi.users.delete({ params: { id: selectedRowKeys.value.join(',') } })
    if (data.value?.status === 'success') {
      message.success(`Đã xoá ${selectedRowKeys.value.length} người dùng`)
      selectedRowKeys.value = []
    } else {
      throw new Error('Không thể xoá')
    }
  } catch (err) {
    message.error(err.message || 'Đã xảy ra lỗi')
  } finally {
    await fetchUsers({ ...param.value })
    modalLoading.value = false
  }
}

const handleDeleteOne = async (id) => {
  modalLoading.value = true
  try {
    const { data } = await RestApi.users.delete({ params: { id } })
    if (data.value?.status === 'success') {
      message.success('Xoá thành công')
    } else {
      throw new Error('Không thể xoá')
    }
  } catch (err) {
    message.error(err.message || 'Đã xảy ra lỗi')
  } finally {
    await fetchUsers({ ...param.value })
    modalLoading.value = false
  }
}

const resetForm = () => {
  modalVisible.value = false
  form.value = { username: '', hoten: '', password: '', role: '' }
  editingUser.value = null
}

await fetchUsers({ ...param.value })
</script>

