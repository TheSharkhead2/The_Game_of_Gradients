# Compiling to WASM 
You can't just straight compile from this branch, so first make sure to be on the main branch: 
```
git checkout main 
git pull
``` 
From here, build the project to web assembly: 
```
cargo build --release --target wasm32-unknown-unknown
```
And then generate the accompanying JavaScript files with wasm-bindgen: 
```
wasm-bindgen --out-dir .\out --target web .\target\wasm32-unknown-unknown\release\the_game_of_gradients.wasm
```
This will generate a new directory ```.\out``` with a few files in it. Replace the corresponding files in this branch with these new ones and also make sure to replace the ```./assets``` folder if assets were changed. 