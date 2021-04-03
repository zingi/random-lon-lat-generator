const path = require('path')
const WasmPackPlugin = require('@wasm-tool/wasm-pack-plugin')

module.exports = {
  outputDir: 'docs',
  publicPath: process.env.NODE_ENV === 'production' ? '/random-lon-lat-generator/' : '/',
  transpileDependencies: [
    'vuetify'
  ],
  chainWebpack: (config) => {
    // rust wasm bindgen https://github.com/rustwasm/wasm-bindgen
    config
      .plugin('wasm-pack')
      .use(WasmPackPlugin)
      .init(
        (Plugin) =>
          new Plugin({
            crateDirectory: path.resolve(__dirname, '../'),
            outDir: path.resolve(__dirname, '../pkg')
          })
      )
      .end()
  }
}
