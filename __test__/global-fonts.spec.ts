import { readFileSync } from 'fs'
import { join } from 'path'
import test from 'ava'

import { GlobalFonts } from '../index'

const fontPath = join(__dirname, 'fonts', 'SourceSerifPro-Regular.ttf')
const fontData = readFileSync(fontPath)
const defaultCount = GlobalFonts.families.length

test('should be able to get global font family names', (t) => {
  t.notThrows(() => GlobalFonts.families)
})

test('should be able to register font and test font existence', (t) => {
  t.is(GlobalFonts.has('114514'), false)

  if (!GlobalFonts.has('Source Serif Pro')) {
    t.true(GlobalFonts.register(fontData))
    t.is(GlobalFonts.families.length, defaultCount + 1)
  } else {
    t.is(GlobalFonts.families.length, defaultCount)
  }
})

test('multiple identical fonts should only exist within one font family', (t) => {
  GlobalFonts.register(fontData)
  const count = GlobalFonts.families.length
  GlobalFonts.register(fontData)
  GlobalFonts.register(fontData)
  t.is(count, GlobalFonts.families.length)
})

test('return false if font path not existed', (t) => {
  t.false(GlobalFonts.register(Buffer.from('whatever')))
})
