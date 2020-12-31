use std::convert::TryInto;
use std::mem;
use std::result;
use std::str::FromStr;

use cssparser::{Color as CSSColor, Parser, ParserInput};
use napi::*;

use crate::error::SkError;
use crate::gradient::CanvasGradient;
use crate::pattern::Pattern;
use crate::sk::*;
use crate::state::Context2dRenderingState;

impl From<SkError> for Error {
  fn from(err: SkError) -> Error {
    Error::new(Status::InvalidArg, format!("{}", err))
  }
}

pub struct Context {
  pub(crate) surface: Surface,
  path: Path,
  paint: Paint,
  pub(crate) states: Vec<Context2dRenderingState>,
}

impl Context {
  #[inline(always)]
  pub fn create_js_class(env: &Env) -> Result<JsFunction> {
    env.define_class(
      "CanvasRenderingContext2D",
      context_2d_constructor,
      &vec![
        Property::new(&env, "canvas")?.with_value(env.get_null()?),
        // properties
        Property::new(&env, "miterLimit")?
          .with_getter(get_miter_limit)
          .with_setter(set_miter_limit),
        Property::new(&env, "globalAlpha")?
          .with_getter(get_global_alpha)
          .with_setter(set_global_alpha),
        Property::new(&env, "globalCompositeOperation")?
          .with_getter(get_global_composite_operation)
          .with_setter(set_global_composite_operation),
        Property::new(&env, "lineCap")?
          .with_setter(set_line_cap)
          .with_getter(get_line_cap),
        Property::new(&env, "lineDashOffset")?
          .with_setter(set_line_dash_offset)
          .with_getter(get_line_dash_offset),
        Property::new(&env, "lineWidth")?
          .with_setter(set_line_width)
          .with_getter(get_line_width),
        Property::new(&env, "fillStyle")?
          .with_setter(set_fill_style)
          .with_getter(get_fill_style),
        Property::new(&env, "strokeStyle")?
          .with_setter(set_stroke_style)
          .with_getter(get_stroke_style),
        Property::new(&env, "shadowBlur")?
          .with_setter(set_shadow_blur)
          .with_getter(get_shadow_blur),
        Property::new(&env, "shadowColor")?
          .with_setter(set_shadow_color)
          .with_getter(get_shadow_color),
        Property::new(&env, "shadowOffsetX")?
          .with_setter(set_shadow_offset_x)
          .with_getter(get_shadow_offset_x),
        Property::new(&env, "shadowOffsetY")?
          .with_setter(set_shadow_offset_y)
          .with_getter(get_shadow_offset_y),
        // methods
        Property::new(&env, "arc")?.with_method(arc),
        Property::new(&env, "arcTo")?.with_method(arc_to),
        Property::new(&env, "beginPath")?.with_method(begin_path),
        Property::new(&env, "bezierCurveTo")?.with_method(bezier_curve_to),
        Property::new(&env, "clearRect")?.with_method(clear_rect),
        Property::new(&env, "clip")?.with_method(clip),
        Property::new(&env, "closePath")?.with_method(close_path),
        Property::new(&env, "createLinearGradient")?.with_method(create_linear_gradient),
        Property::new(&env, "createRadialGradient")?.with_method(create_radial_gradient),
        Property::new(&env, "lineTo")?.with_method(line_to),
        Property::new(&env, "moveTo")?.with_method(move_to),
        Property::new(&env, "fill")?.with_method(fill),
        Property::new(&env, "fillRect")?.with_method(fill_rect),
        Property::new(&env, "quadraticCurveTo")?.with_method(quadratic_curve_to),
        Property::new(&env, "rect")?.with_method(rect),
        Property::new(&env, "restore")?.with_method(restore),
        Property::new(&env, "save")?.with_method(save),
        Property::new(&env, "scale")?.with_method(scale),
        Property::new(&env, "stroke")?.with_method(stroke),
        Property::new(&env, "strokeRect")?.with_method(stroke_rect),
        Property::new(&env, "translate")?.with_method(translate),
        // getter setter method
        Property::new(&env, "getTransform")?.with_method(get_current_transform),
        Property::new(&env, "setTransform")?.with_method(set_current_transform),
      ],
    )
  }

  #[inline(always)]
  pub fn new(width: u32, height: u32) -> Result<Self> {
    let surface = Surface::new_rgba(width, height)
      .ok_or_else(|| Error::from_reason("Create skia surface failed".to_owned()))?;
    let mut states = Vec::new();
    states.push(Context2dRenderingState::default());
    Ok(Context {
      surface,
      path: Path::new(),
      paint: Paint::default(),
      states,
    })
  }

  #[inline(always)]
  pub fn clip(&mut self, path: Option<&mut Path>, fill_rule: FillType) {
    let clip = match path {
      Some(path) => path,
      None => &mut self.path,
    };
    clip.set_fill_type(fill_rule);
    self.surface.canvas.set_clip_path(clip);
  }

  #[inline(always)]
  pub fn save(&mut self) {
    self.surface.save();
    self.states.push(self.states.last().unwrap().clone());
  }

  #[inline(always)]
  pub fn restore(&mut self) {
    self.surface.restore();
    if self.states.len() > 1 {
      if let Some(state) = self.states.pop() {
        mem::drop(state);
      };
    }
  }

  #[inline(always)]
  pub fn stroke_rect(&mut self, x: f32, y: f32, w: f32, h: f32) -> result::Result<(), SkError> {
    let stroke_paint = self.stroke_paint()?;
    if let Some(shadow_paint) = self.shadow_paint(&stroke_paint) {
      let surface = &mut self.surface;
      let last_state = self.states.last().unwrap();
      surface.save();
      Self::apply_shadow_offset_matrix(
        surface,
        last_state.shadow_offset_x,
        last_state.shadow_offset_y,
      )?;
      surface.draw_rect(x, y, w, h, &shadow_paint);
      surface.restore();
    };

    self.surface.draw_rect(x, y, w, h, &stroke_paint);

    Ok(())
  }

  #[inline(always)]
  pub fn fill_rect(&mut self, x: f32, y: f32, w: f32, h: f32) -> result::Result<(), SkError> {
    let fill_paint = self.fill_paint()?;
    if let Some(shadow_paint) = self.shadow_paint(&fill_paint) {
      let surface = &mut self.surface;
      let last_state = self.states.last().unwrap();
      surface.save();
      Self::apply_shadow_offset_matrix(
        surface,
        last_state.shadow_offset_x,
        last_state.shadow_offset_y,
      )?;
      surface.draw_rect(x, y, w, h, &shadow_paint);
      surface.restore();
    };

    self.surface.draw_rect(x, y, w, h, &fill_paint);

    Ok(())
  }

  #[inline(always)]
  pub fn stroke(&mut self, path: Option<&Path>) -> Result<()> {
    let p = path.unwrap_or(&self.path);
    let stroke_paint = self.stroke_paint()?;
    if let Some(shadow_paint) = self.shadow_paint(&stroke_paint) {
      let surface = &mut self.surface;
      let last_state = self.states.last().unwrap();
      surface.save();
      Self::apply_shadow_offset_matrix(
        surface,
        last_state.shadow_offset_x,
        last_state.shadow_offset_y,
      )?;
      self.surface.canvas.draw_path(p, &shadow_paint);
      self.surface.restore();
      mem::drop(shadow_paint);
    }
    self.surface.canvas.draw_path(p, &stroke_paint);
    Ok(())
  }

  #[inline(always)]
  pub fn fill(
    &mut self,
    path: Option<&mut Path>,
    fill_rule: FillType,
  ) -> result::Result<(), SkError> {
    let p = if let Some(p) = path {
      p.set_fill_type(fill_rule);
      p
    } else {
      self.path.set_fill_type(fill_rule);
      &self.path
    };
    let fill_paint = self.fill_paint()?;
    if let Some(shadow_paint) = self.shadow_paint(&fill_paint) {
      let surface = &mut self.surface;
      let last_state = self.states.last().unwrap();
      surface.save();
      Self::apply_shadow_offset_matrix(
        surface,
        last_state.shadow_offset_x,
        last_state.shadow_offset_y,
      )?;
      self.surface.canvas.draw_path(p, &shadow_paint);
      self.surface.restore();
      mem::drop(shadow_paint);
    }
    self.surface.draw_path(p, &fill_paint);
    Ok(())
  }

  #[inline(always)]
  fn fill_paint(&self) -> result::Result<Paint, SkError> {
    let mut paint = self.paint.clone();
    paint.set_style(PaintStyle::Fill);
    let last_state = self.states.last().unwrap();
    match &last_state.fill_style {
      Pattern::Color(c, _) => {
        let mut color = c.clone();
        color.alpha =
          ((color.alpha as f32 / 255.0) * (self.paint.get_alpha() as f32 / 255.0)).round() as u8;
        paint.set_color(color.red, color.green, color.blue, color.alpha);
      }
      Pattern::Gradient(g) => {
        let current_transform = self.surface.canvas.get_transform();
        let shader = g.get_shader(&current_transform)?;
        paint.set_color(0, 0, 0, self.paint.get_alpha());
        paint.set_shader(&shader);
      }
      // TODO, image pattern
      Pattern::ImagePattern(p) => {}
    };
    if last_state.line_dash_list.len() != 0 {
      let path_effect = PathEffect::new_dash_path(
        last_state.line_dash_list.as_slice(),
        last_state.line_dash_offset,
      )
      .ok_or_else(|| SkError::Generic(format!("Make line dash path effect failed")))?;
      paint.set_path_effect(&path_effect);
    }
    Ok(paint)
  }

  #[inline(always)]
  fn stroke_paint(&self) -> result::Result<Paint, SkError> {
    let mut paint = self.paint.clone();
    paint.set_style(PaintStyle::Stroke);
    let last_state = self.states.last().unwrap();
    let global_alpha = self.paint.get_alpha();
    match &last_state.stroke_style {
      Pattern::Color(c, _) => {
        let mut color = c.clone();
        color.alpha = ((color.alpha as f32 / 255.0) * (global_alpha as f32 / 255.0)).round() as u8;
        paint.set_color(color.red, color.green, color.blue, color.alpha);
      }
      Pattern::Gradient(g) => {
        let current_transform = self.surface.canvas.get_transform();
        let shader = g.get_shader(&current_transform)?;
        paint.set_color(0, 0, 0, global_alpha);
        paint.set_shader(&shader);
      }
      // TODO, image pattern
      Pattern::ImagePattern(p) => {}
    };
    if !last_state.line_dash_list.is_empty() {
      let path_effect = PathEffect::new_dash_path(
        last_state.line_dash_list.as_slice(),
        last_state.line_dash_offset,
      )
      .ok_or_else(|| SkError::Generic(format!("Make line dash path effect failed")))?;
      paint.set_path_effect(&path_effect);
    }
    Ok(paint)
  }

  #[inline(always)]
  fn shadow_paint(&self, paint: &Paint) -> Option<Paint> {
    let alpha = paint.get_alpha();
    let last_state = self.states.last().unwrap();
    let mut shadow_alpha = last_state.shadow_color.alpha;
    shadow_alpha = shadow_alpha * alpha;
    if shadow_alpha == 0 {
      return None;
    }
    if last_state.shadow_blur == 0f32
      || last_state.shadow_offset_x == 0f32
      || last_state.shadow_offset_y == 0f32
    {
      return None;
    }
    let mut shadow_paint = paint.clone();
    shadow_paint.set_alpha(shadow_alpha);
    let blur_effect = MaskFilter::make_blur(last_state.shadow_blur / 2f32)?;
    shadow_paint.set_mask_filter(&blur_effect);
    Some(shadow_paint)
  }

  #[inline(always)]
  fn apply_shadow_offset_matrix(
    surface: &mut Surface,
    shadow_offset_x: f32,
    shadow_offset_y: f32,
  ) -> result::Result<(), SkError> {
    let current_transform = surface.canvas.get_transform_matrix();
    let invert = current_transform
      .invert()
      .ok_or_else(|| SkError::Generic("Invert matrix failed".to_owned()))?;
    surface.canvas.concat(invert.into_transform());
    let mut shadow_offset = current_transform.clone();
    shadow_offset.pre_translate(shadow_offset_x, shadow_offset_y);
    surface.canvas.concat(shadow_offset.into_transform());
    surface.canvas.concat(current_transform.into_transform());
    Ok(())
  }
}

#[js_function(2)]
fn context_2d_constructor(ctx: CallContext) -> Result<JsUndefined> {
  let width: u32 = ctx.get::<JsNumber>(0)?.try_into()?;
  let height: u32 = ctx.get::<JsNumber>(1)?.try_into()?;
  let mut this = ctx.this_unchecked::<JsObject>();
  let context_2d = Context::new(width, height)?;
  ctx.env.wrap(&mut this, context_2d)?;
  ctx.env.get_undefined()
}

#[js_function(6)]
fn arc(ctx: CallContext) -> Result<JsUndefined> {
  let this = ctx.this_unchecked::<JsObject>();
  let context_2d = ctx.env.unwrap::<Context>(&this)?;
  let center_x: f64 = ctx.get::<JsNumber>(0)?.try_into()?;
  let center_y: f64 = ctx.get::<JsNumber>(1)?.try_into()?;
  let radius: f64 = ctx.get::<JsNumber>(2)?.try_into()?;
  let start_angle: f64 = ctx.get::<JsNumber>(3)?.try_into()?;
  let end_angle: f64 = ctx.get::<JsNumber>(4)?.try_into()?;
  let from_end = if ctx.length == 6 {
    ctx.get::<JsBoolean>(5)?.get_value()?
  } else {
    false
  };
  context_2d.path.arc(
    center_x as f32,
    center_y as f32,
    radius as f32,
    start_angle as f32,
    end_angle as f32,
    from_end,
  );
  ctx.env.get_undefined()
}

#[js_function(5)]
fn arc_to(ctx: CallContext) -> Result<JsUndefined> {
  let this = ctx.this_unchecked::<JsObject>();
  let context_2d = ctx.env.unwrap::<Context>(&this)?;

  let ctrl_x: f64 = ctx.get::<JsNumber>(0)?.try_into()?;
  let ctrl_y: f64 = ctx.get::<JsNumber>(1)?.try_into()?;
  let to_x: f64 = ctx.get::<JsNumber>(2)?.try_into()?;
  let to_y: f64 = ctx.get::<JsNumber>(3)?.try_into()?;
  let radius: f64 = ctx.get::<JsNumber>(4)?.try_into()?;

  context_2d.path.arc_to_tangent(
    ctrl_x as f32,
    ctrl_y as f32,
    to_x as f32,
    to_y as f32,
    radius as f32,
  );
  ctx.env.get_undefined()
}

#[js_function]
fn begin_path(ctx: CallContext) -> Result<JsUndefined> {
  let this = ctx.this_unchecked::<JsObject>();
  let context_2d = ctx.env.unwrap::<Context>(&this)?;

  let new_sub_path = Path::new();
  mem::drop(mem::replace(&mut context_2d.path, new_sub_path));

  ctx.env.get_undefined()
}

#[js_function(6)]
fn bezier_curve_to(ctx: CallContext) -> Result<JsUndefined> {
  let this = ctx.this_unchecked::<JsObject>();
  let context_2d = ctx.env.unwrap::<Context>(&this)?;

  let cp1x: f64 = ctx.get::<JsNumber>(0)?.try_into()?;
  let cp1y: f64 = ctx.get::<JsNumber>(1)?.try_into()?;
  let cp2x: f64 = ctx.get::<JsNumber>(2)?.try_into()?;
  let cp2y: f64 = ctx.get::<JsNumber>(3)?.try_into()?;
  let x: f64 = ctx.get::<JsNumber>(4)?.try_into()?;
  let y: f64 = ctx.get::<JsNumber>(5)?.try_into()?;

  context_2d.path.cubic_to(
    cp1x as f32,
    cp1y as f32,
    cp2x as f32,
    cp2y as f32,
    x as f32,
    y as f32,
  );

  ctx.env.get_undefined()
}

#[js_function(4)]
fn quadratic_curve_to(ctx: CallContext) -> Result<JsUndefined> {
  let this = ctx.this_unchecked::<JsObject>();
  let context_2d = ctx.env.unwrap::<Context>(&this)?;

  let cpx: f64 = ctx.get::<JsNumber>(0)?.try_into()?;
  let cpy: f64 = ctx.get::<JsNumber>(1)?.try_into()?;
  let x: f64 = ctx.get::<JsNumber>(2)?.try_into()?;
  let y: f64 = ctx.get::<JsNumber>(3)?.try_into()?;

  context_2d
    .path
    .quad_to(cpx as f32, cpy as f32, x as f32, y as f32);

  ctx.env.get_undefined()
}

#[js_function(2)]
fn clip(ctx: CallContext) -> Result<JsUndefined> {
  let this = ctx.this_unchecked::<JsObject>();
  let context_2d = ctx.env.unwrap::<Context>(&this)?;

  if ctx.length == 0 {
    context_2d.clip(None, FillType::Winding);
  } else if ctx.length == 1 {
    let rule = ctx.get::<JsString>(0)?;
    context_2d.clip(None, FillType::from_str(rule.into_utf8()?.as_str()?)?);
  } else {
    let path = ctx.get::<JsObject>(0)?;
    let rule = ctx.get::<JsString>(1)?;
    context_2d.clip(
      Some(ctx.env.unwrap::<Path>(&path)?),
      FillType::from_str(rule.into_utf8()?.as_str()?)?,
    );
  };

  ctx.env.get_undefined()
}

#[js_function(4)]
fn rect(ctx: CallContext) -> Result<JsUndefined> {
  let this = ctx.this_unchecked::<JsObject>();
  let context_2d = ctx.env.unwrap::<Context>(&this)?;

  let x: f64 = ctx.get::<JsNumber>(0)?.try_into()?;
  let y: f64 = ctx.get::<JsNumber>(1)?.try_into()?;
  let width: f64 = ctx.get::<JsNumber>(2)?.try_into()?;
  let height: f64 = ctx.get::<JsNumber>(3)?.try_into()?;

  context_2d
    .path
    .add_rect(x as f32, y as f32, width as f32, height as f32);
  ctx.env.get_undefined()
}

#[js_function(2)]
fn fill(ctx: CallContext) -> Result<JsUndefined> {
  let this = ctx.this_unchecked::<JsObject>();
  let context_2d = ctx.env.unwrap::<Context>(&this)?;

  if ctx.length == 0 {
    context_2d.fill(None, FillType::Winding)?;
  } else if ctx.length == 1 {
    let input = ctx.get::<JsUnknown>(0)?;
    match input.get_type()? {
      ValueType::String => {
        let fill_rule_js = unsafe { input.cast::<JsString>() }.into_utf8()?;
        context_2d.fill(None, FillType::from_str(fill_rule_js.as_str()?)?)?;
      }
      ValueType::Object => {
        let path_js = ctx.get::<JsObject>(0)?;
        let path = ctx.env.unwrap::<Path>(&path_js)?;
        context_2d.fill(Some(path), FillType::Winding)?;
      }
      _ => {
        return Err(Error::new(
          Status::InvalidArg,
          format!("Invalid fill argument"),
        ))
      }
    }
  } else {
    let path_js = ctx.get::<JsObject>(0)?;
    let fill_rule_js = ctx.get::<JsString>(1)?.into_utf8()?;
    let path = ctx.env.unwrap::<Path>(&path_js)?;
    context_2d.fill(Some(path), FillType::from_str(fill_rule_js.as_str()?)?)?;
  };

  ctx.env.get_undefined()
}

#[js_function]
fn save(ctx: CallContext) -> Result<JsUndefined> {
  let this = ctx.this_unchecked::<JsObject>();
  let context_2d = ctx.env.unwrap::<Context>(&this)?;
  context_2d.save();
  ctx.env.get_undefined()
}

#[js_function]
fn restore(ctx: CallContext) -> Result<JsUndefined> {
  let this = ctx.this_unchecked::<JsObject>();
  let context_2d = ctx.env.unwrap::<Context>(&this)?;
  context_2d.restore();
  ctx.env.get_undefined()
}

#[js_function(4)]
fn clear_rect(ctx: CallContext) -> Result<JsUndefined> {
  let this = ctx.this_unchecked::<JsObject>();
  let context_2d = ctx.env.unwrap::<Context>(&this)?;
  let x: f64 = ctx.get::<JsNumber>(0)?.try_into()?;
  let y: f64 = ctx.get::<JsNumber>(1)?.try_into()?;
  let width: f64 = ctx.get::<JsNumber>(2)?.try_into()?;
  let height: f64 = ctx.get::<JsNumber>(3)?.try_into()?;

  let mut paint = Paint::new();
  paint.set_style(PaintStyle::Fill);
  paint.set_color(0, 0, 0, 0);
  paint.set_stroke_miter(10.0);
  paint.set_blend_mode(BlendMode::SourceOver);
  context_2d
    .surface
    .draw_rect(x as f32, y as f32, width as f32, height as f32, &paint);
  ctx.env.get_undefined()
}

#[js_function(4)]
fn create_linear_gradient(ctx: CallContext) -> Result<JsObject> {
  let x0: f64 = ctx.get::<JsNumber>(0)?.try_into()?;
  let y0: f64 = ctx.get::<JsNumber>(1)?.try_into()?;
  let x1: f64 = ctx.get::<JsNumber>(2)?.try_into()?;
  let y1: f64 = ctx.get::<JsNumber>(3)?.try_into()?;
  let linear_gradient =
    CanvasGradient::create_linear_gradient(x0 as f32, y0 as f32, x1 as f32, y1 as f32);
  linear_gradient.into_js_instance(ctx.env)
}

#[js_function(6)]
fn create_radial_gradient(ctx: CallContext) -> Result<JsObject> {
  let x0: f64 = ctx.get::<JsNumber>(0)?.try_into()?;
  let y0: f64 = ctx.get::<JsNumber>(1)?.try_into()?;
  let r0: f64 = ctx.get::<JsNumber>(2)?.try_into()?;
  let x1: f64 = ctx.get::<JsNumber>(3)?.try_into()?;
  let y1: f64 = ctx.get::<JsNumber>(4)?.try_into()?;
  let r1: f64 = ctx.get::<JsNumber>(5)?.try_into()?;
  let radial_gradient = CanvasGradient::create_radial_gradient(
    x0 as f32, y0 as f32, r0 as f32, x1 as f32, y1 as f32, r1 as f32,
  );
  radial_gradient.into_js_instance(ctx.env)
}

#[js_function]
fn close_path(ctx: CallContext) -> Result<JsUndefined> {
  let this = ctx.this_unchecked::<JsObject>();
  let context_2d = ctx.env.unwrap::<Context>(&this)?;

  context_2d.path.close();
  ctx.env.get_undefined()
}

#[js_function(2)]
fn line_to(ctx: CallContext) -> Result<JsUndefined> {
  let this = ctx.this_unchecked::<JsObject>();
  let context_2d = ctx.env.unwrap::<Context>(&this)?;

  let x: f64 = ctx.get::<JsNumber>(0)?.try_into()?;
  let y: f64 = ctx.get::<JsNumber>(1)?.try_into()?;

  context_2d.path.line_to(x as f32, y as f32);

  ctx.env.get_undefined()
}

#[js_function(2)]
fn move_to(ctx: CallContext) -> Result<JsUndefined> {
  let this = ctx.this_unchecked::<JsObject>();
  let context_2d = ctx.env.unwrap::<Context>(&this)?;

  let x: f64 = ctx.get::<JsNumber>(0)?.try_into()?;
  let y: f64 = ctx.get::<JsNumber>(1)?.try_into()?;

  context_2d.path.move_to(x as f32, y as f32);

  ctx.env.get_undefined()
}

#[js_function(1)]
fn set_miter_limit(ctx: CallContext) -> Result<JsUndefined> {
  let this = ctx.this_unchecked::<JsObject>();
  let context_2d = ctx.env.unwrap::<Context>(&this)?;
  let miter: f64 = ctx.get::<JsNumber>(0)?.try_into()?;
  context_2d.paint.set_stroke_miter(miter as f32);
  ctx.env.get_undefined()
}

#[js_function]
fn get_miter_limit(ctx: CallContext) -> Result<JsNumber> {
  let this = ctx.this_unchecked::<JsObject>();
  let context_2d = ctx.env.unwrap::<Context>(&this)?;

  ctx
    .env
    .create_double(context_2d.paint.get_stroke_miter() as f64)
}

#[js_function(4)]
fn stroke_rect(ctx: CallContext) -> Result<JsUndefined> {
  let x: f64 = ctx.get::<JsNumber>(0)?.try_into()?;
  let y: f64 = ctx.get::<JsNumber>(1)?.try_into()?;
  let w: f64 = ctx.get::<JsNumber>(2)?.try_into()?;
  let h: f64 = ctx.get::<JsNumber>(3)?.try_into()?;

  let this = ctx.this_unchecked::<JsObject>();
  let context_2d = ctx.env.unwrap::<Context>(&this)?;

  context_2d.stroke_rect(x as f32, y as f32, w as f32, h as f32)?;

  ctx.env.get_undefined()
}

#[js_function(4)]
fn fill_rect(ctx: CallContext) -> Result<JsUndefined> {
  let x: f64 = ctx.get::<JsNumber>(0)?.try_into()?;
  let y: f64 = ctx.get::<JsNumber>(1)?.try_into()?;
  let w: f64 = ctx.get::<JsNumber>(2)?.try_into()?;
  let h: f64 = ctx.get::<JsNumber>(3)?.try_into()?;

  let this = ctx.this_unchecked::<JsObject>();
  let context_2d = ctx.env.unwrap::<Context>(&this)?;

  context_2d.fill_rect(x as f32, y as f32, w as f32, h as f32)?;

  ctx.env.get_undefined()
}

#[js_function(1)]
fn set_global_alpha(ctx: CallContext) -> Result<JsUndefined> {
  let this = ctx.this_unchecked::<JsObject>();
  let context_2d = ctx.env.unwrap::<Context>(&this)?;
  let alpha: f64 = ctx.get::<JsNumber>(0)?.try_into()?;

  if alpha < 0.0 || alpha > 1.0 {
    return Err(Error::new(
      Status::InvalidArg,
      format!(
        "Alpha value out of range, expected 0.0 - 1.0, but got : {}",
        alpha
      ),
    ));
  }

  context_2d.paint.set_alpha((alpha * 255.0) as u8);
  ctx.env.get_undefined()
}

#[js_function]
fn get_global_alpha(ctx: CallContext) -> Result<JsNumber> {
  let this = ctx.this_unchecked::<JsObject>();
  let context_2d = ctx.env.unwrap::<Context>(&this)?;

  ctx
    .env
    .create_double((context_2d.paint.get_alpha() as f64) / 255.0)
}

#[js_function(1)]
fn set_global_composite_operation(ctx: CallContext) -> Result<JsUndefined> {
  let this = ctx.this_unchecked::<JsObject>();
  let context_2d = ctx.env.unwrap::<Context>(&this)?;
  let blend_string = ctx.get::<JsString>(0)?.into_utf8()?;

  context_2d
    .paint
    .set_blend_mode(BlendMode::from_str(blend_string.as_str()?).map_err(Error::from)?);

  ctx.env.get_undefined()
}

#[js_function]
fn get_global_composite_operation(ctx: CallContext) -> Result<JsString> {
  let this = ctx.this_unchecked::<JsObject>();
  let context_2d = ctx.env.unwrap::<Context>(&this)?;

  ctx
    .env
    .create_string(context_2d.paint.get_blend_mode().as_str())
}

#[js_function]
fn get_current_transform(ctx: CallContext) -> Result<JsObject> {
  let mut transform_object = ctx.env.create_object()?;
  let this = ctx.this_unchecked::<JsObject>();
  let context_2d = ctx.env.unwrap::<Context>(&this)?;

  let current_transform = context_2d.surface.canvas.get_transform();

  transform_object.set_named_property("a", ctx.env.create_double(current_transform.a as f64)?)?;
  transform_object.set_named_property("b", ctx.env.create_double(current_transform.b as f64)?)?;
  transform_object.set_named_property("c", ctx.env.create_double(current_transform.c as f64)?)?;
  transform_object.set_named_property("d", ctx.env.create_double(current_transform.d as f64)?)?;
  transform_object.set_named_property("e", ctx.env.create_double(current_transform.e as f64)?)?;
  transform_object.set_named_property("f", ctx.env.create_double(current_transform.f as f64)?)?;
  Ok(transform_object)
}

#[js_function(6)]
fn set_current_transform(ctx: CallContext) -> Result<JsUndefined> {
  let this = ctx.this_unchecked::<JsObject>();
  let context_2d = ctx.env.unwrap::<Context>(&this)?;

  let transform = if ctx.length == 1 {
    let transform_object = ctx.get::<JsObject>(0)?;
    let a: f64 = transform_object
      .get_named_property::<JsNumber>("a")?
      .try_into()?;
    let b: f64 = transform_object
      .get_named_property::<JsNumber>("b")?
      .try_into()?;
    let c: f64 = transform_object
      .get_named_property::<JsNumber>("c")?
      .try_into()?;
    let d: f64 = transform_object
      .get_named_property::<JsNumber>("d")?
      .try_into()?;
    let e: f64 = transform_object
      .get_named_property::<JsNumber>("e")?
      .try_into()?;
    let f: f64 = transform_object
      .get_named_property::<JsNumber>("f")?
      .try_into()?;
    Transform::new(a as f32, b as f32, c as f32, d as f32, e as f32, f as f32)
  } else if ctx.length == 6 {
    let a: f64 = ctx.get::<JsNumber>(0)?.try_into()?;
    let b: f64 = ctx.get::<JsNumber>(1)?.try_into()?;
    let c: f64 = ctx.get::<JsNumber>(2)?.try_into()?;
    let d: f64 = ctx.get::<JsNumber>(3)?.try_into()?;
    let e: f64 = ctx.get::<JsNumber>(4)?.try_into()?;
    let f: f64 = ctx.get::<JsNumber>(5)?.try_into()?;
    Transform::new(a as f32, b as f32, c as f32, d as f32, e as f32, f as f32)
  } else {
    return Err(Error::new(
      Status::InvalidArg,
      format!("Invalid argument length in setTransform"),
    ));
  };

  context_2d.surface.canvas.set_transform(transform);

  ctx.env.get_undefined()
}

#[js_function(2)]
fn scale(ctx: CallContext) -> Result<JsUndefined> {
  let x: f64 = ctx.get::<JsNumber>(0)?.try_into()?;
  let y: f64 = ctx.get::<JsNumber>(1)?.try_into()?;

  let this = ctx.this_unchecked::<JsObject>();
  let context_2d = ctx.env.unwrap::<Context>(&this)?;

  context_2d.surface.canvas.scale(x as f32, y as f32);

  ctx.env.get_undefined()
}

#[js_function(1)]
fn stroke(ctx: CallContext) -> Result<JsUndefined> {
  let this = ctx.this_unchecked::<JsObject>();
  let context_2d = ctx.env.unwrap::<Context>(&this)?;

  let path = if ctx.length == 0 {
    None
  } else {
    let js_path = ctx.get::<JsObject>(0)?;
    let path = ctx.env.unwrap::<Path>(&js_path)?;
    Some(path)
  };

  context_2d.stroke(path.as_deref())?;

  ctx.env.get_undefined()
}

#[js_function(2)]
fn translate(ctx: CallContext) -> Result<JsUndefined> {
  let x: f64 = ctx.get::<JsNumber>(0)?.try_into()?;
  let y: f64 = ctx.get::<JsNumber>(1)?.try_into()?;

  let this = ctx.this_unchecked::<JsObject>();
  let context_2d = ctx.env.unwrap::<Context>(&this)?;

  let mut inverted = Matrix::identity();
  inverted.pre_translate(-x as f32, -y as f32);
  context_2d.path.transform_matrix(&inverted);
  context_2d.surface.canvas.translate(x as f32, y as f32);

  ctx.env.get_undefined()
}

#[js_function]
fn get_line_cap(ctx: CallContext) -> Result<JsString> {
  let this = ctx.this_unchecked::<JsObject>();
  let context_2d = ctx.env.unwrap::<Context>(&this)?;

  ctx
    .env
    .create_string(context_2d.paint.get_stroke_cap().as_str())
}

#[js_function(1)]
fn set_line_cap(ctx: CallContext) -> Result<JsUndefined> {
  let line_cap_string = ctx.get::<JsString>(0)?;
  let line_cap = line_cap_string.into_utf8()?;

  let this = ctx.this_unchecked::<JsObject>();
  let context_2d = ctx.env.unwrap::<Context>(&this)?;

  context_2d
    .paint
    .set_stroke_cap(StrokeCap::from_str(line_cap.as_str()?)?);

  ctx.env.get_undefined()
}

#[js_function]
fn get_line_dash_offset(ctx: CallContext) -> Result<JsNumber> {
  let this = ctx.this_unchecked::<JsObject>();
  let context_2d = ctx.env.unwrap::<Context>(&this)?;

  ctx
    .env
    .create_double(context_2d.states.last_mut().unwrap().line_dash_offset as f64)
}

#[js_function(1)]
fn set_line_dash_offset(ctx: CallContext) -> Result<JsUndefined> {
  let line_offset_number = ctx.get::<JsNumber>(0)?;
  let offset: f64 = line_offset_number.get_double()?;

  let this = ctx.this_unchecked::<JsObject>();
  let context_2d = ctx.env.unwrap::<Context>(&this)?;

  context_2d.states.last_mut().unwrap().line_dash_offset = offset as f32;

  ctx.env.get_undefined()
}

#[js_function]
fn get_line_join(ctx: CallContext) -> Result<JsString> {
  let this = ctx.this_unchecked::<JsObject>();
  let context_2d = ctx.env.unwrap::<Context>(&this)?;

  ctx
    .env
    .create_string(context_2d.paint.get_stroke_join().as_str())
}

#[js_function(1)]
fn set_line_join(ctx: CallContext) -> Result<JsUndefined> {
  let line_join_string = ctx.get::<JsString>(0)?;
  let line_join = line_join_string.into_utf8()?;

  let this = ctx.this_unchecked::<JsObject>();
  let context_2d = ctx.env.unwrap::<Context>(&this)?;

  context_2d
    .paint
    .set_stroke_join(StrokeJoin::from_str(line_join.as_str()?)?);

  ctx.env.get_undefined()
}

#[js_function]
fn get_line_width(ctx: CallContext) -> Result<JsNumber> {
  let this = ctx.this_unchecked::<JsObject>();
  let context_2d = ctx.env.unwrap::<Context>(&this)?;

  ctx
    .env
    .create_double(context_2d.paint.get_stroke_width() as f64)
}

#[js_function(1)]
fn set_line_width(ctx: CallContext) -> Result<JsUndefined> {
  let width: f64 = ctx.get::<JsNumber>(0)?.try_into()?;

  let this = ctx.this_unchecked::<JsObject>();
  let context_2d = ctx.env.unwrap::<Context>(&this)?;

  context_2d.paint.set_stroke_width(width as f32);

  ctx.env.get_undefined()
}

#[js_function(1)]
fn set_fill_style(ctx: CallContext) -> Result<JsUndefined> {
  let mut this = ctx.this_unchecked::<JsObject>();
  let context_2d = ctx.env.unwrap::<Context>(&this)?;

  let js_fill_style = ctx.get::<JsUnknown>(0)?;

  let last_state = context_2d.states.last_mut().unwrap();

  match js_fill_style.get_type()? {
    ValueType::String => {
      let js_color = unsafe { js_fill_style.cast::<JsString>() }.into_utf8()?;
      last_state.fill_style = Pattern::from_color(js_color.as_str()?)?;
    }
    ValueType::Object => {
      let fill_object = unsafe { js_fill_style.cast::<JsObject>() };
      let gradient = ctx.env.unwrap::<CanvasGradient>(&fill_object)?;
      last_state.fill_style = Pattern::Gradient(gradient.clone());
    }
    // todo ImagePattern
    _ => return Err(Error::new(Status::InvalidArg, format!("Invalid fillStyle"))),
  }

  this.set_named_property("_fillStyle", js_fill_style)?;

  ctx.env.get_undefined()
}

#[js_function]
fn get_fill_style(ctx: CallContext) -> Result<JsUnknown> {
  let this = ctx.this_unchecked::<JsObject>();
  this.get_named_property("_fillStyle")
}

#[js_function(1)]
fn set_stroke_style(ctx: CallContext) -> Result<JsUndefined> {
  let mut this = ctx.this_unchecked::<JsObject>();
  let context_2d = ctx.env.unwrap::<Context>(&this)?;

  let js_stroke_style = ctx.get::<JsUnknown>(0)?;
  let last_state = context_2d.states.last_mut().unwrap();

  match js_stroke_style.get_type()? {
    ValueType::String => {
      let js_color = unsafe { JsString::from_raw_unchecked(ctx.env.raw(), js_stroke_style.raw()) }
        .into_utf8()?;
      last_state.stroke_style = Pattern::from_color(js_color.as_str()?)?;
    }
    ValueType::Object => {
      let stroke_object = unsafe { js_stroke_style.cast::<JsObject>() };
      let gradient = ctx.env.unwrap::<CanvasGradient>(&stroke_object)?;
      last_state.stroke_style = Pattern::Gradient(gradient.clone());
    }
    // todo ImagePattern
    ValueType::External => {}
    _ => {
      return Err(Error::new(
        Status::InvalidArg,
        format!("Invalid strokeStyle"),
      ))
    }
  }

  this.set_named_property("_strokeStyle", js_stroke_style)?;

  ctx.env.get_undefined()
}

#[js_function]
fn get_stroke_style(ctx: CallContext) -> Result<JsUnknown> {
  let this = ctx.this_unchecked::<JsObject>();
  this.get_named_property("_strokeStyle")
}

#[js_function]
fn get_shadow_blur(ctx: CallContext) -> Result<JsNumber> {
  let this = ctx.this_unchecked::<JsObject>();
  let context_2d = ctx.env.unwrap::<Context>(&this)?;
  let last_state = context_2d.states.last_mut().unwrap();

  ctx.env.create_double(last_state.shadow_blur as f64)
}

#[js_function(1)]
fn set_shadow_blur(ctx: CallContext) -> Result<JsUndefined> {
  let blur: f64 = ctx.get::<JsNumber>(0)?.try_into()?;

  let this = ctx.this_unchecked::<JsObject>();
  let context_2d = ctx.env.unwrap::<Context>(&this)?;
  let last_state = context_2d.states.last_mut().unwrap();

  last_state.shadow_blur = blur as f32;

  ctx.env.get_undefined()
}

#[js_function]
fn get_shadow_color(ctx: CallContext) -> Result<JsString> {
  let this = ctx.this_unchecked::<JsObject>();
  let context_2d = ctx.env.unwrap::<Context>(&this)?;

  ctx.env.create_string(
    context_2d
      .states
      .last()
      .unwrap()
      .shadow_color_string
      .as_str(),
  )
}

#[js_function(1)]
fn set_shadow_color(ctx: CallContext) -> Result<JsUndefined> {
  let shadow_color_string = ctx.get::<JsString>(0)?;
  let shadow_color = shadow_color_string.into_utf8()?;

  let this = ctx.this_unchecked::<JsObject>();
  let context_2d = ctx.env.unwrap::<Context>(&this)?;

  let last_state = context_2d.states.last_mut().unwrap();
  let shadow_color_str = shadow_color.as_str()?;
  last_state.shadow_color_string = shadow_color_str.to_owned();

  let mut parser_input = ParserInput::new(shadow_color_str);
  let mut parser = Parser::new(&mut parser_input);
  let color =
    CSSColor::parse(&mut parser).map_err(|e| SkError::Generic(format!("Invalid color {:?}", e)))?;

  match color {
    CSSColor::CurrentColor => {
      return Err(Error::new(
        Status::InvalidArg,
        "Color should not be `currentcolor` keyword".to_owned(),
      ))
    }
    CSSColor::RGBA(rgba) => {
      last_state.shadow_color = rgba;
    }
  }

  ctx.env.get_undefined()
}

#[js_function]
fn get_shadow_offset_x(ctx: CallContext) -> Result<JsNumber> {
  let this = ctx.this_unchecked::<JsObject>();
  let context_2d = ctx.env.unwrap::<Context>(&this)?;
  let last_state = context_2d.states.last_mut().unwrap();

  ctx.env.create_double(last_state.shadow_offset_x as f64)
}

#[js_function(1)]
fn set_shadow_offset_x(ctx: CallContext) -> Result<JsUndefined> {
  let offset: f64 = ctx.get::<JsNumber>(0)?.try_into()?;

  let this = ctx.this_unchecked::<JsObject>();
  let context_2d = ctx.env.unwrap::<Context>(&this)?;
  let last_state = context_2d.states.last_mut().unwrap();

  last_state.shadow_offset_x = offset as f32;

  ctx.env.get_undefined()
}

#[js_function]
fn get_shadow_offset_y(ctx: CallContext) -> Result<JsNumber> {
  let this = ctx.this_unchecked::<JsObject>();
  let context_2d = ctx.env.unwrap::<Context>(&this)?;
  let last_state = context_2d.states.last_mut().unwrap();

  ctx.env.create_double(last_state.shadow_offset_y as f64)
}

#[js_function(1)]
fn set_shadow_offset_y(ctx: CallContext) -> Result<JsUndefined> {
  let offset: f64 = ctx.get::<JsNumber>(0)?.try_into()?;

  let this = ctx.this_unchecked::<JsObject>();
  let context_2d = ctx.env.unwrap::<Context>(&this)?;
  let last_state = context_2d.states.last_mut().unwrap();

  last_state.shadow_offset_y = offset as f32;

  ctx.env.get_undefined()
}

pub enum ContextData {
  PNG(SurfaceRef),
  JPEG(SurfaceRef, u8),
}

unsafe impl Send for ContextData {}
unsafe impl Sync for ContextData {}

impl Task for ContextData {
  type Output = SurfaceDataRef;
  type JsValue = JsBuffer;

  fn compute(&mut self) -> Result<Self::Output> {
    match self {
      ContextData::PNG(surface) => surface.png_data().ok_or_else(|| {
        Error::new(
          Status::GenericFailure,
          format!("Get png data from surface failed"),
        )
      }),
      _ => {
        todo!();
      }
    }
  }

  fn resolve(self, env: Env, output: Self::Output) -> Result<Self::JsValue> {
    unsafe {
      env
        .create_buffer_with_borrowed_data(
          output.0.ptr,
          output.0.size,
          output,
          |data_ref: Self::Output, _| data_ref.unref(),
        )
        .map(|value| value.into_raw())
    }
  }
}
