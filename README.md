# `rust-parcel-template`

**Kickstart your Rust, WebAssembly, and Parcel project!**

This template comes pre-configured with all the boilerplate for compiling Rust
to WebAssembly and hooking into a Parcel build pipeline.

* `npm run start:rust-app` -- Serve the project locally for
  development at `http://localhost:1234`.

* `npm run build` -- Bundle the project (in production mode)


## Using This Template

```sh
cargo install wasm-pack
```

```sh
npm init rust-parcel my-app
```

## Stats from Project

I not only wanted to see how to make a web application with Rust, but I wanted to see the performance benefits where needed.

So I made two different applications in this same project. One application ran with Rust/WebAssembly and the other ran with JavaScript. Both applications calculate all the prime numbers in between 1 and a given number. Then all those prime numbers are displayed on the web page.

Here are the stats! As can be seen by the stats, WebAssembly brings in the efficiency. JS just starts to loose right around 10,000, and seriously sweats with over a million. Web Assembly handles over a million just fine.

### Rust web application

10 = ~ 1ms
100 = ~ 1ms
100 = ~ 1ms
1,000 = ~ 1ms
10,000 = ~ 1ms
100,000 = 3-5 ms
1,000,000 = 24 - 30 ms
10,000,000 = ~ 130 ms

### JS web application

10 = ~ 1ms
100 = ~ 1ms
100 = ~ 1ms
1,000 = ~ 1ms
10,000 = 1-3 ms
100,000 = 13-14 ms
1,000,000 = 122-126 ms
10,000,000 = ~ 2450 ms

## Reference

[wasm-bindgen book!](https://rustwasm.github.io/wasm-bindgen/introduction.html)
