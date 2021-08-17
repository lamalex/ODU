<template>
  <div class="about">
    <h5>This ODU CS450 project is brought to you by</h5>

    <ul>
      <li
        :style="{ color: randomColor() }"
        style="font-size: 33px"
        v-for="author in authors"
        :key="author"
      >
        {{ author }}
      </li>
    </ul>
  </div>
</template>

<script lang="ts">
import Vue from "vue";

export default Vue.extend({
  data() {
    return {
      authors: [
        "Suraksha Tasgaonkar",
        "Joshua Farr",
        "Jason Rich",
        "Alex Launi",
      ],
    };
  },
  methods: {
    randomColor: function (): string {
      const genRandomOctet = () => Math.floor(Math.random() * 255);
      return `rgb(${genRandomOctet()}, ${genRandomOctet()}, ${genRandomOctet()})`;
    },
  },
});
</script>
