<template>
  <div :class="{ shake: registrationRejected }">
    <b-form @submit.prevent="onSubmit">
      <b-form-group label="Your Name:" label-for="name">
        <b-form-input
          id="name"
          v-model="form.name"
          placeholder="Enter name"
          required
        ></b-form-input>
      </b-form-group>

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
          disabled
        ></b-form-input>
      </b-form-group>

      <b-form-group label="Password:" label-for="password">
        <Borat :isValid="pwRequirements">
          <template v-slot:input>
            <b-form-input
              id="password"
              v-model="form.password"
              placeholder="Password"
              type="password"
              required
            ></b-form-input>
          </template>
          <template v-slot:invalid-msg>
            Passwords must
            <ul>
              <li v-if="!has1Capital">have at least 1 capital</li>
              <li v-if="!isLongEnough">
                be longer than {{ minPasswordLen }} characters
              </li>
            </ul>
          </template>
        </Borat>
      </b-form-group>

      <b-form-group label="Re-enter yourpassword:" label-for="password-verify">
        <b-form-input
          id="password-verify"
          v-model="verify.password"
          placeholder="Password"
          type="password"
          required
        ></b-form-input>
        <b-form-invalid-feedback :state="passwordsMatch">
          Passwords must match
        </b-form-invalid-feedback>
      </b-form-group>

      <b-form-group label="Enter your department:" label-for="department">
        <b-form-select v-model="form.department" :options="departmentOptions" />
      </b-form-group>

      <div class="ml-auto">
        <b-button type="submit" variant="primary">Submit</b-button>
      </div>
    </b-form>
  </div>
</template>

<script lang="ts">
import Vue from "vue";
import { mapActions, mapGetters, mapMutations } from "vuex";

import Borat from "@/components/BoratValidated.vue";

type PrefillData = {
  name: string;
  email: string;
  department: number;
  userDataToken: string;
};

export default Vue.extend({
  name: "Register",
  components: { Borat },
  props: {
    prefillData: Object as () => PrefillData,
    minPasswordLen: {
      type: Number,
      default: 5,
    },
  },
  data() {
    return {
      form: {
        email: this.prefillData?.email ?? "",
        name: this.prefillData?.name ?? "",
        password: "",
        department: this.prefillData?.department ?? 1,
        userDataToken: this.prefillData?.userDataToken,
      },
      verify: {
        password: "",
      },
    };
  },
  computed: {
    ...mapGetters(["errorMsg", "departmentOptions"]),
    pwRequirements(): boolean {
      return this.isLongEnough && this.has1Capital;
    },
    isLongEnough(): boolean {
      return this.form.password.length >= this.minPasswordLen;
    },
    has1Capital(): boolean {
      return this.form.password.match(/[A-Z]/) !== null;
    },
    passwordsMatch(): boolean {
      return this.form.password === this.verify.password;
    },
    registrationRejected(): boolean {
      return this.errorMsg && this.errorMsg !== "";
    },
  },
  methods: {
    ...mapMutations(["setToken", "setError"]),
    ...mapActions(["register", "fetchDepartments"]),
    onSubmit(): void {
      if (!(this.passwordsMatch && this.pwRequirements)) {
        return;
      }

      this.register(this.form)
        .catch((errCode: number) => {
          this.form.password = "";
          this.verify.password = "";

          if (errCode === 69) {
            this.$router.replace(`/login/${this.form.email}`);
          }
        });
    },
  },
  created() {
    this.fetchDepartments();
    if (!this.form.userDataToken) {
      this.setError(
        `Your invitation seems to have been damaged.
        Verify the link you were sent, or contact your administrator`
      );
    }
  },
});
</script>

<style lang="scss">
@import "@/assets/scss/cs450.scss";
</style>
