import Vue from "vue";
import Vuex from "vuex";
import axios from "axios";

import { AuthResponse } from "@/api/definitions";

Vue.use(Vuex);

export default new Vuex.Store({
  state: {
    token: "",
    errorMsg: "",
    departments: [],
  },
  getters: {
    errorMsg: (state) => {
      return state.errorMsg;
    },
    authenticated: (state) => {
      return state.token && state.token !== "";
    },
    departments: (state) => {
      return state.departments;
    },
    departmentOptions: (state) => {
      return state.departments.map((dept: { id: number; name: string }) => {
        return {
          value: dept.id,
          text: dept.name,
        };
      });
    },
  },
  mutations: {
    setDepartments(state, departments = []) {
      state.departments = departments;
    },
    setAuthData(state, authData) {
      state.token = authData;
      localStorage.setItem("authData", JSON.stringify(authData));
      axios.defaults.headers.common[
        "Authorization"
      ] = `Bearer ${authData.token}`;
    },
    setError(state, message) {
      state.errorMsg = message;
    },
    clearError(state) {
      state.errorMsg = "";
    },
  },
  actions: {
    authAction({ commit }, { actionName, credentials }) {
      commit("clearError");

      return axios
        .post<AuthResponse>(`/api/auth/${actionName}`, credentials)
        .then(({ data }) => {
          const { token } = data;
          commit("setAuthData", token);
        })
        .catch((error) => {
          const { message: errMsg, code: errCode } = error.response?.data;

          commit("setError", errMsg ?? "Something unexpected happened 😵");
          throw errCode;
        });
    },
    login({ dispatch }, credentials) {
      return dispatch("authAction", {
        actionName: "login",
        credentials,
      });
    },
    register({ dispatch }, credentials) {
      return dispatch("authAction", {
        actionName: "register",
        credentials,
      });
    },
    async fetchDepartments({ commit }) {
      const { data } = await axios.get("/api/departments");
      commit("setDepartments", data);
    },
  },
});
