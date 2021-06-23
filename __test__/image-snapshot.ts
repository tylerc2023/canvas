import { promises as fs } from 'fs'
import { join } from 'path'

import PNG from '@jimp/png'
import JPEG from '@jimp/jpeg'
import { ExecutionContext } from 'ava'

const png = PNG()
const jpeg = JPEG()

export async function snapshotImage<C>(
  t: ExecutionContext<C>,
  context = t.context,
  type: 'png' | 'jpeg' | 'avif' | 'webp' | 'heif' = 'png',
) {
  // @ts-expect-error
  const { canvas } = context
  const image = await canvas.encode(type, 100)
  const ext = type === 'jpeg' ? 'jpg' : type
  const p = join(__dirname, 'snapshots', `${t.title}.${ext}`)

  async function writeFailureImage() {
    await fs.writeFile(join(__dirname, 'failure', `${t.title}.${ext}`), image)
  }

  let existed = true
  try {
    await fs.stat(p)
  } catch {
    existed = false
  }
  if (!existed) {
    await fs.writeFile(p, image)
    t.pass()
  } else {
    const existed = await fs.readFile(p)
    if (type !== 'png' && type !== 'jpeg') {
      t.pass()
      return
    }
    t.notThrowsAsync(async () => {
      const existedPixels =
        type === 'png' ? png.decoders['image/png'](existed).data : jpeg.decoders['image/jpeg'](existed).data
      const imagePixels =
        type === 'png' ? png.decoders['image/png'](image).data : jpeg.decoders['image/jpeg'](image).data
      if (existedPixels.length !== imagePixels.length) {
        await writeFailureImage()
        throw new Error('Image size is not equal')
      }
      let diffCount = 0
      imagePixels.forEach((u8, index) => {
        if (u8 !== existedPixels[index]) {
          diffCount++
        }
      })
      // diff ratio greater than 0.01%
      if (diffCount / existedPixels.length > 0.01 / 100) {
        await writeFailureImage()
        throw new Error('Image bytes is not equal')
      }
    })
  }
}
