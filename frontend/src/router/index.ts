
import Vue from "vue"
import VueRouter, { RouteConfig } from "vue-router";

import Login from "@/views/Login.vue";
import Invite from "@/views/Invite.vue";
import Register from "@/views/Register.vue";
import Dashboard from "@/views/Dashboard.vue";
import { Recoverable } from "node:repl";

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
    meta: {
      unauthOnly: true
    },
  },
  {
    path: "/login/:prefillEmail?",
    name: "Login",
    component: Login,
    props: true,
    meta: {
      unauthOnly: true
    },
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

  // If route needs auth, but they're not autenticated -> login
  if (to.matched.some((record) => record.meta?.requiresAuth) && !loggedIn) {
    return next("/login");
  // If route needs NO auth, but they're autenticated -> dashboard
  } else if (to.matched.some((record) => record.meta?.unauthOnly) && loggedIn) {
    return next("/");
  }

  next();
});

export default router;
