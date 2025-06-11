<template>
  <div class="min-h-screen bg-white p-4">
    <!-- Header -->
    <div class="mb-6">
      <h1 class="text-xl font-bold text-gray-800">Cài đặt cửa hàng</h1>
      <p class="text-gray-600 text-sm">Cập nhật thông tin và logo cửa hàng</p>
    </div>

    <!-- Form -->
    <div class="bg-white p-6 rounded-lg shadow-sm">
      <a-form layout="vertical" :model="form" @finish="handleSubmit">
        <!-- Store Name -->
        <a-form-item label="Tên cửa hàng" name="storeName" :rules="[{ required: true, message: 'Vui lòng nhập tên cửa hàng' }]">
          <a-input v-model:value="form.storeName" placeholder="Nhập tên cửa hàng" />
        </a-form-item>
        <a-form-item label="Địa Chỉ" name="address" :rules="[{ required: true, message: 'Vui lòng địa chỉ' }]">
          <a-input v-model:value="form.address" placeholder="Nhập địa chỉ" />
        </a-form-item>
        <!-- Phone Number -->
        <a-form-item label="Số điện thoại" name="phone" :rules="[
          { required: true, message: 'Vui lòng nhập số điện thoại' },
          { pattern: /(84|0[3|5|7|8|9])+([0-9]{8})\b/, message: 'Số điện thoại không hợp lệ' }
        ]">
          <a-input v-model:value="form.phone" placeholder="Nhập số điện thoại" />
        </a-form-item>
        <a-form-item label="URL logo cửa hàng" name="logoUrl">
          <a-input v-model:value="form.logoUrl" placeholder="Nhập URL ảnh logo" />
        </a-form-item>
        <!-- Actions -->
        <div class="flex justify-end gap-2 mt-6">
          <a-button @click="resetForm">
            Hủy bỏ
          </a-button>
          <a-button type="primary" html-type="submit" :loading="submitting">
            Lưu thay đổi
          </a-button>
        </div>
      </a-form>
    </div>

  </div>
</template>

<script setup>
const { RestApi } = useApi()

// State
const form = ref({
  id: '',
  storeName: '',
  address: '',
  phone: '',
  logoUrl: ''
})
const submitting = ref(false)
const loading = ref(false)

// Methods
const fetchStoreSettings = async () => {
  loading.value = true
  try {
    const { data } = await RestApi.setting.get()
    if (data.value?.status === 'success') {
      form.value = {
        id: data.value.data.id,
        storeName: data.value.data.storeName,
        address:data.value.data.address,
        phone: data.value.data.phone,
        logoUrl: data.value.data.logoUrl
      }
    }
  } finally {
    loading.value = false
  }
}

const handleSubmit = async () => {
  submitting.value = true
  try {
    const { data } = await RestApi.setting.update({ body: form.value })
    if (data.value?.status === 'success') {
      message.success('Cập nhật cài đặt thành công!')
    } else {
      throw new Error('Cập nhật không thành công')
    }
  } catch (error) {
    message.error(error.message || 'Đã xảy ra lỗi khi cập nhật')
  } finally {
    submitting.value = false
  }
}

const resetForm = () => {
  fetchStoreSettings()
}

// Lifecycle
await fetchStoreSettings()
</script>
