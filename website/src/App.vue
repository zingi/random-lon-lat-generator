<template>
  <div id="app">

  </div>
</template>

<script>
const wasm = import('../../pkg')

export default {
  name: 'App',
  data () {
    return {
      lonLatGenerator: null
    }
  },
  methods: {
    getRandomLatFast (count) {
      return this.lonLatGenerator.get_random_lat_coordinates_fast(count)
    },
    getRandomLonFast (count) {
      return this.lonLatGenerator.get_random_lon_coordinates_fast(count)
    },
    getRandomLonBetterEntropy (count) {
      return this.lonLatGenerator.get_random_lon_coordinates_better_entropy(count)
    },
    getRandomLatBetterEntropy (count) {
      return this.lonLatGenerator.get_random_lat_coordinates_better_entropy(count)
    },
    getRandomNumberInRangeFast (from, until, count) {
      return this.lonLatGenerator.get_random_numbers_in_range_fast(from, until, count)
    },
    getRandomNumbersInRangeBetterEntropy (from, until, count) {
      this.lonLatGenerator.get_random_numbers_in_range_better_entropy(from, until, count)
    }
  },
  computed: {
    wasmReady () {
      return !!this.lonLatGenerator
    }
  },
  created: async function () {
    this.lonLatGenerator = await wasm
  }
}
</script>

<style>
#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: #2c3e50;
  margin-top: 60px;
}
</style>
