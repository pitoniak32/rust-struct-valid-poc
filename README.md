# Demo POC for validating yaml 

This project was bootstrapped by [create-neon](https://www.npmjs.com/package/create-neon).

## Installing / Building

Installing requires a [supported version of Node and Rust](https://github.com/neon-bindings/neon#platform-support).

```sh
npm i
```

```sh
npm run build -- --release
```

## Exploring

After building, you can explore its exports at the Node REPL:

```sh
$ npm install
$ node
> require('.').validate_obj()
"No errors while validating."
```
if you make changes to the test.yml values you can cause errors to be found validating, for num > 10 or num < 1