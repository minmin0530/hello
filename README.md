# hello

rustでWebサーバを作り、canvasアニメーションをwasmで実行  

## 注意
chromeでは動きません。  
safariで動作確認しました。

## usage
wasmを各自環境で作り直したい場合  rustc +nightly --target wasm32-unknown-unknown src/main.rs -o libweb.wasm  
wasmを各自環境で作り直したい場合  wasm-gc libweb.wasm libweb.min.wasm  
  
```cargo run```

