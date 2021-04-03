# Random Lon Lat Generator

_Random Coordinates Generator, using Rust/WebAssembly and Vue._

## Structure
* Rust code is in root directory.
* Vue code is in `/website`
* Vue production build is in `/docs` served [here](https://zingi.github.io/random-lon-lat-generator/).

## Building Rust Code
Prerequisites: 
  * `rust/cargo` needs to be [installed](https://rustup.rs)
  * `wasm-pack` needs to be [installed](https://rustwasm.github.io/wasm-pack/installer/)

Bulding:
  * in root dir, run: `wasm-pack build`

## Vue Developement Server (Webpack)
Prerequisites:
  * `npm` needs to be [installed](https://github.com/nvm-sh/nvm)
  * `vue` cli needs to be [installed](https://cli.vuejs.org)
  * in `/website` run `npm i`

Starting Developement Server:
  * in `/website` run `npm run serve`

## Vue Production Build
Prerequisites:
  * _same as dev server_

Creating Production Build:
  * in `website` run `npm run build`
  * _(you may have to adjust the `publicPath` in [vue.config.js](./website/vue.config.js) in order to make it hostable on your localahost)_