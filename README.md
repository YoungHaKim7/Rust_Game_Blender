# Rust_Game_Blender
Rust Game Dev.


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


