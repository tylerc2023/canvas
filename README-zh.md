# `skr canvas`

![CI](https://github.com/Brooooooklyn/canvas/workflows/CI/badge.svg)
![Skia Version](https://img.shields.io/badge/Skia-chrome%2Fm106-hotpink)
[![install size](https://packagephobia.com/badge?p=@napi-rs/canvas)](https://packagephobia.com/result?p=@napi-rs/canvas)
[![Downloads](https://img.shields.io/npm/dm/@napi-rs/canvas.svg?sanitize=true)](https://npmcharts.com/compare/@napi-rs/canvas?minimal=true)

> ๐ ๅธฎๅฉๆๆไธบๅจ่ๅผๆบๅผๅ่: [Sponsoring me on Github](https://github.com/sponsors/Brooooooklyn)

้่ฟ [Node-API](https://napi.rs) ๅฐ Google Skia ็ปๅฎๅฐ Node.jsใ **้ถ็ณป็ปไพ่ต**ใ

> โ ๏ธ ่ฟไธช้กน็ฎ่ฟๅคไบ้ขๅๅธ้ถๆฎต๏ผๅฏ่ฝไผๅญๅจไธไบ Bugใ<br/>
> ๅจ่ฟ้ๅฏไปฅๆพๅฐๆญค้กน็ฎ็ๅทไฝๅผๅ่ฎกๅๅ่ทฏ็บฟๅพ [Roadmap](https://github.com/Brooooooklyn/canvas/issues/113)ใ

[English](./README.md)

# ๅฎ่ฃ

```bash
yarn add @napi-rs/canvas
npm install @napi-rs/canvas
```

# ๆฏๆ็็ณป็ปๅ Node.js ็ๆฌ

|                       | node10 | node12 | node14 | node16 |
| --------------------- | ------ | ------ | ------ | ------ |
| Windows x64           | โ      | โ      | โ      | โ      |
| macOS x64             | โ      | โ      | โ      | โ      |
| macOS arm64 (m chips) | โ      | โ      | โ      | โ      |
| Linux x64 gnu         | โ      | โ      | โ      | โ      |
| Linux x64 musl        | โ      | โ      | โ      | โ      |
| Linux arm64 gnu       | โ      | โ      | โ      | โ      |
| Linux arm64 musl      | โ      | โ      | โ      | โ      |
| Linux arm gnueabihf   | โ      | โ      | โ      | โ      |
| Linux arm64 android   | โ      | โ      | โ      | โ      |

## ็ณป็ป่ฆๆฑ

### `arm64`

**Linux** ้่ฆ [**_cortex-a57_**](https://en.wikipedia.org/wiki/ARM_Cortex-A57) ๆๆดๆฐ็ CPU ๆถๆใ

**macOS** ไธๆฏๆๆๆ Apple M ็ณปๅ่ฏ็ใ

### `armv7`

[**_cortex-a7_**](https://en.wikipedia.org/wiki/ARM_Cortex-A7) ๆๆดๆฐ็ CPU ๆถๆใ

### glibc

็ฑไบ Skia ไพ่ต [glibc](https://www.gnu.org/software/libc/) 2.18 ็ API๏ผๆไปฅๆๅฐ้่ฆๆจ็็ณป็ปไธญ glibc ็ๆฌ >= 2.18ใ

# ็จๆณ

```js
const { promises } = require('fs')
const { join } = require('path')
const { createCanvas } = require('@napi-rs/canvas')

const canvas = createCanvas(300, 320)
const ctx = canvas.getContext('2d')

ctx.lineWidth = 10
ctx.strokeStyle = '#03a9f4'
ctx.fillStyle = '#03a9f4'

// ๅข
ctx.strokeRect(75, 140, 150, 110)

// ้จ
ctx.fillRect(130, 190, 40, 60)

// ๅฑ้กถ
ctx.beginPath()
ctx.moveTo(50, 140)
ctx.lineTo(150, 60)
ctx.lineTo(250, 140)
ctx.closePath()
ctx.stroke()

async function main() {
  const pngData = await canvas.encode('png') // ไนๆฏๆ JPEG ใAVIF ๅ WebP
  // encoding in libuv thread pool, non-blocking
  await promises.writeFile(join(__dirname, 'simple.png'), pngData)
}

main()
```

![](./example/simple.png)

## Emoji ๆๅญ

```js
const { writeFileSync } = require('fs')
const { join } = require('path')

const { createCanvas, GlobalFonts } = require('../index.js')

GlobalFonts.registerFromPath(join(__dirname, '..', 'fonts', 'AppleColorEmoji@2x.ttf'), 'Apple Emoji')
GlobalFonts.registerFromPath(join(__dirname, '..', '__test__', 'fonts', 'COLRv1.ttf'), 'COLRv1')

console.info(GlobalFonts.families)

const canvas = createCanvas(760, 360)
const ctx = canvas.getContext('2d')

ctx.font = '50px Apple Emoji'
ctx.strokeText('๐๐๐๐๐๐๐๐คฃโบ๏ธ๐๐๐', 50, 150)

ctx.font = '100px COLRv1'
ctx.fillText('abc', 50, 300)

const b = canvas.toBuffer('image/png')

writeFileSync(join(__dirname, 'draw-emoji.png'), b)
```

![](./example/draw-emoji.png)

# ๆง่ฝ

[benchmark](./benchmark) ไปฃ็ ใ

็กฌไปถไฟกๆฏ:

```
OS: Windows 10 x86_64
Host: Micro-Star International Co., Ltd. MS-7C35
Kernel: 10.0.19043
Terminal: Windows Terminal
CPU: AMD Ryzen 9 5950X (32) @ 3.400GHz
Memory: 32688MiB
```

```
โฏ yarn bench

> @napi-rs/canvas@0.0.9 bench D:\workspace\skia-rs
> node -r @swc-node/register benchmark/bench.ts

Running "Draw house" suite...
Progress: 100%

  skia-canvas:
    26 ops/s, ยฑ0.70%   | slowest, 29.73% slower

  node-canvas:
    30 ops/s, ยฑ6.95%   | 18.92% slower

  @napi-rs/canvas:
    37 ops/s, ยฑ6.30%   | fastest

Finished 3 cases!
  Fastest: @napi-rs/canvas
  Slowest: skia-canvas
Running "Draw gradient" suite...
Progress: 100%

  skia-canvas:
    36 ops/s, ยฑ6.12%   | 14.29% slower

  node-canvas:
    34 ops/s, ยฑ5.60%   | slowest, 19.05% slower

  @napi-rs/canvas:
    42 ops/s, ยฑ0.53%   | fastest

Finished 3 cases!
  Fastest: @napi-rs/canvas
  Slowest: node-canvas
```

# ๅ่ฝ

## Path2D

```typescript
new Path2D()
new Path2D(path: Path2D)
// new Path2D('M108.956,403.826c0,0,0.178,3.344-1.276,3.311  c-1.455-0.033-30.507-84.917-66.752-80.957C40.928,326.18,72.326,313.197,108.956,403.826z')
new Path2D(path: string)
```

```typescript
export interface DOMMatrix2DInit {
  a: number
  b: number
  c: number
  d: number
  e: number
  f: number
}

export class Path2D {
  constructor(path?: Path2D | string)

  addPath(path: Path2D, transform?: DOMMatrix2DInit): void
  arc(x: number, y: number, radius: number, startAngle: number, endAngle: number, anticlockwise?: boolean): void
  arcTo(x1: number, y1: number, x2: number, y2: number, radius: number): void
  bezierCurveTo(cp1x: number, cp1y: number, cp2x: number, cp2y: number, x: number, y: number): void
  closePath(): void
  ellipse(
    x: number,
    y: number,
    radiusX: number,
    radiusY: number,
    rotation: number,
    startAngle: number,
    endAngle: number,
    anticlockwise?: boolean,
  ): void
  lineTo(x: number, y: number): void
  moveTo(x: number, y: number): void
  quadraticCurveTo(cpx: number, cpy: number, x: number, y: number): void
  rect(x: number, y: number, w: number, h: number): void

  // PathKit methods
  op(path: Path2D, operation: PathOp): Path2D
  toSVGString(): string
  getFillType(): FillType
  getFillTypeString(): string
  setFillType(type: FillType): void
  simplify(): Path2D
  asWinding(): Path2D
  stroke(stroke?: StrokeOptions): Path2D
  transform(transform: DOMMatrix2DInit): Path2D
  getBounds(): [left: number, top: number, right: number, bottom: number]
  computeTightBounds(): [left: number, top: number, right: number, bottom: number]
  trim(start: number, end: number, isComplement?: boolean): Path2D
  equals(path: Path2D): boolean
}
```

## PathKit

PathKit ๆฏ Skia ไธญ็จๆฅๆไฝ Path ็ๅทฅๅท้๏ผๆฏๆไบๆฌก่ดๅกๅฐๆฒ็บฟ๏ผquadratic beziers๏ผใไธๆฌก่ดๅกๅฐๆฒ็บฟ๏ผcubic beziers๏ผๅๅ้ฅๅฝข๏ผconics๏ผใ
ไธป่ฆ็ๅ่ฝๆ๏ผ

### ๅธๅฐ่ฟ็ฎ

`.op(path, PathOp)`

```js
const pathOne = new Path2D(
  'M8 50H92C96.4183 50 100 53.5817 100 58V142C100 146.418 96.4183 150 92 150H8C3.58172 150 0 146.418 0 142V58C0 53.5817 3.58172 50 8 50Z',
)
const pathTwo = new Path2D(
  '"M58 0H142C146.418 0 150 3.58172 150 8V92C150 96.4183 146.418 100 142 100H58C53.5817 100 50 96.4183 50 92V8C50 3.58172 53.5817 0 58 0Z',
)

pathOne.op(pathTwo, PathOp.Intersect).toSVGString()
// => "M100 100L58 100C53.5817 100 50 96.4183 50 92L50 50L92 50C96.4183 50 100 53.5817 100 58L100 100Z"
```

- **Union** ่้๏ผๅนถ้๏ผ
- **Difference** ๅๅป้กถๅฑ
- **ReverseDifference** ๅๅปๅบๅฑ
- **Intersect** ไบค้
- **XOR** ๅทฎ้

![boolean-operations](./docs/imgs/boolean-operations.svg)

### ่ฝฌๆข **_Path_** ็ `FillType`

`.asWinding()`

ๅฏไปฅๆ SVG ไธญ็ `fill-rule="evenodd"` ่ฝฌๆขไธบ `fill-rule="nonzero"`๏ผ
่ฟๅฏน OpenType ๅญไฝ็ธๅณๅทฅๅท้ๅธธๆ็จ๏ผๅ ไธบ OpenType ๅญไฝไธญๅชๆฏๆ `fill-rule="nonzero"`ใ

![SVG fill-rule](./docs/imgs/asWinding@2x.png)

```js
const pathCircle = new Path2D(
  'M24.2979 13.6364H129.394V40.9091H24.2979L14.6278 27.2727L24.2979 13.6364ZM21.9592 0C19.0246 0 16.2716 1.42436 14.571 3.82251L1.67756 22.0043C-0.559186 25.1585 -0.559186 29.387 1.67756 32.5411L14.571 50.7227C16.2716 53.1209 19.0246 54.5455 21.9592 54.5455H70.4673V68.1818H16.073C11.0661 68.1818 7.00728 72.2518 7.00728 77.2727V113.636C7.00728 118.657 11.0661 122.727 16.073 122.727H70.4673V150H84.0658V122.727H128.041C130.975 122.727 133.729 121.303 135.429 118.905L148.323 100.723C150.559 97.5686 150.559 93.3405 148.323 90.1864L135.429 72.0045C133.729 69.6064 130.975 68.1818 128.041 68.1818H84.0658V54.5455H133.927C138.934 54.5455 142.993 50.4755 142.993 45.4545V9.09091C142.993 4.07014 138.934 0 133.927 0H21.9592ZM125.702 109.091H20.6058V81.8182H125.702L135.372 95.4545L125.702 109.091Z',
)
pathCircle.setFillType(FillType.EvenOdd)
pathCircle.asWinding().toSVGString()
// => "M24.2979 13.6364L129.394 13.6364L129.394 40.9091L24.2979 40.9091L14.6278 27.2727L24.2979 13.6364ZM21.9592 0C19.0246 0 16.2716 1.42436 14.571 3.82251L1.67756 22.0043C-0.559186 25.1585 -0.559186 29.387 1.67756 32.5411L14.571 50.7227C16.2716 53.1209 19.0246 54.5455 21.9592 54.5455L70.4673 54.5455L70.4673 68.1818L16.073 68.1818C11.0661 68.1818 7.00728 72.2518 7.00728 77.2727L7.00728 113.636C7.00728 118.657 11.0661 122.727 16.073 122.727L70.4673 122.727L70.4673 150L84.0658 150L84.0658 122.727L128.041 122.727C130.975 122.727 133.729 121.303 135.429 118.905L148.323 100.723C150.559 97.5686 150.559 93.3405 148.323 90.1864L135.429 72.0045C133.729 69.6064 130.975 68.1818 128.041 68.1818L84.0658 68.1818L84.0658 54.5455L133.927 54.5455C138.934 54.5455 142.993 50.4755 142.993 45.4545L142.993 9.09091C142.993 4.07014 138.934 0 133.927 0L21.9592 0ZM125.702 109.091L20.6058 109.091L20.6058 81.8182L125.702 81.8182L135.372 95.4545L125.702 109.091Z"
```

### ็ฎๅ **_Path_**

`.simplify()`

ๅฐ่ทฏๅพ่ฎพ็ฝฎไธบไธๅ่ทฏๅพๅบๅ็ธๅ็้้ๅ ่ฝฎๅป๏ผnon-overlapping๏ผ๏ผไนๅฐฑๆฏ่ฏดๅฎ่ฟๅฏไปฅ็งป้ค้ๅ ่ทฏๅพใ

<img width="800" src="./docs/imgs/simplify.png" >

[ๅธฆๆ้ๅ ่ทฏๅพ็ SVG](./docs/imgs/overlapping-path.svg)๏ผๅทฆไพง๏ผ

```js
const path =
  'M2.933,89.89 L89.005,3.818 Q90.412,2.411 92.249,1.65 Q94.087,0.889 96.076,0.889 Q98.065,0.889 99.903,1.65 Q101.741,2.411 103.147,3.818 L189.22,89.89 Q190.626,91.296 191.387,93.134 Q192.148,94.972 192.148,96.961 Q192.148,98.95 191.387,100.788 Q190.626,102.625 189.219,104.032 Q187.813,105.439 185.975,106.2 Q184.138,106.961 182.148,106.961 Q180.159,106.961 178.322,106.2 Q176.484,105.439 175.077,104.032 L89.005,17.96 L96.076,10.889 L103.147,17.96 L17.075,104.032 Q15.668,105.439 13.831,106.2 Q11.993,106.961 10.004,106.961 Q8.015,106.961 6.177,106.2 Q4.339,105.439 2.933,104.032 Q1.526,102.625 0.765,100.788 Q0.004,98.95 0.004,96.961 Q0.004,94.972 0.765,93.134 Q1.526,91.296 2.933,89.89 Z'

path.simplify().toSVGString()
// => "M89.005 3.818L2.933 89.89Q1.526 91.296 0.765 93.134Q0.004 94.972 0.004 96.961Q0.004 98.95 0.765 100.788Q1.526 102.625 2.933 104.032Q4.339 105.439 6.177 106.2Q8.015 106.961 10.004 106.961Q11.993 106.961 13.831 106.2Q15.668 105.439 17.075 104.032L96.076 25.031L175.077 104.032Q176.484 105.439 178.322 106.2Q180.159 106.961 182.148 106.961Q184.138 106.961 185.975 106.2Q187.813 105.439 189.219 104.032Q190.626 102.625 191.387 100.788Q192.148 98.95 192.148 96.961Q192.148 94.972 191.387 93.134Q190.626 91.296 189.22 89.89L103.147 3.818Q101.741 2.411 99.903 1.65Q98.065 0.889 96.076 0.889Q94.087 0.889 92.249 1.65Q90.412 2.411 89.005 3.818Z"
```

# [Example](./example/tiger.js)

> The tiger.json was serialized from [gojs/samples/tiger](https://github.com/NorthwoodsSoftware/GoJS/blob/master/samples/tiger.html)

<img width="500" src="example/tiger.png">

```shell
node example/anime-girl.js
```

| SVG                                            | PNG                                            |
| ---------------------------------------------- | ---------------------------------------------- |
| <img width="500" src="example/anime-girl.svg"> | <img width="500" src="example/anime-girl.png"> |

# ็ผ่ฏ้กน็ฎ

## ไปๆบ็ ็ผ่ฏ

ไฝ ๅฏไปฅไปๆบ็ ็ผ่ฏๆญค้กน็ฎ๏ผๆ ้ๅฎ่ฃไปปไฝ้ขๅค็็ณป็ปไพ่ต:

```sh
# Clone the code:
$ git clone --recurse-submodules https://github.com/Brooooooklyn/canvas.git
$ cd canvas

# Build Skia:
$ node scripts/build-skia.js

# Install NPM packages, build the Node.js addon:
$ yarn install --ignore-scripts
$ yarn build

# All done! Run test cases or examples now:
$ yarn test
$ node example/tiger.js
```

## ไป GitHub ไธไธ่ฝฝ้ข็ผ่ฏ็ `skia` ไบ่ฟๅถ

ๅฆๆไฝ ๅชๅณๅฟ้กน็ฎ็ `Rust` ้จๅ๏ผๅฏไปฅไธ่ฝฝ้ข็ผ่ฏ็ `skia` ไบ่ฟๅถ้ๆ้พๆฅๆไปถ:

```sh
# Clone the code:
$ git clone --recurse-submodules https://github.com/Brooooooklyn/canvas.git
$ cd canvas

# Download Skia binaries:
# It will pull the binaries match the git hash in `./skia` submodule
$ node scripts/release-skia-binary.js --download

# Install NPM packages, build the Node.js addon:
$ yarn install --ignore-scripts
$ yarn build

# All done! Run test cases or examples now:
$ yarn test
$ node example/tiger.js
```
