<template>
  <div id="app">
    <b-nav id="nav">
      <span v-if="!authenticated">
        <router-link to="/">Register</router-link>
        <span class="pl-1 pr-1">|</span>
      </span>
      <router-link to="/about">About</router-link>
    </b-nav>
    <b-container>
      <b-alert
        fade
        :show="errorMsg !== ''"
        variant="danger"
        dismissible
        @dismissed="clearError"
      >
        {{ errorMsg }}
      </b-alert>
      <router-view />
    </b-container>
  </div>
</template>

<script>
import { mapGetters, mapMutations } from "vuex";

export default {
  name: "App",
  computed: {
    ...mapGetters(["errorMsg", "authenticated"]),
  },
  methods: {
    ...mapMutations(["clearError"]),
  },
};
</script>

<style lang="scss">
#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  color: #2c3e50;
}

#nav {
  padding: 30px;

  a {
    font-weight: bold;
    color: #2c3e50;

    &.router-link-exact-active {
      color: #42b983;
    }
  }
}
</style>
