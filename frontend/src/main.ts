import Vue from "vue";
import axios from "axios";
import BootstrapVue from "bootstrap-vue";

import App from "./App.vue";
import router from "./router";
import store from "./store";

import "./assets/scss/cs450.scss";

Vue.use(BootstrapVue);

Vue.config.productionTip = false;

new Vue({
  router,
  store,
  created() {
    const authString = localStorage.getItem("authData");
    if (authString) {
      const authData = JSON.parse(authString);
      this.$store.commit("setAuthData", authData);
    }

    axios.interceptors.response.use(
      response => response,
      error => {
        if (error.response.status === 401) {
          this.$store.dispatch('logout');
        }

        return Promise.reject(error);
      },
    );
  },
  render: (h) => h(App),
}).$mount("#app");
