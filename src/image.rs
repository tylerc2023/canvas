use std::mem::ManuallyDrop;
use std::slice;

use napi::*;

use crate::sk::Bitmap;

#[derive(Debug, Clone)]
pub struct ImageData {
  pub(crate) width: u32,
  pub(crate) height: u32,
  pub(crate) data: *mut u8,
}

impl Drop for ImageData {
  fn drop(&mut self) {
    let len = (self.width * self.height * 4) as usize;
    unsafe { Vec::from_raw_parts(self.data, len, len) };
  }
}

impl ImageData {
  pub fn create_js_class(env: &Env) -> Result<JsFunction> {
    env.define_class("ImageData", image_data_constructor, &[])
  }
}

#[js_function(3)]
fn image_data_constructor(ctx: CallContext) -> Result<JsUndefined> {
  let first_arg = ctx.get::<JsUnknown>(0)?;
  let first_arg_type = first_arg.get_type()?;
  let ((js_width, width), (js_height, height), arraybuffer_length, mut initial_data) =
    match first_arg_type {
      ValueType::Number => {
        let js_width = unsafe { first_arg.cast::<JsNumber>() };
        let js_height = ctx.get::<JsNumber>(1)?;
        let width = js_width.get_uint32()?;
        let height = js_height.get_uint32()?;
        let arraybuffer_length = (width * height * 4) as usize;
        Ok((
          (js_width, width),
          (js_height, height),
          arraybuffer_length,
          ManuallyDrop::new(vec![0u8; arraybuffer_length]),
        ))
      }
      ValueType::Object => {
        let image_data_ab = unsafe { first_arg.cast::<JsTypedArray>() }.into_value()?;
        if image_data_ab.typedarray_type != TypedArrayType::Uint8Clamped {
          return Err(Error::new(
            Status::InvalidArg,
            "ImageData constructor: Argument 1 does not implement interface Uint8ClampedArray."
              .to_owned(),
          ));
        }
        let arraybuffer_length = image_data_ab.len();
        let js_width = ctx.get::<JsNumber>(1)?;
        let width = js_width.get_uint32()?;
        let (js_height, height) = if ctx.length == 3 {
          let js_height = ctx.get::<JsNumber>(2)?;
          let height = js_height.get_uint32()?;
          if height * width * 4 != arraybuffer_length as u32 {
            return Err(Error::new(
              Status::InvalidArg,
              "Index or size is negative or greater than the allowed amount".to_owned(),
            ));
          }
          (js_height, height)
        } else {
          let height = arraybuffer_length as u32 / width / 4u32;
          (ctx.env.create_uint32(height)?, height)
        };
        Ok((
          (js_width, width),
          (js_height, height),
          arraybuffer_length,
          ManuallyDrop::new(unsafe {
            slice::from_raw_parts(image_data_ab.as_ptr() as *const u8, arraybuffer_length)
              .to_owned()
          }),
        ))
      }
      _ => Err(Error::new(
        Status::InvalidArg,
        format!(
          "Invalid type of first argument of ImageData constructor [{:?}]",
          first_arg_type
        ),
      )),
    }?;
  let data_ptr = initial_data.as_mut_ptr();
  let image_data = ImageData {
    width,
    height,
    data: data_ptr,
  };
  let arraybuffer = unsafe {
    ctx
      .env
      .create_arraybuffer_with_borrowed_data(data_ptr, arraybuffer_length, 0, noop_finalize)
  }?;
  let typed_array =
    arraybuffer
      .into_raw()
      .into_typedarray(TypedArrayType::Uint8Clamped, arraybuffer_length, 0)?;

  let mut this = ctx.this_unchecked::<JsObject>();
  ctx.env.wrap(&mut this, image_data)?;
  this.define_properties(&[
    Property::new(&ctx.env, "data")?
      .with_value(typed_array)
      .with_property_attributes(PropertyAttributes::Enumerable),
    Property::new(&ctx.env, "width")?
      .with_value(js_width)
      .with_property_attributes(PropertyAttributes::Enumerable),
    Property::new(&ctx.env, "height")?
      .with_value(js_height)
      .with_property_attributes(PropertyAttributes::Enumerable),
  ])?;
  ctx.env.get_undefined()
}

pub struct Image {
  pub bitmap: Option<Bitmap>,
  pub complete: bool,
  pub alt: String,
}

impl Image {
  pub fn create_js_class(env: &Env) -> Result<JsFunction> {
    env.define_class(
      "Image",
      image_constructor,
      &vec![
        Property::new(&env, "width")?
          .with_getter(get_width)
          .with_property_attributes(PropertyAttributes::Enumerable),
        Property::new(&env, "height")?
          .with_getter(get_height)
          .with_property_attributes(PropertyAttributes::Enumerable),
        Property::new(&env, "naturalWidth")?
          .with_getter(get_width)
          .with_property_attributes(PropertyAttributes::Enumerable),
        Property::new(&env, "naturalHeight")?
          .with_getter(get_height)
          .with_property_attributes(PropertyAttributes::Enumerable),
        Property::new(&env, "complete")?
          .with_getter(get_complete)
          .with_property_attributes(PropertyAttributes::Enumerable),
        Property::new(&env, "alt")?
          .with_setter(set_alt)
          .with_getter(get_alt),
        Property::new(&env, "src")?
          .with_setter(set_src)
          .with_getter(get_src),
      ],
    )
  }
}

#[js_function]
fn image_constructor(ctx: CallContext) -> Result<JsUndefined> {
  let js_image = Image {
    complete: false,
    bitmap: None,
    alt: "".to_string(),
  };
  let mut this = ctx.this_unchecked::<JsObject>();
  ctx.env.wrap(&mut this, js_image)?;

  ctx.env.get_undefined()
}

#[js_function]
fn get_width(ctx: CallContext) -> Result<JsNumber> {
  let this = ctx.this_unchecked::<JsObject>();
  let image = ctx.env.unwrap::<Image>(&this)?;

  ctx
    .env
    .create_double(image.bitmap.as_ref().unwrap().width as f64)
}

#[js_function]
fn get_height(ctx: CallContext) -> Result<JsNumber> {
  let this = ctx.this_unchecked::<JsObject>();
  let image = ctx.env.unwrap::<Image>(&this)?;

  ctx
    .env
    .create_double(image.bitmap.as_ref().unwrap().height as f64)
}

#[js_function]
fn get_complete(ctx: CallContext) -> Result<JsBoolean> {
  let this = ctx.this_unchecked::<JsObject>();
  let image = ctx.env.unwrap::<Image>(&this)?;

  ctx.env.get_boolean(image.complete)
}

#[js_function]
fn get_alt(ctx: CallContext) -> Result<JsString> {
  let this = ctx.this_unchecked::<JsObject>();
  let image = ctx.env.unwrap::<Image>(&this)?;

  ctx.env.create_string(image.alt.as_str())
}

#[js_function(1)]
fn set_alt(ctx: CallContext) -> Result<JsUndefined> {
  let this = ctx.this_unchecked::<JsObject>();
  let mut image = ctx.env.unwrap::<Image>(&this)?;
  let arg = ctx.get::<JsString>(0)?.into_utf8()?;
  image.alt = arg.as_str()?.to_string();

  ctx.env.get_undefined()
}

#[js_function]
fn get_src(ctx: CallContext) -> Result<JsUndefined> {
  ctx.env.get_undefined()
}

#[js_function(1)]
fn set_src(ctx: CallContext) -> Result<JsUndefined> {
  let this = ctx.this_unchecked::<JsObject>();
  let image = ctx.env.unwrap::<Image>(&this)?;

  let src_arg = ctx.get::<JsUnknown>(0)?;
  let src_data_ab = unsafe { src_arg.cast::<JsTypedArray>() }.into_value()?;
  if src_data_ab.typedarray_type != TypedArrayType::Uint8 {
    return Err(Error::new(
      Status::InvalidArg,
      "Image src setter: Argument 1 does not implement interface Buffer.".to_owned(),
    ));
  }
  let length = src_data_ab.len();

  image.complete = true;
  image
    .bitmap
    .get_or_insert(Bitmap::from_buffer(src_data_ab.as_ptr() as *mut u8, length));

  ctx.env.get_undefined()
}
