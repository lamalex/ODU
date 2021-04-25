<template>
<div>
    <Grant />
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
