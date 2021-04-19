<template>
  <b-alert
    fade
    :show="errorMsg !== ''"
    variant="danger"
    dismissible
    @dismissed="clearError"
  >
    {{ errorMsg }}
  </b-alert>
</template>

<script>
import Vue from "vue";
import { mapGetters, mapMutations } from "vuex";

export default Vue.extend({
  name: "MessageBox",
  computed: {
    ...mapGetters(["errorMsg"]),
  },
  methods: {
    ...mapMutations(["clearError"]),
  },
});
</script>
