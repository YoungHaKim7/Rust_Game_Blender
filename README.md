# Rust_Game_Blender
Rust Game Dev.(Rust, Rust_Bevy, Rust_Fyrox, Blender)
<p align="center">
  <img width=120px src="https://user-images.githubusercontent.com/67513038/213436632-820a1675-98d9-4626-979d-be63c60cdcb7.png" hspace="20"/>
  <img width=200px src="https://bevyengine.org/assets/bevy_logo_dark.svg" hspace="20" />
  <img width=120px src="https://github.com/YoungHaKim7/Cpp_Training/assets/67513038/6f1c2f3c-25e0-4191-8510-b0e7b26c6ea3" hspace="20"/>
  <br><img width=120px src="https://github.com/YoungHaKim7/Cpp_Training/assets/67513038/b96e6f3b-f5ba-4b3d-8f13-501b8d7b9870" hspace="20"/>
  <img width=180px src="https://godotengine.org/assets/logo_dark.svg" hspace="3"/>
</p>

<hr>

- Rust <img width=35px src="https://user-images.githubusercontent.com/67513038/213436632-820a1675-98d9-4626-979d-be63c60cdcb7.png" hspace="3"/> https://www.rust-lang.org/
- Rust_Bevy <img width=80px src="https://bevyengine.org/assets/bevy_logo_dark.svg" hspace="3"/> https://bevyengine.org/
- Rust_Fyrox <img width=35px src="https://github.com/YoungHaKim7/Cpp_Training/assets/67513038/6f1c2f3c-25e0-4191-8510-b0e7b26c6ea3" hspace="3"/> https://fyrox.rs/

- Blender <img width=40px src="https://github.com/YoungHaKim7/Cpp_Training/assets/67513038/b96e6f3b-f5ba-4b3d-8f13-501b8d7b9870" hspace="3"/>https://www.blender.org/

- godot-rust https://godot-rust.github.io/book/index.html
  - https://github.com/godot-rust/gdext
  - <img width=95px src="https://godotengine.org/assets/logo_dark.svg" hspace="3"/> https://godotengine.org/

<hr>

# Royalty-Free Video Games Music

https://www.epidemicsound.com/

<hr>

# 게임엔진 비교 굿

- https://www.dragonflydb.io/game-dev/engines/rust

<hr>

<br>

# 내가 공부하려고 정리 중.

- Rust_Game_Dev 모아보기
  - https://youtube.com/playlist?list=PLcMveqN_07mY5cEcTgC4ICHnla6LSVtnh&si=aKiXhqr61OsLYAfH

<hr>


# 무료블렌더 Blender (3D캐릭터 만들때 필요하다. )
- blender.org - Home of the Blender project - Free and Open 3D ...
blender.org
https://www.blender.org
Blender is a public project hosted on blender.org, licensed as GNU GPL, owned by its contributors. For that reason Blender is Free and Open Source software, ...
https://www.blender.org/

- 3D 왕초보 블렌더 #01 일단 한번 열어서 움직여 보자 | Blender 튜토리얼
  - https://youtu.be/qlaJbG1vVHc?si=kTQDJ-iLQDLhDn_g
- 블렌더 완전 기초 강의 3.3 LTS
  - https://youtube.com/playlist?list=PLqf2JB4ViQO5JhlXD9bXDerr-w3frv6Np&si=fZbaeAwKbozZrl40

- R3F(React Three Fiber)를 이용한 3D 웹 개발 : 10. 3D 모델 / 애니메이션
  - https://youtu.be/S7qWQ1li32o?si=KqMTvoF_RuqL9Kp4

- 블렌더로 직접 만드는 영상 I animated this in 18 days... in Blender
  - https://youtu.be/tCTkkHGRpNk?si=O8qUVhc5qq2_CVkx
    - The Best Way To Create Nature In 3D (진짜 잔디 같다. ㅎ)
      - https://youtu.be/7Um3FaXJixg?si=Bvt5wN-Ye2JaHu2U
    - 도시 만드는 영상Creating The Last Of Us Environment In Blender 3.4
      - https://youtu.be/OKmfqixt_l4?si=D0hS_NbyB4rzsnbh
    - 진정한 리얼 ㅎRealistic Environment With Blender 3D
      - https://youtu.be/v3NrfXaJW6c?si=wID5-qnvxXF4uvWN

# Rust + Blender 로 게임 만드는 Demo영상
- Making an FPS game with Bevy and Rust!
  - https://youtu.be/06M2lT_I11c?si=ACv_8jUDmrWv2iXE

<hr>

# Rust 게임개발 기본 구성 

- 외국 사람의 Github 참고 

  - https://github.com/DigitalExtinction/Game

- Assets 파일 대략 구성 

```
$ tree
.
├── audio
│   ├── music
│   │   └── menu_loop.mp3
│   └── sounds
│       ├── construct.ogg
│       ├── destruction_building.ogg
│       ├── destruction_unit.ogg
│       ├── laser.ogg
│       └── manufacture.ogg
├── fonts
│   └── Fira_Mono
│       ├── FiraMono-Medium.ttf
│       └── LICENSE
├── maps
│   ├── 8a9d5f0e522cc1aac64c45f0d4da353eccb410a00c04c84a23788e5ca5c01e2e.dem.tar
│   └── c653d17ba9a26c2d58c8a8723f37c881971207c330853764441a16df35ec7521.dem.tar
├── models
│   ├── attacker.glb
│   ├── base.glb
│   ├── pole.glb
│   ├── powerhub.glb
│   └── tree.glb
├── objects
│   ├── attacker.obj.json
│   ├── base.obj.json
│   ├── powerhub.obj.json
│   └── tree.obj.json
├── shaders
│   ├── bar.wgsl
│   ├── rally_point.wgsl
│   ├── terrain.wgsl
│   └── trail.wgsl
└── textures
    ├── skybox.png
    └── terrain.png

11 directories, 25 files
```

- ogg 소리 파일 공부 https://cloudinary.com/guides/video-formats/ogg-format-an-in-depth-look

