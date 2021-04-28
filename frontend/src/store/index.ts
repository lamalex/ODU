import Vue from "vue";
import Vuex from "vuex";
import axios from "axios";

import { AuthResponse } from "@/api/definitions";

Vue.use(Vuex);

export default new Vuex.Store({
  state: {
    authData: { token: "", user: { role: null } },
    errorMsg: "",
    grants: [],
    adminGrants: [],
    departments: [],
    students: [],
    faculty: [],
  },
  getters: {
    errorMsg: (state) => {
      return state.errorMsg;
    },
    authenticated: (state) => {
      return !!state.authData?.token;
    },
    isAdministrator: (state) => {
      return state.authData?.user?.role === "ADMINISTRATOR"
    },
    adminGrants: (state) => {
      return state.adminGrants;
    },
    grants: (state) => {
      return state.grants;
    },
    departments: (state) => {
      return state.departments;
    },
    students: (state) => {
      return state.students;
    },
    faculty: (state) => {
      return state.faculty;
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
    setGrants(state, grants = []) {
      state.grants = grants;
    },
    setAdminGrants(state, grants = []) {
      state.adminGrants = grants;
    },
    setDepartments(state, departments = []) {
      state.departments = departments;
    },

    setStudents(state, students = []) {
      state.students = students;
    },
    setFaculty(state, faculty = []) {
      state.faculty = faculty
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
    async fetchGrants({ commit }) {
      const { data } = await axios.get("/api/grants");
      commit("setGrants", data);
    },
    async fetchAdminGrants({ commit }) {
      const { data } = await axios.get("/api/admin/grants");
      commit("setAdminGrants", data);
    },
    async fetchDepartments({ commit }) {
      const { data } = await axios.get("/api/departments");
      commit("setDepartments", data);
    },
    async fetchStudents({ commit }){
      const { data } = await axios.get("/api/students");
      commit("setStudents", data);
    },
    async fetchFaculty({ commit }) {
      try {
        const { data } = await axios.get("/api/admin/faculty");
        commit("setFaculty", data);
      } catch {
        console.log("bang!");
      }
    },
    deleteFaculty({ commit }, facultyId) {
      return axios.delete(`/api/admin/faculty/${facultyId}`).then(() => {
        this.dispatch("fetchFaculty");
      });
    },
    sendInvite({ commit }, inviteData) {
      commit("clearError");

      return axios.post("/api/auth/sendinvite", inviteData).catch((error) => {
        const { message: errMsg, code: errCode } = error.response?.data;

        commit("setError", errMsg ?? "Something unexpected happened 😵");
        throw errCode;
      });
    },
    updateGrantStatus({ dispatch }, { id, status }) {
      console.log("updating grant status");
      return axios.post(`/api/admin/grant/${id}`, {
        status
      }).then(() => {
        dispatch("fetchGrants");
        dispatch("fetchAdminGrants");
      });
    },
  },
});
