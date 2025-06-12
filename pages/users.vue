<template>
  <div class="min-h-screen bg-white p-4">
    <div class="">
      <!-- Header -->
      <div class="mb-6">
        <h1 class="text-xl font-bold text-gray-800">Quản lý tài khoản</h1>
        <p class="text-gray-600 text-sm">Tạo, chỉnh sửa và xoá các tài khoản người dùng</p>
      </div>

      <!-- Toolbar -->
      <div class="bg-white p-3 rounded-lg shadow-sm mb-4">
        <div class="flex flex-col md:flex-row md:items-center justify-between gap-2">
          <a-input-search
            v-model:value="searchText"
            placeholder="Tìm kiếm tài khoản..."
            enter-button
            allow-clear
            class="w-full md:w-80"
            @search="onSearch"
          />
          <div class="flex items-center gap-2">
            <a-popconfirm
              title="Bạn chắc chắn muốn xoá?"
              ok-text="Xoá"
              cancel-text="Huỷ"
              @confirm="handleDelete"
              :disabled="!selectedRowKeys.length"
            >
              <a-button danger :disabled="!selectedRowKeys.length" class="flex items-center gap-1">
                Xoá đã chọn
              </a-button>
            </a-popconfirm>
            <a-button type="primary" @click="openAddModal" class="flex items-center gap-1">
              Thêm tài khoản
            </a-button>
          </div>
        </div>
      </div>

      <!-- Table -->
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
                <a-button type="text" size="small" @click="openEditModal(record)">
                  Sửa
                </a-button>
                <a-popconfirm
                  title="Bạn chắc chắn muốn xoá?"
                  ok-text="Xoá"
                  cancel-text="Huỷ"
                  @confirm="() => handleDeleteOne(record.id)"
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

    <!-- Add/Edit Modal -->
    <a-modal
      v-model:visible="modalVisible"
      :title="editingUser ? 'Sửa tài khoản' : 'Thêm tài khoản mới'"
      @ok="submitForm"
      @cancel="resetForm"
      :confirm-loading="modalLoading"
      width="500px"
      :destroy-on-close="true"
    >
      <a-form layout="vertical" :model="form">
        <a-form-item
          label="Tên đăng nhập"
          name="username"
          :rules="[{ required: true, message: 'Vui lòng nhập tên đăng nhập' }]"
        >
          <a-input v-model:value="form.username" placeholder="Tên đăng nhập" />
        </a-form-item>
        <a-form-item
          label="Họ tên"
          name="fullName"
          :rules="[{ required: true, message: 'Vui lòng nhập họ tên' }]"
        >
          <a-input v-model:value="form.fullName" placeholder="Họ tên" />
        </a-form-item>
        <a-form-item
          v-if="!editingUser"
          label="Mật khẩu"
          name="password"
          :rules="[{ required: true, message: 'Vui lòng nhập mật khẩu' }]"
        >
          <a-input-password v-model:value="form.password" placeholder="Mật khẩu" />
        </a-form-item>
        <a-form-item
          label="Vai trò"
          name="role"
          :rules="[{ required: true, message: 'Vui lòng chọn vai trò' }]"
        >
          <a-select v-model:value="form.role" placeholder="Chọn vai trò">
            <a-select-option value="admin">Admin</a-select-option>
            <a-select-option value="user">User</a-select-option>
          </a-select>
        </a-form-item>
      </a-form>
      <template #footer>
        <a-button key="back" @click="resetForm">Huỷ</a-button>
        <a-button key="submit" type="primary" :loading="modalLoading" @click="submitForm">
          {{ editingUser ? 'Cập nhật' : 'Thêm mới' }}
        </a-button>
      </template>
    </a-modal>
  </div>
</template>

<script setup>
const { RestApi } = useApi();

const param = ref({ page: 1, limit: 10, search: '' });
const searchText = ref('');
const users = ref([]);
const total = ref(0);
const loading = ref(false);
const modalLoading = ref(false);
const selectedRowKeys = ref([]);

const modalVisible = ref(false);
const editingUser = ref(null);
const form = ref({ username: '', fullName: '', password: '', role: '' });

const columns = [
  { title: 'Tên đăng nhập', dataIndex: 'username', key: 'username', ellipsis: true },
  { title: 'Họ tên', dataIndex: 'fullName', key: 'fullName', ellipsis: true },
  { title: 'Vai trò', dataIndex: 'role', key: 'role', width: '120px', align: 'center' },
  { title: 'Hành động', key: 'actions', width: '120px', align: 'center' },
];

const pagination = computed(() => ({
  current: param.value.page,
  pageSize: param.value.limit,
  total: total.value,
  showSizeChanger: true,
  pageSizeOptions: ['1', '10', '20', '50', '100'],
  showTotal: total => `Tổng ${total} tài khoản`,
  size: 'small',
}));

const fetchUsers = async paramSource => {
  loading.value = true;
  try {
    const { data } = await RestApi.user.list({ params: paramSource });
    users.value = data.value?.data?.users || [];
    total.value = data.value?.data?.total || 0;
  } finally {
    loading.value = false;
  }
};

const handleTableChange = async paginator => {
  param.value.page = paginator.current;
  param.value.limit = paginator.pageSize;
  await fetchUsers({ ...param.value });
};

const onSearch = async () => {
  param.value.page = 1;
  param.value.search = searchText.value;
  await fetchUsers({ ...param.value });
};

const onSelectChange = selectedKeys => {
  selectedRowKeys.value = selectedKeys;
};

const openAddModal = () => {
  editingUser.value = null;
  form.value = { username: '', fullName: '', password: '', role: '' };
  modalVisible.value = true;
};

const openEditModal = record => {
  editingUser.value = record;
  form.value = { id: record.id, username: record.username, fullName: record.fullName, role: record.role };
  modalVisible.value = true;
};

const submitForm = async () => {
  modalLoading.value = true;
  try {
    if (editingUser.value) {
      const { data, error } = await RestApi.user.update({ body: { ...form.value } });
      if (data.value?.status === 'success') {
        message.success('Cập nhật tài khoản thành công!');
      } else {
        throw new Error(error.value?.data?.message || 'Không thành công');
      }
    } else {
      const payload = { ...form.value };
      const { data, error } = await RestApi.user.create({ body: payload });
      if (data.value?.status === 'success') {
        message.success('Thêm tài khoản thành công!');
      } else {
        throw new Error(error.value?.data?.message || 'Không thành công');
      }
    }
  } catch (error) {
    message.error(error.message || error.response?.data?.message || 'Đã xảy ra lỗi khi lưu thông tin');
  } finally {
    modalVisible.value = false;
    await fetchUsers({ ...param.value });
    modalLoading.value = false;
  }
};

const handleDelete = async () => {
  if (!selectedRowKeys.value.length) return;
  modalLoading.value = true;
  try {
    const { data, error } = await RestApi.user.delete({ params: { id: selectedRowKeys.value.join(',') } });
    if (data.value?.status === 'success') {
      message.success(`Đã xoá ${selectedRowKeys.value.length} tài khoản`);
      selectedRowKeys.value = [];
    } else {
      throw new Error(error.value?.data?.message || 'Không thành công');
    }
  } catch (error) {
    message.error(error.message || error.response?.data?.message || 'Đã xảy ra lỗi khi lưu thông tin');
  } finally {
    await fetchUsers({ ...param.value });
    modalLoading.value = false;
  }
};

const handleDeleteOne = async id => {
  modalLoading.value = true;
  try {
    const { data, error } = await RestApi.user.delete({ params: { id } });
    if (data.value?.status === 'success') {
      message.success('Xoá tài khoản thành công!');
    } else {
      throw new Error(error.value?.data?.message || 'Không thành công');
    }
  } catch (error) {
    message.error(error.message || error.response?.data?.message || 'Đã xảy ra lỗi khi lưu thông tin');
  } finally {
    await fetchUsers({ ...param.value });
    modalLoading.value = false;
  }
};

const resetForm = () => {
  modalVisible.value = false;
  form.value = { username: '', fullName: '', password: '', role: '' };
  editingUser.value = null;
};

await fetchUsers({ ...param.value });
</script>
