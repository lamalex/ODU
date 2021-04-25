<template>
  <table>
    <t-header :header="tableHeaders"/>
    <t-row v-for="(row, $index) in tableRows" :key="$index" :row="row"
    />
  </table>
</template>