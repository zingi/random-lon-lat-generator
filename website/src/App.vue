<template>
  <v-app>
    <div id="app">
      <div id="input">

        <v-switch v-model="customBoundingBox" label="Custom Bounding Box"></v-switch>

        <v-slider v-model="count" min="1" max="10000" label="Count">
          <template v-slot:append>
                <v-text-field
                  v-model="count"
                  class="mt-0 pt-0"
                  hide-details
                  single-line
                  type="number"
                  style="width: 60px"
                ></v-text-field>
              </template>
        </v-slider>

        <v-btn :disabled="!wasmReady" @click="generateButtonClicked">generate coordinates</v-btn>
      </div>

      <div id="output">
        <v-data-table dense :headers="headers" :items="coordinates" item-key="i" class="elevation-1">
        </v-data-table>
      </div>

    </div>
  </v-app>
</template>

<script>
import { VBtn, VSwitch, VSlider, VTextField, VDataTable } from 'vuetify/lib'
const wasm = import('../../pkg')

export default {
  name: 'App',
  components: {
    VBtn,
    VSwitch,
    VSlider,
    VTextField,
    VDataTable
  },
  data () {
    return {
      lonLatGenerator: null,
      customBoundingBox: false,
      count: 100,
      coordinates: [],
      headers: [
        {
          text: 'Longitude',
          align: 'start',
          value: 'lon'
        },
        {
          text: 'Latitude',
          align: 'start',
          value: 'lat'
        }
      ]
    }
  },
  methods: {
    generateButtonClicked () {
      this.generateRandomCoordinatesFast()
    },
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
    },
    generateRandomCoordinatesFast () {
      const lon = this.getRandomLonFast(this.count)
      const lat = this.getRandomLatFast(this.count)

      this.coordinates = []
      for (let i = 0; i < lon.length; i++) {
        this.coordinates.push({
          lon: lon[i],
          lat: lat[i],
          i
        })
      }

      console.log(this.coordinates)
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
* {
  margin: 0;
  padding: 0;
  font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
}
#app {
  width: 100vw;
  max-width: 100%;
  display: grid;
  grid-column-gap: 10px;
  grid-row-gap: 10px;

  grid-template-columns: 1fr 1fr;
  grid-template-rows: 1fr;

  grid-template-areas: 'inp out';
}

#input {
  grid-area: inp;
  background: yellowgreen;
}
#output {
  grid-area: out;
  background: burlywood;
}

</style>
