#ifndef SKIA_CAPI_H
#define SKIA_CAPI_H

#include <include/core/SkPathEffect.h>
#include <include/core/SkCanvas.h>
#include <include/core/SkData.h>
#include <include/core/SkGraphics.h>
#include <include/core/SkPaint.h>
#include <include/core/SkSurface.h>
#include <include/effects/SkDashPathEffect.h>
#include <include/effects/SkGradientShader.h>

#include <stdint.h>

typedef struct skiac_surface skiac_surface;
typedef struct skiac_canvas skiac_canvas;
typedef struct skiac_paint skiac_paint;
typedef struct skiac_path skiac_path;
typedef struct skiac_shader skiac_shader;
typedef struct skiac_path_effect skiac_path_effect;
typedef struct skiac_matrix skiac_matrix;

struct skiac_transform
{
  float a;
  float b;
  float c;
  float d;
  float e;
  float f;
};

struct skiac_point
{
  float x;
  float y;
};

struct skiac_surface_data
{
  uint8_t *ptr;
  uint32_t size;
};

extern "C"
{

  // Surface
  skiac_surface *skiac_surface_create_rgba_premultiplied(int width, int height);
  skiac_surface *skiac_surface_create_rgba(int width, int height);
  void skiac_surface_destroy(skiac_surface *c_surface);
  skiac_surface *skiac_surface_copy_rgba(
      skiac_surface *c_surface,
      uint32_t x,
      uint32_t y,
      uint32_t width,
      uint32_t height);
  skiac_canvas *skiac_surface_get_canvas(skiac_surface *c_surface);
  int skiac_surface_get_width(skiac_surface *c_surface);
  int skiac_surface_get_height(skiac_surface *c_surface);
  void skiac_surface_read_pixels(skiac_surface *c_surface, skiac_surface_data *data);
  int skiac_surface_get_alpha_type(skiac_surface *c_surface);
  bool skiac_surface_save(skiac_surface *c_surface, const char *path);

  // Canvas
  void skiac_canvas_clear(skiac_canvas *c_canvas, uint32_t color);
  void skiac_canvas_flush(skiac_canvas *c_canvas);
  void skiac_canvas_set_transform(skiac_canvas *c_canvas, skiac_transform c_ts);
  void skiac_canvas_concat(skiac_canvas *c_canvas, skiac_transform c_ts);
  void skiac_canvas_scale(skiac_canvas *c_canvas, float sx, float sy);
  void skiac_canvas_translate(skiac_canvas *c_canvas, float dx, float dy);
  skiac_transform skiac_canvas_get_total_transform(skiac_canvas *c_canvas);
  void skiac_canvas_draw_color(skiac_canvas *c_canvas, float r, float g, float b, float a);
  void skiac_canvas_draw_path(skiac_canvas *c_canvas, skiac_path *c_path, skiac_paint *c_paint);
  void skiac_canvas_draw_rect(
      skiac_canvas *c_canvas,
      float x, float y, float w, float h,
      skiac_paint *c_paint);
  void skiac_canvas_draw_surface(
      skiac_canvas *c_canvas,
      skiac_surface *c_surface,
      float left,
      float top,
      uint8_t alpha,
      int blend_mode,
      int filter_quality);
  void skiac_canvas_draw_surface_rect(
      skiac_canvas *c_canvas,
      skiac_surface *c_surface,
      float x, float y, float w, float h,
      int filter_quality);
  void skiac_canvas_reset_transform(skiac_canvas *c_canvas);
  void skiac_canvas_clip_rect(skiac_canvas *c_canvas, float x, float y, float w, float h);
  void skiac_canvas_clip_path(skiac_canvas *c_canvas, skiac_path *c_path);
  void skiac_canvas_save(skiac_canvas *c_canvas);
  void skiac_canvas_restore(skiac_canvas *c_canvas);

  // Paint
  skiac_paint *skiac_paint_create();
  void skiac_paint_destroy(skiac_paint *c_paint);
  void skiac_paint_set_style(skiac_paint *c_paint, int style);
  void skiac_paint_set_color(skiac_paint *c_paint, uint8_t r, uint8_t g, uint8_t b, uint8_t a);
  void skiac_paint_set_alpha(skiac_paint *c_paint, uint8_t a);
  uint8_t skiac_paint_get_alpha(skiac_paint *c_paint);
  void skiac_paint_set_anti_alias(skiac_paint *c_paint, bool aa);
  void skiac_paint_set_blend_mode(skiac_paint *c_paint, int blend_mode);
  int skiac_paint_get_blend_mode(skiac_paint *c_paint);
  void skiac_paint_set_shader(skiac_paint *c_paint, skiac_shader *c_shader);
  void skiac_paint_set_stroke_width(skiac_paint *c_paint, float width);
  void skiac_paint_set_stroke_cap(skiac_paint *c_paint, int cap);
  void skiac_paint_set_stroke_join(skiac_paint *c_paint, int join);
  void skiac_paint_set_stroke_miter(skiac_paint *c_paint, float miter);
  float skiac_paint_get_stroke_miter(skiac_paint *c_paint);
  void skiac_paint_set_path_effect(skiac_paint *c_paint, skiac_path_effect *c_path_effect);

  // Path
  skiac_path *skiac_path_create();
  skiac_path *skiac_path_clone(skiac_path *c_path);
  bool skiac_path_op(skiac_path *c_path_one, skiac_path *c_path_two, int op);
  void skiac_path_destroy(skiac_path *c_path);
  void skiac_path_set_fill_type(skiac_path *c_path, int type);
  void skiac_path_arc_to(skiac_path *c_path, float left, float top, float right, float bottom, float startAngle, float sweepAngle, bool forceMoveTo);
  void skiac_path_arc_to_tangent(skiac_path *c_path, float x1, float y1, float x2, float y2, float radius);
  void skiac_path_move_to(skiac_path *c_path, float x, float y);
  void skiac_path_line_to(skiac_path *c_path, float x, float y);
  void skiac_path_cubic_to(
      skiac_path *c_path,
      float x1, float y1, float x2, float y2, float x3, float y3);
  void skiac_path_close(skiac_path *c_path);
  void skiac_path_add_rect(skiac_path *c_path, float l, float t, float r, float b);
  void skiac_path_add_circle(skiac_path *c_path, float x, float y, float r);
  void skiac_path_transform(skiac_path *c_path, skiac_transform c_transform);
  void skiac_path_transform_matrix(skiac_path *c_path, skiac_matrix *c_matrix);
  bool skiac_path_is_empty(skiac_path *c_path);

  // PathEffect
  skiac_path_effect *skiac_path_effect_make_dash_path(const float *intervals, int count, float phase);
  void skiac_path_effect_destroy(skiac_path_effect *c_path_effect);

  // Shader
  skiac_shader *skiac_shader_make_linear_gradient(
      const skiac_point *points,
      const uint32_t *colors,
      const float *positions,
      int count,
      int tile_mode,
      uint32_t flags,
      skiac_transform c_ts);

  skiac_shader *skiac_shader_make_two_point_conical_gradient(
      skiac_point start_point,
      float start_radius,
      skiac_point end_point,
      float end_radius,
      const uint32_t *colors,
      const float *positions,
      int count,
      int tile_mode,
      uint32_t flags,
      skiac_transform c_ts);

  skiac_shader *skiac_shader_make_from_surface_image(
      skiac_surface *c_surface,
      skiac_transform c_ts,
      int filter_quality);

  void skiac_shader_destroy(skiac_shader *c_shader);

  skiac_matrix *skiac_matrix_create();

  void skiac_matrix_pre_translate(skiac_matrix *c_matrix, float dx, float dy);

  void skiac_matrix_pre_rotate(skiac_matrix *c_matrix, float degrees);

  bool skiac_matrix_invert(skiac_matrix *c_matrix, skiac_matrix *inverse);
}

#endif // SKIA_CAPI_H
