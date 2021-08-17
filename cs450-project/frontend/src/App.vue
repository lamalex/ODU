<template>
  <div id="app">
    <Header />
    <b-container>
      <MessageBox />
      <router-view />
    </b-container>
  </div>
</template>

<script lang="ts">
import Header from "@/components/Header.vue";
import MessageBox from "@/components/MessageBox.vue";

export default {
  name: "App",
  components: { Header, MessageBox },
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
