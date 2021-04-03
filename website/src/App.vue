<template>
  <v-app>
    <div id="app">
      <div id="input">
        <v-switch v-model="customBoundingBox" label="Custom Bounding Box"></v-switch>

        <div>
          <v-range-slider v-model="lonRange" min="-180" max="180" hide-details v-if="customBoundingBox">
            <template v-slot:prepend>
              <v-text-field
                :value="lonRange[0]"
                class="mt-0 pt-0"
                hide-details
                single-line
                type="number"
                style="width: 60px"
                @change="$set(lonRange, 0, $event)"
              ></v-text-field>
            </template>
            <template v-slot:append>
              <v-text-field
                :value="lonRange[1]"
                class="mt-0 pt-0"
                hide-details
                single-line
                type="number"
                style="width: 60px"
                @change="$set(lonRange, 1, $event)"
              ></v-text-field>
            </template>
          </v-range-slider>
        </div>

        <div>
          <v-range-slider v-model="latRange" min="-90" max="90" hide-details v-if="customBoundingBox">
            <template v-slot:prepend>
              <v-text-field
                :value="latRange[0]"
                class="mt-0 pt-0"
                hide-details
                single-line
                type="number"
                style="width: 60px"
                @change="$set(latRange, 0, $event)"
              ></v-text-field>
            </template>
            <template v-slot:append>
              <v-text-field
                :value="latRange[1]"
                class="mt-0 pt-0"
                hide-details
                single-line
                type="number"
                style="width: 60px"
                @change="$set(latRange, 1, $event)"
              ></v-text-field>
            </template>
          </v-range-slider>
        </div>

        <div>
          <v-slider v-model="count" min="1" max="1000000" label="Count">
            <template v-slot:append>
                  <v-text-field
                    v-model="count"
                    class="mt-0 pt-0"
                    hide-details
                    single-line
                    type="number"
                    style="width: 80px"
                  ></v-text-field>
                </template>
          </v-slider>
        </div>

        <div>
          <v-select
            v-model="selectedAlgorithm"
            :hint="`Algorithm: ${selectedAlgorithm.algo}`"
            :items="randomAlgorithms"
            item-text="description"
            item-value="algo"
            label="Random Number Generator"
            persistent-hint
            return-object
          ></v-select>
        </div>

        <br/>
        <v-btn :disabled="!wasmReady" @click="generateButtonClicked">generate coordinates</v-btn>
        <br/>
        <v-btn :disabled="!coordinatesExist" @click="downloadCsvClicked">download csv</v-btn>
      </div>

      <div id="output">
        <h2>Preview</h2>
        <v-data-table dense disable-sort :headers="headers" :items="coordinates" item-key="i" class="elevation-1" id="preview-table"></v-data-table>
      </div>
    </div>
  </v-app>
</template>

<script>
import { VBtn, VSwitch, VSlider, VRangeSlider, VTextField, VDataTable, VSelect } from 'vuetify/lib'
const wasm = import('../../pkg')
const textEncoder = new TextEncoder()

const COMMA = new Uint8Array([44])
const NEW_LINE = new Uint8Array([10])

export default {
  name: 'App',
  components: {
    VBtn,
    VSwitch,
    VSlider,
    VRangeSlider,
    VTextField,
    VDataTable,
    VSelect
  },
  data () {
    return {
      lonLatGenerator: null,
      lon: new Float32Array(),
      lat: new Float32Array(),
      lonRange: [-180, 180],
      latRange: [-90, 90],
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
      ],
      randomAlgorithms: [
        { description: 'fast', algo: 'Xoshiro128PlusPlus' },
        { description: 'better entropy', algo: 'Crypto.getRandomValues()' }
      ],
      selectedAlgorithm: {}
    }
  },
  methods: {
    generateButtonClicked () {
      if (this.selectedAlgorithm.description === 'better entropy') {
        this.generateRandomCoordinatesBetterEntropy()
      } else {
        this.generateRandomCoordinatesFast()
      }
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
      return this.lonLatGenerator.get_random_numbers_in_range_better_entropy(from, until, count)
    },
    generateRandomCoordinatesFast () {
      if (this.customBoundingBox) {
        this.lon = this.getRandomNumberInRangeFast(this.lonRange[0], this.lonRange[1], this.count)
        this.lat = this.getRandomNumberInRangeFast(this.latRange[0], this.latRange[1], this.count)
      } else {
        this.lon = this.getRandomLonFast(this.count)
        this.lat = this.getRandomLatFast(this.count)
      }
      this.updatePreview()
    },
    generateRandomCoordinatesBetterEntropy () {
      if (this.customBoundingBox) {
        this.lon = this.getRandomNumbersInRangeBetterEntropy(this.lonRange[0], this.lonRange[1], this.count)
        this.lat = this.getRandomNumbersInRangeBetterEntropy(this.latRange[0], this.latRange[1], this.count)
      } else {
        this.lon = this.getRandomLonBetterEntropy(this.count)
        this.lat = this.getRandomLatBetterEntropy(this.count)
      }
      this.updatePreview()
    },
    updatePreview () {
      // show max 1000 coordinates in preview
      const previewCount = Math.min(this.lon.length, 1000)
      this.coordinates = []
      for (let i = 0; i < previewCount; i++) {
        this.coordinates.push({
          lon: this.lon[i],
          lat: this.lat[i],
          i
        })
      }
    },
    downloadCsvClicked () {
      // max length in byte of lon or lat
      const maxNumberByteLength = 19
      // max count of bytes, which lon and lat arr could occupy
      const numbersByteCount = this.lon.length * maxNumberByteLength * 2
      // byte count of ';' and '\n' chars
      const layoutByteCount = this.lon.length * 2

      // empty result buffer with fixed length
      const buffer = new Uint8Array(numbersByteCount + layoutByteCount)

      // conveniance function to fill result buffer
      let bufferIndex = 0
      const pushIntoBuffer = (typedArr = new Uint8Array()) => {
        buffer.set(typedArr, bufferIndex)
        bufferIndex += typedArr.byteLength
      }

      // conveniance function to convert a number to a string to a byte array
      const toByteArr = (e) => textEncoder.encode(`${e}`)

      // iterate over all coordinates, convert them to bytes and copy into result buffer
      for (let i = 0; i < this.lon.length; i++) {
        const lonBytes = toByteArr(this.lon[i])
        const latBytes = toByteArr(this.lat[i])

        pushIntoBuffer(lonBytes)
        pushIntoBuffer(COMMA)
        pushIntoBuffer(latBytes)
        pushIntoBuffer(NEW_LINE)
      }

      // create fake link and simulate download click
      const link = document.createElement('a')
      link.download = 'coordinates.csv'
      const blob = new Blob([buffer.buffer], { type: 'text/csv' })
      link.href = URL.createObjectURL(blob)
      link.click()
      URL.revokeObjectURL(link.href)
    }
  },
  computed: {
    wasmReady () {
      return !!this.lonLatGenerator
    },
    coordinatesExist () {
      return this.lon.length > 0
    }
  },
  beforeMount: function () {
    this.selectedAlgorithm = this.randomAlgorithms[0]
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
  grid-template-rows: auto;

  grid-template-areas: 'inp out';
}

#input {
  grid-area: inp;
  display: flex;
  flex-direction: column;
  justify-content: flex-start;
  padding: 20px;
}
#output {
  grid-area: out;
  display: flex;
  flex-direction: column;
  justify-content: flex-start;
  align-items: center;
}

#preview-table {
  width: 100%;
}

@media (max-width: 1000px) {
  #app {
    grid-template-columns: 1fr;
    grid-template-rows: auto auto;
    grid-template-areas:
      'inp'
      'out';
  }
}
</style>
