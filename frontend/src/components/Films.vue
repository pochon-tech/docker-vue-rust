<template>
  <table>
    <tr>
      <th>id</th>
      <th>title</th>
    </tr>
    <tr v-for="(film, key) in films" v-bind:key="key">
      <td>{{ film['film_id'] }}</td>
      <td>{{ film['title'] }}</td>
    </tr>
  </table>
</template>

<script>
export default {
  name: 'Films',
  data() {
    return {
      films: []
    }
  },
  mounted : function() {
    this.$axios.get('http://localhost/api/films')
    .then((response) => {
      this.films = response.data
    })
    .catch((error) => {
      console.log(error)
    });
  }
}
</script>
