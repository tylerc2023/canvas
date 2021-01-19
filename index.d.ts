export interface DOMMatrix2DInit {
  a: number
  b: number
  c: number
  d: number
  e: number
  f: number
}

export class ImageData {
  /**
   * Returns the one-dimensional array containing the data in RGBA order, as integers in the range 0 to 255.
   */
  readonly data: Uint8ClampedArray
  /**
   * Returns the actual dimensions of the data in the ImageData object, in pixels.
   */
  readonly height: number
  /**
   * Returns the actual dimensions of the data in the ImageData object, in pixels.
   */
  readonly width: number

  constructor(sw: number, sh: number)
  constructor(data: Uint8ClampedArray, sw: number, sh?: number)
}

export class Image {
  readonly width: number
  readonly height: number
  readonly naturalWidth: number
  readonly naturalHeight: number
  readonly complete: boolean
  alt: string
  src: Buffer
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
}

export interface SKRSContext2D extends Omit<CanvasRenderingContext2D, 'drawImage'> {
  drawImage(image: Image, dx: number, dy: number): void
  drawImage(image: Image, dx: number, dy: number, dw: number, dh: number): void
  drawImage(
    image: Image,
    sx: number,
    sy: number,
    sw: number,
    sh: number,
    dx: number,
    dy: number,
    dw: number,
    dh: number,
  ): void
}

export interface Canvas extends Omit<HTMLCanvasElement, 'getContext'> {
  getContext(contextId: '2d'): SKRSContext2D
  png(): Promise<Buffer>
}

export function createCanvas(width: number, height: number): Canvas
