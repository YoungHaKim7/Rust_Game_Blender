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

# link

- Rust <img width=35px src="https://user-images.githubusercontent.com/67513038/213436632-820a1675-98d9-4626-979d-be63c60cdcb7.png" hspace="3"/> https://www.rust-lang.org/
- Rust_Bevy <img width=80px src="https://bevyengine.org/assets/bevy_logo_dark.svg" hspace="3"/> https://bevyengine.org/
- Rust_Fyrox <img width=35px src="https://github.com/YoungHaKim7/Cpp_Training/assets/67513038/6f1c2f3c-25e0-4191-8510-b0e7b26c6ea3" hspace="3"/> https://fyrox.rs/

- Blender <img width=40px src="https://github.com/YoungHaKim7/Cpp_Training/assets/67513038/b96e6f3b-f5ba-4b3d-8f13-501b8d7b9870" hspace="3"/>https://www.blender.org/

- godot-rust https://godot-rust.github.io/book/index.html
  - https://github.com/godot-rust/gdext
  - <img width=95px src="https://godotengine.org/assets/logo_dark.svg" hspace="3"/> https://godotengine.org/

<hr>

- raytracing
  - audio
    - [A First Look At Raytraced Audio | Vercidium](https://youtu.be/u6EuAUjq92k?si=szrxLVxNQFpuddC3)

<hr />

- [게임엔진-비교-굿](#게임엔진-비교-굿)
- [royalty-free-video-games-music](#royalty-free-video-games-music)

- [Assets구성하는 기본틀_rust-게임개발-기본-구성](#rust-게임개발-기본-구성)

- [#무료블렌더-blender-3d캐릭터-만들때-필요하다-](#무료블렌더-blender-3d캐릭터-만들때-필요하다-)
  - [rust--blender-로-게임-만드는-demo영상](#rust--blender-로-게임-만드는-demo영상)
    - [blender최신-소식](#blender최신-소식)

<hr>

- [요즘은Bevy정리중.Blender랑 통합해서.(내가-공부하려고-정리-중..)](#내가-공부하려고-정리-중)

<hr>

- [러스트로 오래 살아남은 case VS. 러스트하다가 포기한 케이스](#러스트해서-살아남은-case)

<hr>

- [유니코드-검색하기-굿unicode-search](#유니코드-검색하기-굿unicode-search)

<hr>

- C++
  - [(C++) Unreal Engine 5 – Full Course for Beginners | freeCodeCamp.org](#c-unreal-engine-5--full-course-for-beginners--freecodecamporg)

<hr>

# 여기 죽어라 파야함!!Shadertoys ported to Rust GPU
- April 10, 2025
  - https://rust-gpu.github.io/blog/2025/04/10/shadertoys/

- 🐉 Making Rust a first-class language and ecosystem for GPU shaders 🚧
  - https://github.com/rust-gpu/rust-gpu

<hr />


# 류광님 짱 👍이곳은 IT 개발 전반과 게임 개발에 관한 소식과 정보를 공유하는 GpgStudy.com입니다.

- 게임 개발 번역서 질문&논의 :≪GPG 카탈로그≫, 기존 GgpgStudy 포럼: ≪GpgStudy 포럼≫.
  - https://gpgstudy.com/
- 번역가 류광의 홈페이지 '류광의 번역 이야기'
  - https://occamsrazr.net/

# 옥찬호님 game dev 로드맵
- https://github.com/utilForever/game-developer-roadmap

<hr />

# Royalty-Free Video Games Music[[🔝]](#link)

https://www.epidemicsound.com/

<hr>

# 게임엔진 비교 굿[[🔝]](#link)

- https://www.dragonflydb.io/game-dev/engines/rust

- https://arewegameyet.rs/

<hr>

<br>

# 내가 공부하려고 정리 중.[[🔝]](#link)

- Rust_Game_Dev 모아보기
  - https://youtube.com/playlist?list=PLcMveqN_07mY5cEcTgC4ICHnla6LSVtnh&si=aKiXhqr61OsLYAfH
    - Bevy정리하는 Github
      - https://github.com/YoungHaKim7/Rust_Bevy_Game_Engine

<hr>

# Rust 게임개발 기본 구성[[🔝]](#link)

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

<hr>

# 무료블렌더 Blender (3D캐릭터 만들때 필요하다. )[[🔝]](#link)
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

# Rust + Blender 로 게임 만드는 Demo영상[[🔝]](#link)
- Making an FPS game with Bevy and Rust!
  - https://youtu.be/06M2lT_I11c?si=ACv_8jUDmrWv2iXE

<hr>

# Blender최신 소식[[🔝]](#link)

# (240717)IT IS TIME! Blender 4.2, the next LTS release, has been released!
  - https://www.blender.org/download/releases/4-2/

You can also get it on Steam. If you do, make sure to use Steam's betas feature to pick 4.2 (or any other version you want!) so you don't get accidentally upgraded to 4.3 or so on later.

- Showcase Reel: https://www.youtube.com/watch?v=jTVG5kdHRQQ
- Video Recap: https://www.youtube.com/watch?v=iwpBgtKilvw

----

- Release notes: https://developer.blender.org/docs/release_notes/4.2/

<hr>

# 러스트해서 살아남은 case

# 러스트 게임 개발자(240717기준) 최근까지 살아남은 몇 안되는 Rust Dev.존경스럽다.[[🔝]](#link)
- Game dev in Rust - some notes on the mess
  - https://users.rust-lang.org/t/game-dev-in-rust-some-notes-on-the-mess/104939

## 러스트하다가 포기한 case

# 러스트 동시실행에서 무너진 개발자(거의 다 왔는데 바보...)[[🔝]](#link)
- **[GN⁺: Rust로 게임 개발을 한 3년 후에 떠나며](<https://news.hada.io/topic?id=14521&utm_source=discord&utm_medium=bot&utm_campaign=1480>)**
- Rust에 익숙해지면 모든 문제가 사라질 것이라는 주장에 대해  
  - Rust에 익숙해져도 근본적인 문제는 사라지지 않음  
  - 게임은 복잡한 상태 머신이고 요구사항이 계속 바뀌기 때문에 Rust의 정적이고 과도하게 검사하는 특성과 맞지 않음  
  - 코드를 계속 리팩토링해야 하는 문제는 self-inflicted임  
- ...

<hr>

# 유니코드 검색하기 굿(Unicode Search)[[🔝]](#link)
- https://unicodelookup.com/

<hr>

# (C++) Unreal Engine 5 – Full Course for Beginners | freeCodeCamp.org[[🔝]](#link)
- https://youtu.be/6UlU_FsicK8?si=GmblLzvswNeVNeDC

<hr>
