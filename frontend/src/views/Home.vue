<template lang="pug">
.home
  h1 UserList
  button(@click="load()") Load
  UserList(:users="state.users")
</template>

<script lang="ts">
import { reactive } from 'vue'
import axios from 'axios'
import UserList from '@/components/UserList.vue'

const API_BASE_URL = 'http://localhost:8000'

function apiCall (endpoint: string) {
  return `${API_BASE_URL}/${endpoint}`
}

type User = {
  id: number;
  name: string;
}

export default {
  components: {
    UserList
  },

  setup () {
    const state = reactive<{
      users: Array<User>;
    }>({
      users: []
    })

    function load () {
      axios.get(apiCall('users'))
        .then(res => {
          state.users = res.data.map(function (user: User) {
            return { id: user.id, name: user.name }
          })
        })
    }

    return { state, load }
  }
}
</script>
