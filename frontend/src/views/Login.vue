<template>
  <div>
    <b-form-group
      label="Email address:"
      label-for="email"
      description="We'll never share your email with anyone else."
    >
      <b-form-input
        id="email"
        v-model="form.email"
        type="email"
        placeholder="Enter email"
        required
      ></b-form-input>
    </b-form-group>

    <b-form-group label="Password:" label-for="password">
      <b-form-input
        id="password"
        v-model="form.password"
        placeholder="Password"
        type="password"
        required
      />
    </b-form-group>
  </div>
</template>

<script>
import Vue from "vue";

export default Vue.extend({
  name: "Login",
  data() {
    return {
      form: {
        email: this.$route?.params?.prefillEmail ?? "",
        password: "",
      },
    };
  },
});
</script>
