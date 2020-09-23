<template lang="pug">
ul
  li(v-for="user in users" :key="user")
    div id: {{ user.id }}, name: {{ user.name }}
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
