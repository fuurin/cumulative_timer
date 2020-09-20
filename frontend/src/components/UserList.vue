<template>
  <ul>
    <li v-for="user in users" :key="user">
      id: {{ user.id }}, name: {{ user.name }}
    </li>
  </ul>
</template>

<script lang="ts">
export default {
  props: {
    // @TODO: User型的なのの配列を受け取れるようにしたい...
    users: Array
  }
}
</script>

<style scoped>
</style>
