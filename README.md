Battlecode 2019 Rust Starter Pack
=================================
Author: ansatz

This is a minimal framework I used to call Rust functions from my javascript bot. There are probably bugs and there are some limitation . You will need `wasm-pack` (`cargo install wasm-pack`) and Rust version 1.30 or higher. Optionally, install [wasm-opt] from here in your path to produce slightly smaller wasm files. Good examples of and information about the interface between Rust and Javascript can be found [here](https://rustwasm.github.io/wasm-bindgen/examples/index.html)

#Usage

Most of the process is automated, so you should only need to have `import {<your functions>} from './bot/native.js'` in the main robot file (as is shown in the `robot.js` file here).

* `make build-wasm` will compile the rust to wasm
* `make play-self` will start a game against yourself
* `make compile-bot` will produce the compiled version of your bot
* `make upload` will compile and upload your bot, given your credentials are already in their environment variables

#Caveats

This example is very minimally featured based on what I needed. Below are a few limitations of the current system I've run into. This list isn't exhaustive and contains only issues that I have run into while developing my bot.

* Passing `String` types between Rust and JS doesn't seem to work in the BC19 framework currently, since `wasm_bindgen` produces some JS glue that uses TextEncoder (when passing String type parameters to Rust functions) and TextDecoder (when returning a String type from a Rust function). There is a polyfill for TextEncoder (one of the missing classes) included, but it doesn't seem to work in the BC19 framework.

#Methodology

`wasm_bindgen` outputs an ES6 `npm` module, which there was no way to directly include in from one's bot. The included makefile runs a python script that uses the files from that module to create the file `bot/native.js`, which has the encoded wasm and `wasm_bindgen` exports and seamlessly works in the BC19 framework.
