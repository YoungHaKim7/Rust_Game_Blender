# link
- [블렌더 최신 릴리즈 버젼 업데이트 소식Blender news](#blender-업데이트-뉴스-모음)
- [Blender Cheat Sheets](#blender-cheat-sheets9min31sec)
  - [generalmost-window-types](#generalmost-window-types)
  - [navigation3d-viewport](#navigation3d-viewport)
  - [view-3d-viewport](#view-3d-viewport)
  - [object-mode-3d-viewport](#object-mode-3d-viewport)
  - [general-selectionmost-window-types](#general-selectionmost-window-types)
  - [shading-3d-viewport](#shading-3d-viewport)
  - [selectionedit-mode](#selectionedit-mode)
  - [curve-editingedit-mode](#curve-editingedit-mode)
  - [texturingedit-mode](#texturingedit-mode)
  - [uv-editor](#uv-editor)
  - [image-editor-view](#image-editor-view)
  - [nodesmaterials--compositor](#nodesmaterials--compositor)
  - [compositor](#compositor)

- [blender-plugin-블렌더-플러그인](#blender-plugin-블렌더-플러그인)
  - [플러그인maps-in-blender-35-using-the-blendergis-plugin--radical-graphics](#블렌더-플러그인-굿maps-in-blender-35-using-the-blendergis-plugin--radical-graphics)

<hr>

- [무료블렌더-blender-3d캐릭터-만들때-필요하다-](#무료블렌더-blender-3d캐릭터-만들때-필요하다-)
  - [블랜더-기초-tutorial](#블랜더-기초-tutorial)
    - [블렌더 강의 3 머그컵 모델링 + 뷰포트와 셀렉션 | DESIGN COSMOS](https://youtu.be/sgc6XzZV9jc?si=AeCNln8nMCA8D8V6)
    - [메이저 게임 3D 캐릭터 모델러의 비법을 녹여낸 블렌더 3D 기초 클래스 | 따즈아](https://youtu.be/mVGeT-1lNg0?si=D8q1ZZzbaMygIpOA)
  - [blendercharacter-modeling-for-beginners](#blendercharacter-modeling-for-beginners)

- [geometry-nodes-모아보기-굿---open-class](#geometry-nodes-모아보기-굿---open-class)

<hr>

# Blender 업데이트 뉴스 모음[|🔝|](#link)
- [250324 Blender4.4업데이트 정리)거절하기 힘든 블렌더 업그레이드 타이밍 | 런타임](https://youtu.be/vYbo8tgLXQM?si=SA8X7KYhsOLHb63r)
- [241120 Blender4.3업데이트 정리)블렌더 선수의 증명 | 런타임](https://youtu.be/eXmapz-mawE?si=YIsl02kGUlGz1z7X)
  - [4.3 신기능 정리_각잡은 블렌더 4.3의 신기능 | 런타임](https://youtu.be/GhV-8OCLAsA?si=GpqceyWiXWYma2X1)
- [240327 Blender4.1업데이트 정리 따끈한 블렌더가 왔어요! | 런타임](https://youtu.be/CZtxRtkXgHI?si=E0-ssFutNgxnoaxr)
  - [블렌더 4.1 신기능 평가 | 런타임](https://youtu.be/jD8l9ve2sWA?si=S1JfgHF5f3z5Lm7U)


# Blender Market (Assets들 사고 파는곳인듯)[|🔝|](#link)

- https://blendermarket.com/

<hr>

# (250201)Blender 4.4 Alpha ranked gameplay EU low ping grandmaster high MMR | Polyfjord[|🔝|](#link)
- https://www.youtube.com/live/cQJMbnJOsD8?si=Ql9Rt9_oCv9fzEk1

<hr />


# Blender Cheat Sheets(9min31sec)[|🔝|](#link)
- https://youtu.be/B0J27sf9N1Y?si=oLug9wsqjn7ilOfb

- `LMB` means click or press the Left-mouse button.
- `RMB` means click or press the Right-mouse button.
   - https://learn.foundry.com/katana/3.0/Content/ug/appendix_hotkeys/hotkeys.html
    
# General(most window types)[|🔝|](#link)

|||
|-|-|
|Toolbar|T|
|Properties|N|
|Add Object/Node| Shift + A<br> ⇧ + A|
|Delete| X or Delete|
|Search|F3|
|Move|G|
|Scale|S|
|Rotate|R|
|Rotate along axis|R then X/Y/Z|
|Rotate along local axis|R then X, X/Y, Y/Z, Z|
|Trackball Rotate|R, R|
|Precise movement|Shift(hold)|
|Incremental movement|Ctrl(hold)|
|Duplicate|Shift + D|
|Duplicate Linked|Alt + D|
|Hide|H|
|Unhide All|Alt + H|
|Hide all Except Selected|Shift +H|
|Annotate|D(hold) + LMB(drag)|
|Erase Annotation|D(hold) +RMB(drag)|
|Click favs menu|Q|
|||


<hr>

# Navigation(3D viewport)[[🔝]](#link)

|||
|-|-|
|Orbit|MMB|
|Pan|Shift + MMB|
|Zoom In/Out|Scroll or Ctrl + MMB|
|Fly|Shift + ~|
|||

<hr>

# View (3D viewport)[[🔝]](#link)

- Numpad views:
  
||||
|-|-|-|
||/<br>Isolate||
|7<br>Top|8<br>Up|9<br>Opposite|
|4<br>Left|5<br>Persp/Ortho|6<br>Right|
|1<br>Front|2<br>Down|3<br>Side|
||0<br>Camera|.<br>Focus|
||||


|||
|-|-|
|View Pie Menu|-|
|Fast View Switch|Alt + MMB(drag)|
|Show All Objects|Home|
|Focus to region| Shift+ B|
|||

<hr>

# Object Mode (3D viewport)[[🔝]](#link)

|||
|-|-|
|Mode Pie Menu|Ctrl + TAB|
|Edit/Object mode<br>toggle|TAB|
|Mirror|Ctrl+ M then X/Y/Z<br>(or MMB(drag)|
|Set Parent|Ctrl + P|
|Clear Parent| Alt + P|
|Toggle Snapping |Shift + TAB|
|Clear Location |Alt + G|
|Clear Rotation|Alt + R|
|Clear Scale |Alt + S|
|Apply Location/<br>Scale / Rotation|Ctrl + A|
|Join Selected<br>Objects|Ctrl + J|
|Copy Attributes to<br>New Objects|Ctrl+L|
|Add Subdivision <br>level|Ctrl + 0/1/2/3/4/5|
|Mask view to region <br> Clear mask|Alt + B|
|Center 3D cursor|Shift + C|
|Move active object<br>to Collection|M|
|Move Active <br>Camera to view|Ctrl + Alt + NumPad 0|
|Set as Active Camera|Ctrl + Numpad 0|
|||

<hr>

# General Selection(most window types)[[🔝]](#link)

|||
|-|-|
|Select | LMB|
|Select All| A|
|Deselect All|Alt + A|
|Marquee Box<br>Select|B|
|Circle Select|C|
|Lasso Select|Ctrl + RMB|
|Invert Selection|Ctrl + i|
|Select Linked|Shift + L |
|Select Similar|Shift + G|
|Select specific<br>object|Alt + LMB|
|||

<hr>

# Shading (3D viewport)[[🔝]](#link)

|||
|-|-|
|Shading Pie Menu|Z|
|Toggle X-Ray|Alt + Z|
|||

<hr>

# Pie Menus[[🔝]](#link)

|||
|-|-|
|Pivot point pie<br>menu|.|
|Snap pie menu|Shift + S|
|Orientation pie<br>menu|,|
|||


<hr>

# Selection(Edit Mode)[[🔝]](#link)

|||
|-|-|
|Select Connected<br>Mesh|Ctrl + L|
|Select Connected <br> Mesh Under Cursor|L|
|Select Edge/Face<br>Loop|Alt + LMB|
|Select Edge Ring|Ctrl + Alt + RMB|
|Vertex Select Mode|1|
|Edge Select Mod|2|
|Face Select Mode|3|
|Mirror current<br>selection|Ctrl + Shift + M|
|Select More/Less|Ctrl +/-|
|Edge Crease|Shift + E|
|||

<hr>

# Curve Editing(Edit Mode)[[🔝]](#link)

|||
|-|-|
|Add new handle|E or <br> Ctrl + RMB|
|Change ahndle type| V|
|Delete but maintain<br>connection|Ctrl + X|
|Close curve|Alt + C|
|Tilt|Ctrl + T|
|Clear Tilt|Alt + T|
|||

<hr>

# Modeling(Edit Mode)[[🔝]](#link)

|||
|-|-|
|Extrude|E|
|Inset|i|
|Bevel|Ctrl + B|
|Bevel Vertices|Ctrl + Shift + B|
|Loop cut|Ctrl + R|
|Vertex/Edge Slide|G,G|
|Knife|K|
|Fill Face|F|
|Shear|Ctrl + Shift + Alt + S|
|Bend|Shift + W|
|Split|Y|
|Rip|V|
|Rip Fill|Alt +V|
|Merge|M|
|Recalculate<br>Normals|Shift + N|
|Flip Normals|Ctrl + Shift + N|
|Proportional Editing<br> On/Off|O|
|Proportional Falloff<br> Type|Shift + O|
|Separate selection<br>to new object|P|
|||


<hr>

# Texturing(Edit Mode)[[🔝]](#link)

|||
|-|-|
|Unwrap |U|
|Mark Seam|Ctrl + E|
|||

<hr>

# UV Editor[[🔝]](#link)

|||
|-|-|
|Select Island| L (under cursor) or <br> Ctrl + L|
|Stitch|V|
|Weld|Shift + W|
|Pin|P|
|Unpin|Alt + P|
|Select Pinned|Shift + P|
|||

<hr>

# Image Editor (View)[[🔝]](#link)

|||
|-|-|
|Properties, Scopes,<br>Slots and Metadata|N|
|View at 100%|1(Numpad)|
|View to Fit|Shift + Home|
|Next Render Slot |J|
|Previous Render<br>Slot|Alt + J|
|Select Render Slot |1-8|
|Save Image|Alt + S|
|Save Image As|Shift + S|
|||

<hr>

# Image Editor (Paint)[[🔝]](#link)

|||
|-|-|
|Create New Blank<br>Image|Alt + N|
|Open Image|Alt + O|
|Brush Properties|N|
|Brush Size|F|
|Brush Strength|Shift + F|
|Sample Color|S|
|Flip Color|X|
|||

<hr>

# Nodes(Materials / Compositor)[[🔝]](#link)

|||
|-|-|
|Cut Connection|Ctrl + RMB(drag)|
|Reroute<br>Connection|Shift + RMB(drag)|
|Connect selected|F|
|Properties|N|
|Delete selected but<br>maintain<br>connection|Ctrl + X|
|Duplicate selected<br>and maintain<br>connection|Ctrl + Shift + D|
|Mute Selected |M|
|Group Selected |Ctrl + G|
|Ungroup Selected|Ctrl + Alt + G|
|Edit Group (Toggle)|TAB|
|Frame Selected<br>Nodes|Ctrl + J|
|Show/Hide inactive<br>node slots|Ctrl + H|
|||

<hr>


# Compositor[[🔝]](#link)

|||
|-|-|
|Move backdrop|Alt + MMB|
|Zoom backdrop|V/Alt + V|
|Properties and<br>performance|N|
|||

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
    - Cascadeur - Is This the Future of Animation
      - https://youtu.be/JrddPZmUHvE?si=li_DlfdOdBAmYwCv
    - 블렌더 캐릭터 모델링 - Yuyuki part04(Blender character modeling - Yuyuki part04) | Na Merge
      -  https://youtu.be/Pi5hsB5jDLc?si=MBGcoovbOX8r0pAC
    - [블렌더 강좌] 모델링 따라하기 : 공중부양 시계 만들기 종합 | Bravo Your Life.THE100_더백
      - https://youtu.be/E0Ijw1tsv7I?si=I81Sx3sP5hW4-9KK
        - 더백클래스(블렌더) 시리즈 Bravo Your Life.THE100_더백
          - https://youtube.com/playlist?list=PLTmUI8HoTmE74gPdS8WumeDKhGLFHwxV7&si=Km8Ot2IHPcYybZm7
    - 블렌더4.0_blender maker_ ep2 art _이제는 한글로 배운다_영문은 어렵다? | 오리듀_oridoou
      - https://youtu.be/-fn_E8o59-Q?si=MjVX2Nsb4732JWsP

# 블랜더 기초 Tutorial[[🔝]](#link)
- Blender 4.0 Beginner Donut Tutorial (NEW) | Blender Guru
  - https://youtube.com/playlist?list=PLjEaoINr3zgEPv5y--4MKpciLaoQYZB1Z&si=NBbNiy41wX0Ee7_4

- Getting started - Blender for complete beginners | Joey Carlino
  -  https://youtu.be/uOmYInaX-wE?si=UdWj4Ong0U3G89Pt


# Blender(Character modeling for beginners)[[🔝]](#link)
- Character modeling for beginners - Blender | Joey Carlino
  - https://youtu.be/O6HQhs-gk50?si=eF18Tir-JpvNfsE0

<hr>

# Geometry Nodes (모아보기 굿 👍 )| Open Class[[🔝]](#link)
- https://youtube.com/playlist?list=PLhVUZkOKttOw_19Qo63WYW7A8oZmec5BG&si=9fWvawb9CbyOy2nd

<br>

<hr>

# Blender Plugin 블렌더 플러그인[[🔝]](#link)

# (블렌더 플러그인 굿)Maps in Blender 3.5 using the BlenderGIS Plugin | Radical Graphics[[🔝]](#link)
https://youtu.be/PmHxBn7F9Fw?si=gKKoYxK-RmqVeiWI
