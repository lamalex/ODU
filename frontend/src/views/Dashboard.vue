<template>
<div>
    <div>
        <h2>My Grants</h2>
        <Grant />
    </div>
</div>
</template>
<script>
import Vue from "vue";
import Grant from "@/components/Grant.vue";

export default Vue.extend({
    name: "Dashboard",
    components: { Grant }
})
</script>
