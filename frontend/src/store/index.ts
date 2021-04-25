import Vue from "vue";
import Vuex from "vuex";
import axios from "axios";

import { AuthResponse } from "@/api/definitions";

Vue.use(Vuex);

export default new Vuex.Store({
  state: {
    authData: { token: "" },
    errorMsg: "",
    departments: [],
    students: [],
  },
  getters: {
    errorMsg: (state) => {
      return state.errorMsg;
    },
    authenticated: (state) => {
      return !!state.authData?.token;
    },
    departments: (state) => {
      return state.departments;
    },
    students: (state) => {
      return state.students;
    },
    departmentOptions: (state) => {
      return state.departments.map((dept: { id: number; name: string }) => {
        return {
          value: dept.id,
          text: dept.name,
        };
      });
    },
    studentOptions: (state) => {
      return state.students.map((student: { uin: number; name: string }) => {
        return {
          value: student.uin,
          text: student.name,
        };
      });
    },
    
  },
  mutations: {
    setDepartments(state, departments = []) {
      state.departments = departments;
    },

    setStudents(state, students = []){
      state.students = students;
    },

    setAuthData(state, authData) {
      state.authData = authData;

      localStorage.setItem("authData", JSON.stringify(authData));

      axios.defaults.headers.common["Authorization"] = `Bearer ${
        authData?.token ?? ""
      }`;
    },
    clearAuthData() {
      localStorage.removeItem("authData");
      location.reload();
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
          commit("setAuthData", data);
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
    logout({ commit }) {
      commit("clearAuthData");
    },

    employment({ dispatch }, credentials){
      return dispatch("authAction", {
        actionName: "employ",
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
    async fetchStudents({ commit }){
      const { data } = await axios.get("/api/students");
      commit("setStudents", data);
    },
    sendInvite({ commit }, inviteData) {
      commit("clearError");

      return axios.post("/api/auth/sendinvite", inviteData).catch((error) => {
        const { message: errMsg, code: errCode } = error.response?.data;

        commit("setError", errMsg ?? "Something unexpected happened 😵");
        throw errCode;
      });
    },
  },
});
