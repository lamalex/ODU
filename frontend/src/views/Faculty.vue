<template>
<div>
    <div>
    <b-table :items="tableData" :fields="fields" striped responsive="sm">
      <template #cell(actions)="row">
        <b-button size="sm" @click="markAndDelete(row.item.id)" class="mr-2" variant="danger" block>
           <b-spinner v-if="row.item.pending" small></b-spinner>
          <span v-else>Delete</span>
        </b-button>
      </template>
    </b-table>
  </div>
</div>
</template>

<script>
import Vue from "vue"
import { mapActions, mapGetters, mapMutations } from "vuex";

export default Vue.extend({
    name: "Faculty",
    data() {
        return {
            fields: ["name", "department", "actions"],
        };
    },
    computed: {
        ...mapGetters(["faculty"]),
        tableData() {
            return Object.values(this.faculty).map((fac) => {
                return {
                    ...fac,
                    actions: true,
                    _rowVariant: fac.pending ? "warning" : ""
                };
            });
        }
    },
    methods: {
        ...mapMutations(["setFaculty"]),
        ...mapActions(["fetchFaculty", "deleteFaculty"]),
        markAndDelete(facultyId) {
            const pendingFac = {
                ...this.faculty,
                [facultyId]: {
                    ...this.faculty[facultyId],
                    pending: true,
                }
            };
            this.setFaculty(pendingFac);
            this.deleteFaculty(facultyId);
        }
    },
    created() {
      this.fetchFaculty();
    }
});
</script>
