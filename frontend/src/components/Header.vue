<template>
  <b-nav id="nav" pills>
    <b-nav-item-dropdown
      v-if="authenticated"
      id="my-nav-dropdown"
      text="Add a new..."
      right
    >
    <b-dropdown-item class="align-middle">
      <router-link to="/invite">Faculty member</router-link>
    </b-dropdown-item>
    <b-dropdown-item class="align-middle">
      <router-link to="/employment">Student Employee</router-link>
    </b-dropdown-item>
    </b-nav-item-dropdown>
    <b-nav-item>
      <router-link to="/about">About</router-link>
    </b-nav-item>
    <b-nav-item class="ml-auto" @click="logout">
      <router-link v-if="authenticated" to="">Logout</router-link>
      <router-link v-else to="/login">Login</router-link>
    </b-nav-item>
  </b-nav>
</template>

<script>
import Vue from "vue";
import { mapActions, mapGetters } from "vuex";

export default Vue.extend({
  name: "Header",
  computed: {
    ...mapGetters(["authenticated"]),
  },
  methods: {
    ...mapActions(["logout"]),
  },
});
</script>
