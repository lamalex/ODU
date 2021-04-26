<template>
<div>
  <b-navbar id="nav" variant="faded" type="light">
    <b-navbar-nav >
      <b-nav-item-dropdown
      v-if="authenticated"
      id="my-nav-dropdown"
      text="Add a new..."
      right
      is-nav
    >
    <b-dropdown-item class="align-middle">
      <router-link to="/invite">Faculty member</router-link>
    </b-dropdown-item>
    <b-dropdown-item class="align-middle">
      <router-link to="/employment">Student Employee</router-link>
    </b-dropdown-item>
    </b-nav-item-dropdown>

    <b-nav-item-dropdown
      v-if="authenticated"
      id="my-nav-dropdown"
      text="Manage..."
    >
      <b-dropdown-item class="align-middle">
        <router-link to="/faculty">Faculty</router-link>
      </b-dropdown-item>
    </b-nav-item-dropdown>
    </b-navbar-nav>

    <b-navbar-nav class="ml-auto">
        <b-nav-item right>
        <router-link to="/about">About</router-link>
      </b-nav-item>
      <b-nav-item  @click="logout" right >
        <router-link v-if="authenticated" to="">Logout</router-link>
        <router-link v-else to="/login">Login</router-link>
      </b-nav-item>
    </b-navbar-nav>
  </b-navbar>
  </div>
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
