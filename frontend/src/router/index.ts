
import Vue from "vue"
import VueRouter, { RouteConfig } from "vue-router";

import Login from "@/views/Login.vue";
import Invite from "@/views/Invite.vue";
import Register from "@/views/Register.vue";
import Dashboard from "@/views/Dashboard.vue";

Vue.use(VueRouter);

const routes: Array<RouteConfig> = [
  {
    path: "/",
    name: "Dashboard",
    component: Dashboard,
    meta: {
      requiresAuth: true,
    },
  },
  {
    path: "/register/:prefillData",
    name: "Register",
    component: Register,
    props: (route) => {
      try {
        const jsonStr = Buffer.from(
          route.params.prefillData,
          "base64"
        ).toString("utf-8");

        const prefillData = JSON.parse(jsonStr);
        return prefillData?.userDataToken
          ? { prefillData }
          : { prefillData: {} };
      } catch (e) {
        console.error(
          `failed to parse base64 ${route.params?.prefillData} ${e}`
        );
      }
    },
    beforeEnter(to, from, next) {
      const loggedIn = localStorage.getItem("authData");
      if (loggedIn) {
        return next("/dashboard");
      }
      next();
    },
  },
  {
    path: "/login/:prefillEmail?",
    name: "Login",
    component: Login,
    props: true,
  },
  {
    path: "/about",
    name: "About",
    // route level code-splitting
    // this generates a separate chunk (about.[hash].js) for this route
    // which is lazy-loaded when the route is visited.
    component: () =>
      import(/* webpackChunkName: "about" */ "../views/About.vue"),
  },
  {
    path: "/invite",
    name: "Invite",
    component: Invite,
    meta: {
      requiresAuth: true,
    },
  },
];

const router = new VueRouter({
  routes,
});

router.beforeEach((to, from, next) => {
  const loggedIn = localStorage.getItem("authData");

  if (to.matched.some((record) => record.meta?.requiresAuth) && !loggedIn) {
    return next("/");
  }

  next();
});

export default router;
