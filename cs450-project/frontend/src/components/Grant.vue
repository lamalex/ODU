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
        <b-row>
          <b-col>
            <small class="text-muted">{{ grant.status }}</small> 
          </b-col>
          <b-col class="pr-0" v-if="admin">
            <b-button v-if="grant.status !== 'APPROVED'" @click="approveGrant(grant.id)" size="sm" variant="success">Approve</b-button>
            <b-button v-if="grant.status !== 'DENIED'" @click="denyGrant(grant.id)" class="ml-2" size="sm" variant="danger">Deny</b-button>
            <b-button v-if="grant.status !== 'PENDING'" @click="resetGrant(grant.id)" class="ml-2" size="sm" variant="warning">Unmark</b-button>
          </b-col>
        </b-row>
      </b-card-footer>
    </b-card>
  </b-card-group>
</template>

<script>
import Vue from "vue";
import { mapActions } from "vuex";

export default Vue.extend({
  name: "Dashboard",
  props: {
    admin: Boolean
  },
  computed: {
    grants() {
      return this.admin ? this.$store.getters.adminGrants: this.$store.getters.grants
    },
  },
  methods: {
    ...mapActions(["fetchGrants", "fetchAdminGrants", "updateGrantStatus"]),
    borderStyle(grant) {
        const styles = {
            "PENDING": "warning",
            "APPROVED": "success",
            "DENIED": "danger",
        };

        return styles[grant.status];
    },
    approveGrant(grantId) {
      this.updateGrantStatus({
        id: grantId,
        status: "APPROVE"
      });
    },
    denyGrant(grantId) {
      this.updateGrantStatus({
        id: grantId,
        status: "DENY"
      });
    },
    resetGrant(grantId) {
      this.updateGrantStatus({
        id: grantId,
        status: "PENDING"
      });
    }
  },
  created() {
    this.fetchGrants();
    this.fetchAdminGrants();
  },
});
</script>
