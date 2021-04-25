<template>
  <div :class="{ shake: registrationRejected }">
    <b-form @submit.prevent="onSubmit">
      <b-form-group label="Enter your student" label-for="student">
        <b-form-select v-model="form.student" :options="studentOptions" />
      </b-form-group>
      <b-form-group label="Type of assistantship" label-for="type">
        <b-form-input id="type" v-model="form.type" type="type"></b-form-input>
      </b-form-group>
      <b-form-group label="Semester" label-for="sem">
        <b-form-input id="type" v-model="form.semester" type="semester"></b-form-input>
      </b-form-group>
      <b-form-group label="Amount" label-for="sem">
        <b-form-input id="type" v-model="form.amount" type="amount"></b-form-input>
      </b-form-group>
      <b-form-group label="Payment Type" label-for="pay">
        <b-form-input id="type" v-model="form.payment" type="payment"></b-form-input>
      </b-form-group>
      <b-form-group label="Workload" label-for="work">
        <b-form-input id="work" v-model="form.workload" type="workload"></b-form-input>
      </b-form-group>
      <b-form-group label="Start Date" label-for="start">
        <b-form-input id="start" v-model="form.startdate" type="startdate"></b-form-input>
      </b-form-group>
      <b-form-group label="End Date" label-for="end">
        <b-form-input id="end" v-model="form.enddate" type="enddate"></b-form-input>
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


type PrefillData = {
  student: number;
  type: string;
  semester: string;
  amount: number;
  payment:string;
  workload:number;
  startdate:string;
  enddate:string;

};

export default Vue.extend({
  name: "Employ",
  props: {
    prefillData: Object as () => PrefillData,
  },
  data() {
    return {
      form: {
        student: this.prefillData?.student ?? 1,
        type: this.prefillData?.type ?? "",
        amount: this.prefillData?.amount ?? 1,
        payment: this.prefillData?.payment ?? "",
        workload: this.prefillData?.payment ?? 1,
        startdate: this.prefillData?.startdate ?? "",
        enddate: this.prefillData?.enddate ?? "",

      },
    };
  },
  computed: {
    ...mapGetters(["errorMsg", "studentOptions"]),
  },
  methods: {
    ...mapMutations(["setToken", "setError"]),
    ...mapActions(["employment", "fetchStudents"]),
    onSubmit(): void {
        this.employment(this.form)
        .then(() => {
          //this.$router.replace("/about");
        })
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
    this.fetchStudents();
    
    
  },
});
</script>

<style lang="scss">
@import "@/assets/scss/cs450.scss";
</style>
