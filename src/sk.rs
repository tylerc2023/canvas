use std::ops::{Deref, DerefMut};
use std::slice;
use std::str::FromStr;

use thiserror::Error;

mod ffi {
  #[repr(C)]
  #[derive(Copy, Clone, Debug)]
  pub struct skiac_surface {
    _unused: [u8; 0],
  }

  #[repr(C)]
  #[derive(Copy, Clone, Debug)]
  pub struct skiac_canvas {
    _unused: [u8; 0],
  }

  #[repr(C)]
  #[derive(Copy, Clone, Debug)]
  pub struct skiac_paint {
    _unused: [u8; 0],
  }

  #[repr(C)]
  #[derive(Copy, Clone, Debug)]
  pub struct skiac_path {
    _unused: [u8; 0],
  }

  #[repr(C)]
  #[derive(Copy, Clone, Debug)]
  pub struct skiac_matrix {
    _unused: [u8; 0],
  }

  #[repr(C)]
  #[derive(Copy, Clone, Debug)]
  pub struct skiac_shader {
    _unused: [u8; 0],
  }

  #[repr(C)]
  #[derive(Copy, Clone, Debug)]
  pub struct skiac_path_effect {
    _unused: [u8; 0],
  }

  #[repr(C)]
  #[derive(Copy, Clone, Debug)]
  pub struct skiac_transform {
    pub a: f32,
    pub b: f32,
    pub c: f32,
    pub d: f32,
    pub e: f32,
    pub f: f32,
  }

  #[repr(C)]
  #[derive(Copy, Clone, Debug)]
  pub struct skiac_point {
    pub x: f32,
    pub y: f32,
  }

  #[repr(C)]
  #[derive(Copy, Clone, Debug)]
  pub struct skiac_surface_data {
    pub ptr: *mut u8,
    pub size: u32,
  }

  extern "C" {

    pub fn skiac_surface_create_rgba_premultiplied(width: i32, height: i32) -> *mut skiac_surface;

    pub fn skiac_surface_create_rgba(width: i32, height: i32) -> *mut skiac_surface;

    pub fn skiac_surface_destroy(surface: *mut skiac_surface);

    pub fn skiac_surface_copy_rgba(
      surface: *mut skiac_surface,
      x: u32,
      y: u32,
      width: u32,
      height: u32,
    ) -> *mut skiac_surface;

    pub fn skiac_surface_save(
      c_surface: *mut skiac_surface,
      path: *const ::std::os::raw::c_char,
    ) -> bool;

    pub fn skiac_surface_get_canvas(surface: *mut skiac_surface) -> *mut skiac_canvas;

    pub fn skiac_surface_get_width(surface: *mut skiac_surface) -> i32;

    pub fn skiac_surface_get_height(surface: *mut skiac_surface) -> i32;

    pub fn skiac_surface_read_pixels(surface: *mut skiac_surface, data: *mut skiac_surface_data);

    pub fn skiac_surface_get_alpha_type(surface: *mut skiac_surface) -> i32;

    pub fn skiac_canvas_clear(canvas: *mut skiac_canvas, color: u32);

    pub fn skiac_canvas_flush(canvas: *mut skiac_canvas);

    pub fn skiac_canvas_set_transform(canvas: *mut skiac_canvas, ts: skiac_transform);

    pub fn skiac_canvas_concat(canvas: *mut skiac_canvas, ts: skiac_transform);

    pub fn skiac_canvas_scale(canvas: *mut skiac_canvas, sx: f32, sy: f32);

    pub fn skiac_canvas_translate(canvas: *mut skiac_canvas, dx: f32, dy: f32);

    pub fn skiac_canvas_get_total_transform(canvas: *mut skiac_canvas) -> skiac_transform;

    pub fn skiac_canvas_draw_color(canvas: *mut skiac_canvas, r: f32, g: f32, b: f32, a: f32);

    pub fn skiac_canvas_draw_path(
      canvas: *mut skiac_canvas,
      path: *mut skiac_path,
      paint: *mut skiac_paint,
    );

    pub fn skiac_canvas_draw_rect(
      canvas: *mut skiac_canvas,
      x: f32,
      y: f32,
      w: f32,
      h: f32,
      paint: *mut skiac_paint,
    );

    pub fn skiac_canvas_draw_surface(
      canvas: *mut skiac_canvas,
      surface: *mut skiac_surface,
      left: f32,
      top: f32,
      alpha: u8,
      blend_mode: i32,
      filter_quality: i32,
    );

    pub fn skiac_canvas_draw_surface_rect(
      canvas: *mut skiac_canvas,
      surface: *mut skiac_surface,
      x: f32,
      y: f32,
      w: f32,
      h: f32,
      filter_quality: i32,
    );

    pub fn skiac_canvas_reset_transform(canvas: *mut skiac_canvas);

    pub fn skiac_canvas_clip_rect(canvas: *mut skiac_canvas, x: f32, y: f32, w: f32, h: f32);
    
    pub fn skiac_canvas_clip_path(canvas: *mut skiac_canvas, path: *mut skiac_path);

    pub fn skiac_canvas_save(canvas: *mut skiac_canvas);

    pub fn skiac_canvas_restore(canvas: *mut skiac_canvas);

    pub fn skiac_paint_create() -> *mut skiac_paint;

    pub fn skiac_paint_destroy(paint: *mut skiac_paint);

    pub fn skiac_paint_set_style(paint: *mut skiac_paint, style: i32);

    pub fn skiac_paint_set_color(paint: *mut skiac_paint, r: u8, g: u8, b: u8, a: u8);

    pub fn skiac_paint_set_alpha(paint: *mut skiac_paint, a: u8);
    pub fn skiac_paint_get_alpha(paint: *mut skiac_paint) -> u8;

    pub fn skiac_paint_set_anti_alias(paint: *mut skiac_paint, aa: bool);

    pub fn skiac_paint_set_blend_mode(paint: *mut skiac_paint, blend_mode: i32);
    pub fn skiac_paint_get_blend_mode(paint: *mut skiac_paint) -> i32;

    pub fn skiac_paint_set_shader(paint: *mut skiac_paint, shader: *mut skiac_shader);

    pub fn skiac_paint_set_stroke_width(paint: *mut skiac_paint, width: f32);

    pub fn skiac_paint_set_stroke_cap(paint: *mut skiac_paint, cap: i32);

    pub fn skiac_paint_set_stroke_join(paint: *mut skiac_paint, join: i32);

    pub fn skiac_paint_set_stroke_miter(paint: *mut skiac_paint, miter: f32);
    pub fn skiac_paint_get_stroke_miter(paint: *mut skiac_paint) -> f32;

    pub fn skiac_paint_set_path_effect(
      paint: *mut skiac_paint,
      path_effect: *mut skiac_path_effect,
    );

    pub fn skiac_path_create() -> *mut skiac_path;

    pub fn skiac_path_clone(path: *mut skiac_path) -> *mut skiac_path;

    pub fn skiac_path_op(c_path_one: *mut skiac_path,  c_path_two: *mut skiac_path, op: i32) -> bool;

    pub fn skiac_path_destroy(path: *mut skiac_path);

    pub fn skiac_path_set_fill_type(path: *mut skiac_path, kind: i32);

    pub fn skiac_path_arc_to(
      path: *mut skiac_path,
      left: f32,
      top: f32,
      right: f32,
      bottom: f32,
      start_angle: f32,
      sweep_angle: f32,
      force_move_to: bool,
    );

    pub fn skiac_path_arc_to_tangent(
      path: *mut skiac_path,
      x1: f32,
      y1: f32,
      x2: f32,
      y2: f32,
      radius: f32,
    );

    pub fn skiac_path_move_to(path: *mut skiac_path, x: f32, y: f32);

    pub fn skiac_path_line_to(path: *mut skiac_path, x: f32, y: f32);

    pub fn skiac_path_cubic_to(
      path: *mut skiac_path,
      x1: f32,
      y1: f32,
      x2: f32,
      y2: f32,
      x3: f32,
      y3: f32,
    );

    pub fn skiac_path_close(path: *mut skiac_path);

    pub fn skiac_path_add_rect(path: *mut skiac_path, l: f32, t: f32, r: f32, b: f32);

    pub fn skiac_path_add_circle(path: *mut skiac_path, x: f32, y: f32, r: f32);

    pub fn skiac_path_transform(path: *mut skiac_path, matrix: skiac_transform);

    pub fn skiac_path_transform_matrix(path: *mut skiac_path, matrix: *mut skiac_matrix);

    pub fn skiac_path_is_empty(path: *mut skiac_path) -> bool;

    pub fn skiac_path_effect_make_dash_path(
      intervals: *const f32,
      count: i32,
      phase: f32,
    ) -> *mut skiac_path_effect;

    pub fn skiac_path_effect_destroy(path_effect: *mut skiac_path_effect);

    pub fn skiac_shader_make_linear_gradient(
      points: *const skiac_point,
      colors: *const super::Color,
      positions: *const f32,
      count: i32,
      tile_mode: i32,
      flags: u32,
      ts: skiac_transform,
    ) -> *mut skiac_shader;

    pub fn skiac_shader_make_two_point_conical_gradient(
      start_point: skiac_point,
      start_radius: f32,
      end_point: skiac_point,
      end_radius: f32,
      colors: *const super::Color,
      positions: *const f32,
      count: i32,
      tile_mode: i32,
      flags: u32,
      ts: skiac_transform,
    ) -> *mut skiac_shader;

    pub fn skiac_shader_make_from_surface_image(
      surface: *mut skiac_surface,
      ts: skiac_transform,
      filter_quality: i32,
    ) -> *mut skiac_shader;

    pub fn skiac_shader_destroy(shader: *mut skiac_shader);

    pub fn skiac_matrix_create() -> *mut skiac_matrix;

    pub fn skiac_matrix_pre_translate(matrix: *mut skiac_matrix, dx: f32, dy: f32);

    pub fn skiac_matrix_pre_rotate(matrix: *mut skiac_matrix, degrees: f32);

    pub fn skiac_matrix_invert(matrix: *mut skiac_matrix, inverse: *mut skiac_matrix) -> bool;
  }
}

#[derive(Error, Debug)]
pub enum SkError {
  #[error("[`{0}`] is not valid Blend value")]
  StringToBlendError(String),
  #[error("[`{0}`] is not valid FillRule value")]
  StringToFillRuleError(String),
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum PaintStyle {
  Fill = 0,
  Stroke = 1,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum FillType {
  Winding = 0,
  EvenOdd = 1,
}

impl FromStr for FillType {
  type Err = SkError;

  fn from_str(value: &str) -> Result<Self, SkError> {
    match value {
      "nonzero" => Ok(FillType::Winding),
      "evenodd" => Ok(FillType::EvenOdd),
      _ => Err(SkError::StringToFillRuleError(value.to_owned())),
    }
  }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum StrokeCap {
  Butt = 0,
  Round = 1,
  Square = 2,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum StrokeJoin {
  Miter = 0,
  Round = 1,
  Bevel = 2,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum TileMode {
  Clamp = 0,
  Repeat = 1,
  Mirror = 2,
}

#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum BlendMode {
  /// Replaces destination with zero: fully transparent.
  Clear = 0,
  /// Replaces destination.
  Source,
  /// Preserves destination.
  Destination,
  /// Source over destination.
  SourceOver,
  /// Destination over source.
  DestinationOver,
  /// Source trimmed inside destination.
  SourceIn,
  /// Destination trimmed by source.
  DestinationIn,
  /// Source trimmed outside destination.
  SourceOut,
  /// Destination trimmed outside source.
  DestinationOut,
  /// Source inside destination blended with destination.
  SourceATop,
  /// Destination inside source blended with source.
  DestinationATop,
  /// Each of source and destination trimmed outside the other.
  Xor,
  /// Sum of colors.
  Plus,
  /// Product of premultiplied colors; darkens destination.
  Modulate,
  /// Multiply inverse of pixels, inverting result; brightens destination.
  Screen,
  /// Multiply or screen, depending on destination.
  Overlay,
  /// Darker of source and destination.
  Darken,
  /// Lighter of source and destination.
  Lighten,
  /// Brighten destination to reflect source.
  ColorDodge,
  /// Darken destination to reflect source.
  ColorBurn,
  /// Multiply or screen, depending on source.
  HardLight,
  /// Lighten or darken, depending on source.
  SoftLight,
  /// Subtract darker from lighter with higher contrast.
  Difference,
  /// Subtract darker from lighter with lower contrast.
  Exclusion,
  /// Multiply source with destination, darkening image.
  Multiply,
  /// Hue of source with saturation and luminosity of destination.
  Hue,
  /// Saturation of source with hue and luminosity of destination.
  Saturation,
  /// Hue and saturation of source with luminosity of destination.
  Color,
  /// Luminosity of source with hue and saturation of destination.
  Luminosity,
}

impl BlendMode {
  pub fn as_str(&self) -> &'static str {
    match self {
      BlendMode::Clear => "clear",
      BlendMode::Color => "color",
      BlendMode::ColorBurn => "color-burn",
      BlendMode::ColorDodge => "color-dodge",
      BlendMode::Darken => "darken",
      BlendMode::Destination => "destination",
      BlendMode::DestinationATop => "destination-atop",
      BlendMode::DestinationIn => "destination-in",
      BlendMode::DestinationOut => "destination-out",
      BlendMode::DestinationOver => "destination-over",
      BlendMode::Difference => "difference",
      BlendMode::Exclusion => "exclusion",
      BlendMode::HardLight => "hard-light",
      BlendMode::Hue => "hue",
      BlendMode::Lighten => "lighten",
      BlendMode::Luminosity => "luminosity",
      BlendMode::Modulate => "modulate",
      BlendMode::Multiply => "multiply",
      BlendMode::Overlay => "overlay",
      BlendMode::Plus => "plus",
      BlendMode::Saturation => "saturation",
      BlendMode::Screen => "screen",
      BlendMode::SoftLight => "soft-light",
      BlendMode::Source => "source",
      BlendMode::SourceATop => "source-atop",
      BlendMode::SourceIn => "source-in",
      BlendMode::SourceOut => "source-out",
      BlendMode::SourceOver => "source-over",
      BlendMode::Xor => "xor",
    }
  }
}

impl FromStr for BlendMode {
  type Err = SkError;

  fn from_str(value: &str) -> Result<BlendMode, Self::Err> {
    match value {
      "clear" => Ok(BlendMode::Clear),
      "color" => Ok(BlendMode::Color),
      "color-burn" => Ok(BlendMode::ColorBurn),
      "color-dodge" => Ok(BlendMode::ColorDodge),
      "darken" => Ok(BlendMode::Darken),
      "destination" => Ok(BlendMode::Destination),
      "destination-atop" => Ok(BlendMode::DestinationATop),
      "destination-in" => Ok(BlendMode::DestinationIn),
      "destination-out" => Ok(BlendMode::DestinationOut),
      "destination-over" => Ok(BlendMode::DestinationOver),
      "difference" => Ok(BlendMode::Difference),
      "exclusion" => Ok(BlendMode::Exclusion),
      "hard-light" => Ok(BlendMode::HardLight),
      "hue" => Ok(BlendMode::Hue),
      "lighten" => Ok(BlendMode::Lighten),
      "luminosity" => Ok(BlendMode::Luminosity),
      "modulate" => Ok(BlendMode::Modulate),
      "multiply" => Ok(BlendMode::Multiply),
      "overlay" => Ok(BlendMode::Overlay),
      "plus" => Ok(BlendMode::Plus),
      "saturation" => Ok(BlendMode::Saturation),
      "screen" => Ok(BlendMode::Screen),
      "soft-light" => Ok(BlendMode::SoftLight),
      "source" => Ok(BlendMode::Source),
      "source-atop" => Ok(BlendMode::SourceATop),
      "source-in" => Ok(BlendMode::SourceIn),
      "source-out" => Ok(BlendMode::SourceOut),
      "source-over" => Ok(BlendMode::SourceOver),
      "xor" => Ok(BlendMode::Xor),
      _ => Err(SkError::StringToBlendError(value.to_owned())),
    }
  }
}

impl From<i32> for BlendMode {
  fn from(value: i32) -> BlendMode {
    match value {
      0 => Self::Clear,
      1 => Self::Source,
      2 => Self::Destination,
      3 => Self::SourceOver,
      4 => Self::DestinationOver,
      5 => Self::SourceIn,
      6 => Self::DestinationIn,
      7 => Self::SourceOut,
      8 => Self::DestinationOut,
      9 => Self::SourceATop,
      10 => Self::DestinationATop,
      11 => Self::Xor,
      12 => Self::Plus,
      13 => Self::Modulate,
      14 => Self::Screen,
      15 => Self::Overlay,
      16 => Self::Darken,
      17 => Self::Lighten,
      18 => Self::ColorDodge,
      19 => Self::ColorBurn,
      20 => Self::HardLight,
      21 => Self::SoftLight,
      22 => Self::Difference,
      23 => Self::Exclusion,
      24 => Self::Multiply,
      25 => Self::Hue,
      26 => Self::Saturation,
      27 => Self::Color,
      28 => Self::Luminosity,
      _ => unreachable!(),
    }
  }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum FilterQuality {
  None = 0,
  Low = 1,
  Medium = 2,
  High = 3,
}

/// Describes how to interpret the alpha component of a pixel.
///
/// A pixel may be opaque, or alpha, describing multiple levels of transparency.
///
/// In simple blending, alpha weights the draw color and the destination
/// color to create a new color. If alpha describes a weight from zero to one:
///
/// new color = draw color * alpha + destination color * (1 - alpha)
///
/// In practice alpha is encoded in two or more bits, where 1.0 equals all bits set.
///
/// RGB may have alpha included in each component value; the stored
/// value is the original RGB multiplied by alpha. Premultiplied color
/// components improve performance.
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum AlphaType {
  Unknown,
  Opaque,
  Premultiplied,
  Unpremultiplied,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum PathOp {
  Difference,         // subtract the op path from the first path
  Intersect,          // intersect the two paths
  Union,              // union (inclusive-or) the two paths
  XOR,                // exclusive-or the two paths
  ReverseDifference,  // subtract the first path from the op path
}

pub struct Surface {
  ptr: *mut ffi::skiac_surface,
  pub (crate) canvas: Canvas,
}

impl Surface {
  // TODO: use AlphaType

  #[inline]
  pub fn new_rgba(width: u32, height: u32) -> Option<Surface> {
    unsafe { Self::from_ptr(ffi::skiac_surface_create_rgba(width as i32, height as i32)) }
  }

  #[inline]
  pub fn new_rgba_premultiplied(width: u32, height: u32) -> Option<Surface> {
    unsafe {
      Self::from_ptr(ffi::skiac_surface_create_rgba_premultiplied(
        width as i32,
        height as i32,
      ))
    }
  }

  #[inline]
  unsafe fn from_ptr(ptr: *mut ffi::skiac_surface) -> Option<Surface> {
    if ptr.is_null() {
      None
    } else {
      Some(Surface {
        ptr,
        canvas: Canvas(ffi::skiac_surface_get_canvas(ptr)),
      })
    }
  }

  #[inline]
  pub fn copy_rgba(&self, x: u32, y: u32, width: u32, height: u32) -> Option<Surface> {
    unsafe { Self::from_ptr(ffi::skiac_surface_copy_rgba(self.ptr, x, y, width, height)) }
  }

  #[inline]
  pub fn try_clone(&self) -> Option<Surface> {
    unsafe {
      Self::from_ptr(ffi::skiac_surface_copy_rgba(
        self.ptr,
        0,
        0,
        self.width(),
        self.height(),
      ))
    }
  }

  pub fn save_png(&self, path: &str) -> bool {
    let c_path = std::ffi::CString::new(path).unwrap();
    unsafe { ffi::skiac_surface_save(self.ptr, c_path.as_ptr()) }
  }

  #[inline]
  pub fn width(&self) -> u32 {
    unsafe { ffi::skiac_surface_get_width(self.ptr) as u32 }
  }

  #[inline]
  pub fn height(&self) -> u32 {
    unsafe { ffi::skiac_surface_get_height(self.ptr) as u32 }
  }

  #[inline]
  pub fn alpha_type(&self) -> AlphaType {
    let kind = unsafe { ffi::skiac_surface_get_alpha_type(self.ptr) };
    match kind {
      0 => AlphaType::Unknown,
      1 => AlphaType::Opaque,
      2 => AlphaType::Premultiplied,
      3 => AlphaType::Unpremultiplied,
      _ => unreachable!(),
    }
  }

  #[inline]
  pub fn data_u8(&self) -> &[u8] {
    unsafe {
      let mut data = ffi::skiac_surface_data {
        ptr: std::ptr::null_mut(),
        size: 0,
      };
      ffi::skiac_surface_read_pixels(self.ptr, &mut data);

      slice::from_raw_parts(data.ptr, data.size as usize)
    }
  }

  #[inline]
  pub fn data(&self) -> SurfaceData {
    SurfaceData {
      slice: self.data_u8(),
    }
  }

  #[inline]
  pub fn data_mut(&mut self) -> SurfaceDataMut {
    unsafe {
      let mut data = ffi::skiac_surface_data {
        ptr: std::ptr::null_mut(),
        size: 0,
      };
      ffi::skiac_surface_read_pixels(self.ptr, &mut data);

      SurfaceDataMut {
        slice: slice::from_raw_parts_mut(data.ptr, data.size as usize),
      }
    }
  }
}

impl std::ops::Deref for Surface {
  type Target = Canvas;

  #[inline]
  fn deref(&self) -> &Self::Target {
    &self.canvas
  }
}

impl std::ops::DerefMut for Surface {
  #[inline]
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.canvas
  }
}

impl Drop for Surface {
  #[inline]
  fn drop(&mut self) {
    unsafe {
      ffi::skiac_surface_destroy(self.ptr);
    }
  }
}

pub struct SurfaceData<'a> {
  slice: &'a [u8],
}

impl<'a> Deref for SurfaceData<'a> {
  type Target = [u8];

  #[inline]
  fn deref(&self) -> &[u8] {
    self.slice
  }
}

pub struct SurfaceDataMut<'a> {
  slice: &'a mut [u8],
}

impl<'a> Deref for SurfaceDataMut<'a> {
  type Target = [u8];

  #[inline]
  fn deref(&self) -> &[u8] {
    self.slice
  }
}

impl<'a> DerefMut for SurfaceDataMut<'a> {
  #[inline]
  fn deref_mut(&mut self) -> &mut [u8] {
    self.slice
  }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Color(pub u32);

impl Color {
  #[inline]
  pub fn from_rgba(r: u8, g: u8, b: u8, a: u8) -> Color {
    Color((a as u32) << 24 | (r as u32) << 16 | (g as u32) << 8 | (b as u32))
  }
}

#[repr(transparent)]
pub struct Canvas(*mut ffi::skiac_canvas);

impl Canvas {
  #[inline]
  pub fn clear(&mut self) {
    unsafe {
      ffi::skiac_canvas_clear(self.0, 0);
    }
  }

  #[inline]
  pub fn fill(&mut self, r: u8, g: u8, b: u8, a: u8) {
    unsafe {
      ffi::skiac_canvas_clear(
        self.0,
        (a as u32) << 24 | (r as u32) << 16 | (g as u32) << 8 | b as u32,
      );
    }
  }

  #[inline]
  pub fn flush(&mut self) {
    unsafe {
      ffi::skiac_canvas_flush(self.0);
    }
  }

  #[inline]
  pub fn set_transform(&mut self, ts: Transform) {
    unsafe {
      ffi::skiac_canvas_set_transform(self.0, ts.into());
    }
  }

  #[inline]
  pub fn concat(&mut self, ts: Transform) {
    unsafe {
      ffi::skiac_canvas_concat(self.0, ts.into());
    }
  }

  #[inline]
  pub fn scale(&mut self, sx: f32, sy: f32) {
    unsafe {
      ffi::skiac_canvas_scale(self.0, sx, sy);
    }
  }

  #[inline]
  pub fn translate(&mut self, dx: f32, dy: f32) {
    unsafe {
      ffi::skiac_canvas_translate(self.0, dx, dy);
    }
  }

  #[inline]
  pub fn get_transform(&self) -> Transform {
    unsafe { ffi::skiac_canvas_get_total_transform(self.0).into() }
  }

  #[inline]
  pub fn reset_transform(&mut self) {
    unsafe {
      ffi::skiac_canvas_reset_transform(self.0);
    }
  }

  #[inline]
  pub fn draw_color(&mut self, r: f32, g: f32, b: f32, a: f32) {
    unsafe {
      ffi::skiac_canvas_draw_color(self.0, r, g, b, a);
    }
  }

  #[inline]
  pub fn draw_path(&mut self, path: &Path, paint: &Paint) {
    unsafe {
      ffi::skiac_canvas_draw_path(self.0, path.0, paint.0);
    }
  }

  #[inline]
  pub fn draw_rect(&mut self, x: f32, y: f32, w: f32, h: f32, paint: &Paint) {
    unsafe {
      ffi::skiac_canvas_draw_rect(self.0, x, y, w, h, paint.0);
    }
  }

  #[inline]
  pub fn draw_surface(
    &mut self,
    surface: &Surface,
    left: f32,
    top: f32,
    alpha: u8,
    blend_mode: BlendMode,
    filter_quality: FilterQuality,
  ) {
    unsafe {
      ffi::skiac_canvas_draw_surface(
        self.0,
        surface.ptr,
        left,
        top,
        alpha,
        blend_mode as i32,
        filter_quality as i32,
      );
    }
  }

  #[inline]
  pub fn draw_surface_rect(
    &mut self,
    surface: &Surface,
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    filter_quality: FilterQuality,
  ) {
    unsafe {
      ffi::skiac_canvas_draw_surface_rect(self.0, surface.ptr, x, y, w, h, filter_quality as i32);
    }
  }

  #[inline]
  pub fn set_clip_rect(&mut self, x: f32, y: f32, w: f32, h: f32) {
    unsafe {
      ffi::skiac_canvas_clip_rect(self.0, x, y, w, h);
    }
  }

  #[inline]
  pub fn set_clip_path(&mut self, path: &Path) {
    unsafe {
      ffi::skiac_canvas_clip_path(self.0, path.0);
    }
  }

  #[inline]
  pub fn save(&mut self) {
    unsafe {
      ffi::skiac_canvas_save(self.0);
    }
  }

  #[inline]
  pub fn restore(&mut self) {
    unsafe {
      ffi::skiac_canvas_restore(self.0);
    }
  }
}

pub struct Paint(*mut ffi::skiac_paint);

impl Paint {
  #[inline]
  pub fn new() -> Paint {
    unsafe { Paint(ffi::skiac_paint_create()) }
  }

  #[inline]
  pub fn set_style(&mut self, style: PaintStyle) {
    unsafe {
      ffi::skiac_paint_set_style(self.0, style as i32);
    }
  }

  #[inline]
  pub fn set_color(&mut self, r: u8, g: u8, b: u8, a: u8) {
    unsafe {
      ffi::skiac_paint_set_color(self.0, r, g, b, a);
    }
  }

  #[inline]
  pub fn set_alpha(&mut self, a: u8) {
    unsafe {
      ffi::skiac_paint_set_alpha(self.0, a);
    }
  }

  #[inline]
  pub fn get_alpha(&self) -> u8 {
    unsafe { ffi::skiac_paint_get_alpha(self.0) }
  }

  #[inline]
  pub fn set_anti_alias(&mut self, aa: bool) {
    unsafe {
      ffi::skiac_paint_set_anti_alias(self.0, aa);
    }
  }

  #[inline]
  pub fn set_blend_mode(&mut self, blend_mode: BlendMode) {
    unsafe {
      ffi::skiac_paint_set_blend_mode(self.0, blend_mode as i32);
    }
  }

  #[inline]
  pub fn get_blend_mode(&self) -> BlendMode {
    unsafe { ffi::skiac_paint_get_blend_mode(self.0).into() }
  }

  #[inline]
  pub fn set_shader(&mut self, shader: &Shader) {
    unsafe {
      ffi::skiac_paint_set_shader(self.0, shader.0);
    }
  }

  #[inline]
  pub fn set_stroke_width(&mut self, width: f32) {
    unsafe {
      ffi::skiac_paint_set_stroke_width(self.0, width);
    }
  }

  #[inline]
  pub fn set_stroke_cap(&mut self, cap: StrokeCap) {
    unsafe {
      ffi::skiac_paint_set_stroke_cap(self.0, cap as i32);
    }
  }

  #[inline]
  pub fn set_stroke_join(&mut self, join: StrokeJoin) {
    unsafe {
      ffi::skiac_paint_set_stroke_join(self.0, join as i32);
    }
  }

  #[inline]
  pub fn set_stroke_miter(&mut self, miter: f32) {
    unsafe {
      ffi::skiac_paint_set_stroke_miter(self.0, miter as f32);
    }
  }

  #[inline]
  pub fn get_stroke_miter(&self) -> f32 {
    unsafe { ffi::skiac_paint_get_stroke_miter(self.0) }
  }

  #[inline]
  pub fn set_path_effect(&mut self, path_effect: &PathEffect) {
    unsafe {
      ffi::skiac_paint_set_path_effect(self.0, path_effect.0);
    }
  }
}

impl Default for Paint {
  fn default() -> Self {
    let mut paint = Self::new();
    paint.set_color(0, 0, 0, 0);
    paint.set_stroke_miter(10.0);
    paint
  }
}

impl Drop for Paint {
  #[inline]
  fn drop(&mut self) {
    unsafe { ffi::skiac_paint_destroy(self.0) }
  }
}

#[repr(transparent)]
pub struct Path(*mut ffi::skiac_path);

impl Clone for Path {
  fn clone(&self) -> Path {
    Path(unsafe { ffi::skiac_path_clone(self.0) })
  }
}

impl Path {
  #[inline]
  pub fn new() -> Path {
    unsafe { Path(ffi::skiac_path_create()) }
  }

  #[inline]
  pub fn op(&self, other: &Path, op: PathOp) -> bool {
    unsafe { ffi::skiac_path_op(self.0, other.0, op as i32)  }
  }

  #[inline]
  pub fn set_fill_type(&mut self, kind: FillType) {
    unsafe {
      ffi::skiac_path_set_fill_type(self.0, kind as i32);
    }
  }

  #[inline]
  pub fn arc_to(
    &mut self,
    left: f32,
    top: f32,
    right: f32,
    bottom: f32,
    start_angle: f32,
    sweep_angle: f32,
    force_move_to: bool,
  ) {
    unsafe {
      ffi::skiac_path_arc_to(
        self.0,
        left,
        top,
        right,
        bottom,
        start_angle,
        sweep_angle,
        force_move_to,
      )
    }
  }

  #[inline]
  pub fn arc_to_tangent(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, radius: f32) {
    unsafe { ffi::skiac_path_arc_to_tangent(self.0, x1, y1, x2, y2, radius) }
  }

  #[inline]
  pub fn move_to(&mut self, x: f32, y: f32) {
    unsafe {
      ffi::skiac_path_move_to(self.0, x, y);
    }
  }

  #[inline]
  pub fn line_to(&mut self, x: f32, y: f32) {
    unsafe {
      ffi::skiac_path_line_to(self.0, x, y);
    }
  }

  #[inline]
  pub fn cubic_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x3: f32, y3: f32) {
    unsafe {
      ffi::skiac_path_cubic_to(self.0, x1, y1, x2, y2, x3, y3);
    }
  }

  #[inline]
  pub fn close(&mut self) {
    unsafe {
      ffi::skiac_path_close(self.0);
    }
  }

  #[inline]
  pub fn add_rect(&mut self, l: f32, t: f32, r: f32, b: f32) {
    unsafe {
      ffi::skiac_path_add_rect(self.0, l, t, r, b);
    }
  }

  #[inline]
  pub fn push_circle(&mut self, x: f32, y: f32, r: f32) {
    unsafe {
      ffi::skiac_path_add_circle(self.0, x, y, r);
    }
  }

  #[inline]
  pub fn transform(&mut self, transform: &Transform) {
    unsafe { ffi::skiac_path_transform(self.0, transform.into()) };
  }

  #[inline]
  pub fn transform_matrix(&mut self, matrix: &Matrix) {
    unsafe { ffi::skiac_path_transform_matrix(self.0, matrix.0) };
  }

  #[inline]
  pub fn is_empty(&self) -> bool {
    unsafe { ffi::skiac_path_is_empty(self.0) }
  }
}

impl Drop for Path {
  #[inline]
  fn drop(&mut self) {
    unsafe {
      ffi::skiac_path_destroy(self.0);
    }
  }
}

pub struct Gradient {
  pub colors: Vec<Color>,
  pub positions: Vec<f32>,
  pub tile_mode: TileMode,
  pub transform: Transform,
}

pub struct LinearGradient {
  pub start_point: (f32, f32),
  pub end_point: (f32, f32),
  pub base: Gradient,
}

pub struct TwoPointConicalGradient {
  pub start: (f32, f32),
  pub start_radius: f32,
  pub end: (f32, f32),
  pub end_radius: f32,
  pub base: Gradient,
}

pub struct Shader(*mut ffi::skiac_shader);

impl Shader {
  #[inline]
  pub fn new_linear_gradient(grad: &LinearGradient) -> Option<Shader> {
    let points = [
      ffi::skiac_point {
        x: grad.start_point.0,
        y: grad.start_point.1,
      },
      ffi::skiac_point {
        x: grad.end_point.0,
        y: grad.end_point.1,
      },
    ];

    unsafe {
      Self::from_ptr(ffi::skiac_shader_make_linear_gradient(
        points.as_ptr(),
        grad.base.colors.as_ptr(),
        grad.base.positions.as_ptr(),
        grad.base.colors.len() as i32,
        grad.base.tile_mode as i32,
        0 as u32,
        grad.base.transform.into(),
      ))
    }
  }

  #[inline]
  pub fn new_two_point_conical_gradient(grad: &TwoPointConicalGradient) -> Option<Shader> {
    let start_point = ffi::skiac_point {
      x: grad.start.0,
      y: grad.start.1,
    };
    let end_point = ffi::skiac_point {
      x: grad.end.0,
      y: grad.end.1,
    };

    unsafe {
      Self::from_ptr(ffi::skiac_shader_make_two_point_conical_gradient(
        start_point,
        grad.start_radius,
        end_point,
        grad.end_radius,
        grad.base.colors.as_ptr(),
        grad.base.positions.as_ptr(),
        grad.base.colors.len() as i32,
        grad.base.tile_mode as i32,
        0 as u32,
        grad.base.transform.into(),
      ))
    }
  }

  #[inline]
  pub fn new_from_surface_image(
    surface: &Surface,
    ts: Transform,
    q: FilterQuality,
  ) -> Option<Shader> {
    unsafe {
      Self::from_ptr(ffi::skiac_shader_make_from_surface_image(
        surface.ptr,
        ts.into(),
        q as i32,
      ))
    }
  }

  #[inline]
  unsafe fn from_ptr(ptr: *mut ffi::skiac_shader) -> Option<Shader> {
    if ptr.is_null() {
      None
    } else {
      Some(Shader(ptr))
    }
  }
}

impl Drop for Shader {
  #[inline]
  fn drop(&mut self) {
    unsafe {
      ffi::skiac_shader_destroy(self.0);
    }
  }
}

pub struct PathEffect(*mut ffi::skiac_path_effect);

impl PathEffect {
  #[inline]
  pub fn new_dash_path(intervals: &[f32], phase: f32) -> Option<PathEffect> {
    unsafe {
      let ptr =
        ffi::skiac_path_effect_make_dash_path(intervals.as_ptr(), intervals.len() as i32, phase);

      if ptr.is_null() {
        None
      } else {
        Some(PathEffect(ptr))
      }
    }
  }
}

impl Drop for PathEffect {
  #[inline]
  fn drop(&mut self) {
    unsafe {
      ffi::skiac_path_effect_destroy(self.0);
    }
  }
}

#[repr(transparent)]
pub struct Matrix(*mut ffi::skiac_matrix);

impl Matrix {
  #[inline(always)]
  pub fn identity() -> Self {
    Matrix(unsafe { ffi::skiac_matrix_create() })
  }

  #[inline(always)]
  pub fn pre_translate(&mut self, dx: f32, dy: f32) {
    unsafe { ffi::skiac_matrix_pre_translate(self.0, dx, dy) };
  }

  #[inline(always)]
  pub fn pre_rotate(&mut self, degrees: f32) {
    unsafe { ffi::skiac_matrix_pre_rotate(self.0, degrees) };
  }

  #[must_use]
  #[inline(always)]
  pub fn invert(&self) -> Option<Matrix> {
    let mut m = Matrix::identity();
    if unsafe { ffi::skiac_matrix_invert(self.0, m.0) } {
      Some(m)
    } else {
      None
    }
  }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Transform {
  pub a: f32,
  pub b: f32,
  pub c: f32,
  pub d: f32,
  pub e: f32,
  pub f: f32,
}

impl Transform {
  pub fn new(a: f32, b: f32, c: f32, d: f32, e: f32, f: f32) -> Self {
    Transform { a, b, c, d, e, f }
  }
}

impl Default for Transform {
  fn default() -> Self {
    Transform::new(1.0, 0.0, 0.0, 1.0, 0.0, 0.0)
  }
}

impl From<ffi::skiac_transform> for Transform {
  #[inline]
  fn from(ts: ffi::skiac_transform) -> Self {
    Transform::new(ts.a, ts.b, ts.c, ts.d, ts.e, ts.f)
  }
}

impl From<Transform> for ffi::skiac_transform {
  #[inline]
  fn from(ts: Transform) -> Self {
    ffi::skiac_transform {
      a: ts.a,
      b: ts.b,
      c: ts.c,
      d: ts.d,
      e: ts.e,
      f: ts.f,
    }
  }
}

impl <'a> From<&'a Transform> for ffi::skiac_transform {
  #[inline]
  fn from(ts: &'a Transform) -> Self {
    ffi::skiac_transform {
      a: ts.a,
      b: ts.b,
      c: ts.c,
      d: ts.d,
      e: ts.e,
      f: ts.f,
    }
  }
}
