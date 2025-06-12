import { defineStore } from "pinia";
export const useSettingStore = defineStore(
  "setting",
  {
    state: () => ({
      loading: {
        title: "Đang xử lý",
        description: "Vui lòng chờ trong giây lát...",
        isActive: false,
        showLogo: true,
        transparent: false,
      },
      menu: [
        // {
        //   title: "Dashboard",
        //   key: "dashboard",
        //   url: "/dashboard",
        //   bitIndex: 0,
        //   icon: "ant-design:project-outlined",
        //   children: [],
        // },
        {
          title: "Sản Phẩm",
          key: "product",
          url: "/product",
          bitIndex: 0,
          icon: "ant-design:product-outlined",
          children: [],
        },
        {
          title: "Hóa Đơn",
          key: "invoices",
          url: "/invoices",
          bitIndex: 0,
          icon: "ant-design:product-outlined",
          children: [],
        },
        {
          title: "Tạo Hóa Đơn",
          key: "create_invoices",
          url: "/create_invoices",
          bitIndex: 0,
          icon: "ant-design:product-outlined",
          children: [],
        },
        {
          title: "Cài Đặt Cửa Hàng",
          key: "setting",
          url: "/setting",
          bitIndex: 0,
          icon: "ant-design:product-outlined",
          children: [],
        },
      ],
    }),
    actions: {
      setLoading(value) {
        this.loading.isActive = value;
      },
      setDetailLoading(value) {
        this.loading = value;
      },
      setMenu(value) {
        this.menu = value;
      },
    },
    getters: {
      menuItems: state => state.menu,
      isLoading: state => state.loading.isActive,
    },
    // persist: {
    //   // storage: piniaPluginPersistedstate.localStorage(),
    //   storage: piniaPluginPersistedstate.cookies(),
    // },
  },
  // {
  //   persist: true,
  // },
);
