<template>
  <div>
    <slot name="input"></slot>
    <b-form-valid-feedback :state="isValid">
      <span>&lt;Borat&gt;Veh-ry niice&lt;/Borat&gt;</span>
    </b-form-valid-feedback>
    <b-form-invalid-feedback :state="isValid">
      <slot name="invalid-msg"></slot>
    </b-form-invalid-feedback>
  </div>
</template>

<script lang="ts">
import Vue from "vue";

export default Vue.extend({
  props: {
    isValid: Boolean,
  },
});
</script>
