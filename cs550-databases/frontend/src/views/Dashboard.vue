<template>
<div>
    <div>
        <h2>My Grants</h2>
        <Grant />
    </div>
    <div class="mt-4" v-if="isAdministrator">
        <h2>Administered Grants</h2>
        <Grant admin />
    </div>
</div>
</template>
<script>
import Vue from "vue";
import Grant from "@/components/Grant.vue";
import { mapGetters } from "vuex";

export default Vue.extend({
    name: "Dashboard",
    components: { Grant },
    computed: {
        ...mapGetters(["isAdministrator"])
    }
})
</script>
