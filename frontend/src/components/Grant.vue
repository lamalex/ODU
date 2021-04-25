<template>
  <b-card-group columns>
    <b-card
      no-body
      v-for="(grant, i) in grants"
      :key="i"
      :border-variant="borderStyle(grant)"
      :header="grant.title"
      header-border-variant="secondary"
    >
      <b-card-body>
        <b-card-text>
          <ul>
            <li>{{ grant.balance | currency }}</li>
            <li>{{ grant.originalAmount | currency }}</li>
          </ul>
        </b-card-text>
      </b-card-body>
      <b-card-footer>
          <small class="text-muted">{{ grant.status }}</small>
      </b-card-footer>
    </b-card>
  </b-card-group>
</template>

<script>
import Vue from "vue";
import { mapActions, mapGetters } from "vuex";

export default Vue.extend({
  name: "Dashboard",
  computed: {
    ...mapGetters(["grants"]),
  },
  methods: {
    ...mapActions(["fetchGrants"]),
    borderStyle(grant) {
        const styles = {
            "PENDING": "warning",
            "APPROVED": "success",
            "DENIED": "danger",
        };

        return styles[grant.status];
    }
  },
  async created() {
    await this.fetchGrants();
    console.log(JSON.stringify(this.grants));
  },
});
</script>
