# Rust Game Dev.(Rust, Rust_Fyrox, Blender)
<p align="center">
  <img width=120px src="https://user-images.githubusercontent.com/67513038/213436632-820a1675-98d9-4626-979d-be63c60cdcb7.png" hspace="20"/>
  <img width=120px src="https://github.com/YoungHaKim7/Cpp_Training/assets/67513038/6f1c2f3c-25e0-4191-8510-b0e7b26c6ea3" hspace="20"/>
  <br><img width=120px src="https://github.com/YoungHaKim7/Cpp_Training/assets/67513038/b96e6f3b-f5ba-4b3d-8f13-501b8d7b9870" hspace="20"/>
</p>


<hr>

# fyrox

https://fyrox.rs/

- eBook
  - https://fyrox-book.github.io/
    - Tutorial eBooks
      - https://fyrox-book.github.io/tutorials/tutorials.html

# rust_fyrox_project만들기 & init & fyrox 파일 구조

```
// install 
cargo install fyrox-template

// init 2d    이름이 여기를 수정해 주자 "fyrox_test"
fyrox-template init --name fyrox_test --style 2d

// init 3d    이름이 여기를 수정해 주자 "my_game"
fyrox-template init --name my_game --style 3d



cd fyrox_test

// fyrox editor 실행
cargo run --package editor --release
```

- file 내가 수정해야할곳
  - ```game/src/lib.rs``` 여기에서 수정해 주면 된다.

```
/my_3d_fyrox_game
 ├──🔒  Cargo.lock
▶├──􀌜  Cargo.toml
 ├──􀀂  data …
 ├──􀀂  editor
 │  ├──􀌜  Cargo.toml
 │  └──􀀂  src …
 ├──􀀂  executor
 │  ├──􀌜  Cargo.toml
 │  └──􀀂  src …
 ├──􀀂  executor-android
 │  ├──􀌜  Cargo.toml
 │  ├──􀇱  README.md
 │  └──􀀂  src …
 ├──􀀂  executor-wasm
 │  ├──􀌜  Cargo.toml
 │  ├──􀀀  index.html
 │  ├──􀀀  main.js
 │  └──􀇱  3 unlisted
└──􀀂  game
    ├──􀌜  game/Cargo.toml
    └──􀀂  game/src
       └──􀋒  game/src/lib.rs
```



# fyrox3d예전에 한 3d game

https://github.com/YoungHaKim7/fyrox3dgame_rpg

- 한글러스트Rust강의_046⭐️Rust_3d_RGP_game만들기3D_Game RG3D_Fyrox - a modern Rust Game engine2 #rustlang #fyrox
  - https://youtu.be/-GDO5ykuFoo?si=8De8yhjIY3ZcBwvd

- 여기로 변경되었다고 함(240719)
  - https://github.com/fyrox-book/fyrox-book.github.io/tree/main/src/code/tutorials
