//! A full (small) app built on the D124 material system: an Apple-style
//! Liquid Glass surface with REAL controls living on it.
//!
//! - Background: vivid animated aurora (self-contained `ShaderPaint`).
//! - Surface: a `Card` whose material is a BACKDROP-SAMPLING glass pipeline
//!   (`ShaderSpec::with_backdrop()`, D124 Step 4) — thick-slab edge
//!   refraction, subtle chromatic split, light frost, specular rim. The
//!   glass has NO fallback color by design: painting one would land in the
//!   scene right before the shader quad and get sampled instead of the real
//!   content (the Card/Dialog/… fallback rule exists for exactly this).
//! - On the glass: a `TextInput` (click to focus, drag to select), a
//!   `Dropdown` (the "spinner"), and a `CircularProgress` spinner.
//!
//! Not scrollable on purpose: backdrop-sampling quads inside a GPU scroll
//! layer are a named, deferred limitation (see glass_material_demo).

use rosace::prelude::*;
use rosace::shader::{register_shader, PipelineId, ShaderSpec, ShaderMaterial, ShaderUniforms};

fn aurora_id() -> PipelineId { PipelineId::user(0x2000) }
fn glass_id() -> PipelineId { PipelineId::user(0x2001) }

// ── Background: slow-drifting color field ─────────────────────────────────

#[derive(ShaderUniforms)]
struct AuroraUniforms {
    time: f32, // first = the standard animated slot (patch_time)
    _p1: f32,
    _p2: f32,
    _p3: f32,
}

const AURORA_WGSL: &str = r#"
struct Bg { time: f32, p1: f32, p2: f32, p3: f32, };
@group(0) @binding(1) var<uniform> b: Bg;

@fragment
fn fs_main(in: RosaceVsOut) -> @location(0) vec4<f32> {
    let uv = in.uv;
    let t = b.time * 0.4;
    // Tight, saturated blobs over a near-black indigo base — the first
    // attempt summed wide gaussians in LINEAR space and the sRGB encode
    // washed the whole window into pale pastel.
    let c1 = vec2<f32>(0.30 + 0.25 * sin(t * 0.7), 0.35 + 0.20 * cos(t * 0.9));
    let c2 = vec2<f32>(0.75 + 0.20 * sin(t * 1.1 + 2.0), 0.30 + 0.25 * cos(t * 0.6 + 1.0));
    let c3 = vec2<f32>(0.50 + 0.30 * sin(t * 0.5 + 4.0), 0.80 + 0.15 * cos(t * 0.8 + 3.0));
    let w1 = exp(-16.0 * dot(uv - c1, uv - c1));
    let w2 = exp(-16.0 * dot(uv - c2, uv - c2));
    let w3 = exp(-14.0 * dot(uv - c3, uv - c3));
    var col = vec3<f32>(0.010, 0.008, 0.030);
    col += w1 * vec3<f32>(0.45, 0.03, 0.20);
    col += w2 * vec3<f32>(0.02, 0.16, 0.55);
    col += w3 * vec3<f32>(0.50, 0.22, 0.03);
    return vec4<f32>(col, 1.0);
}
"#;

fn aurora() -> ShaderMaterial {
    let u = AuroraUniforms { time: 0.0, _p1: 0.0, _p2: 0.0, _p3: 0.0 };
    ShaderMaterial::new(aurora_id(), u.to_bytes()).fallback(Color::rgb(18, 14, 34))
}

// ── The glass ─────────────────────────────────────────────────────────────

/// All-scalar uniforms (4 × f32 = one 16-byte row) so every parameter is
/// tunable from Rust without touching the WGSL during visual iteration.
#[derive(ShaderUniforms)]
struct GlassUniforms {
    radius: f32,
    refract_px: f32,
    frost_px: f32,
    bright: f32,
}

const GLASS_WGSL: &str = r#"
struct Mat { radius: f32, refract_px: f32, frost_px: f32, bright: f32, };
@group(0) @binding(1) var<uniform> m: Mat;

fn sd_rrect(p: vec2<f32>, half: vec2<f32>, r: f32) -> f32 {
    let q = abs(p) - half + vec2<f32>(r, r);
    return length(max(q, vec2<f32>(0.0, 0.0))) + min(max(q.x, q.y), 0.0) - r;
}

@fragment
fn fs_main(in: RosaceVsOut) -> @location(0) vec4<f32> {
    let size = rosace_quad.size_px;
    let px = in.uv * size;
    let p = px - size * 0.5;
    let d = sd_rrect(p, size * 0.5 - vec2<f32>(1.0, 1.0), m.radius);
    let mask = clamp(0.5 - d, 0.0, 1.0);

    // Thick-slab refraction: bend the sample OUTWARD near the rim, like
    // looking through the beveled edge of real glass. Quadratic falloff so
    // the panel center stays optically flat.
    let edge = 22.0;
    let bend = pow(smoothstep(-edge, 0.0, d), 2.0);
    let dir = p / max(length(p), 0.001);
    let uv_off = dir * bend * m.refract_px / size;

    // Subtle chromatic split on the refracted rim (real dispersion).
    let ca = 1.0 + bend * 0.06;
    var col: vec3<f32>;
    col.r = rosace_sample_backdrop(in.uv + uv_off * ca).r;
    col.g = rosace_sample_backdrop(in.uv + uv_off).g;
    col.b = rosace_sample_backdrop(in.uv + uv_off / ca).b;

    // Light frost: 4 extra taps in a cross — enough to soften what's
    // behind so controls stay legible, nowhere near a real gaussian cost.
    let fr = vec2<f32>(m.frost_px, m.frost_px) / size;
    var acc = col * 0.40;
    acc += rosace_sample_backdrop(in.uv + uv_off + vec2<f32>(fr.x, 0.0)).rgb * 0.15;
    acc += rosace_sample_backdrop(in.uv + uv_off - vec2<f32>(fr.x, 0.0)).rgb * 0.15;
    acc += rosace_sample_backdrop(in.uv + uv_off + vec2<f32>(0.0, fr.y)).rgb * 0.15;
    acc += rosace_sample_backdrop(in.uv + uv_off - vec2<f32>(0.0, fr.y)).rgb * 0.15;

    // Smoked-glass body: tint toward a deep neutral so the light theme
    // text on top stays legible over ANY backdrop (the white-lift version
    // of this washed out into white-on-white), then a gentle lift.
    var glass = mix(acc, vec3<f32>(0.030, 0.036, 0.070), 0.38) * m.bright;
    glass += vec3<f32>(0.012);

    // Specular rim — strongest along the top edge, fading down the sides.
    let rim = smoothstep(-3.0, -0.5, d);
    glass += rim * (0.08 + 0.22 * (1.0 - in.uv.y));

    // Soft inner shade toward the bottom edge for depth.
    glass -= smoothstep(-6.0, -0.5, d) * in.uv.y * 0.04;

    // No branch anywhere: texture sampling stays in uniform control flow,
    // and outside the mask this is premultiplied transparent black.
    return vec4<f32>(glass * mask, mask);
}
"#;

const GLASS_RADIUS: f32 = 26.0;

fn liquid_glass() -> ShaderMaterial {
    let u = GlassUniforms {
        radius: GLASS_RADIUS,
        refract_px: 20.0,
        frost_px: 3.5,
        bright: 1.0,
    };
    // Deliberately NO fallback — see the module doc.
    ShaderMaterial::new(glass_id(), u.to_bytes())
}

// ── The app ───────────────────────────────────────────────────────────────

struct LiquidGlassApp;

impl Component for LiquidGlassApp {
    fn build(&self, ctx: &mut Context) -> BoxedWidget {
        let name: Atom<String> = ctx.state(String::from("Select some of this text"));
        let scene: Atom<usize> = ctx.state(0usize);
        let scene_open: Atom<bool> = ctx.state(false);

        let controls = Column::new()
            .spacing(14.0)
            .child(Text::title("Liquid Glass"))
            .child(Text::caption("Real backdrop refraction — drag over the text to select it."))
            .child(
                TextInput::new()
                    .placeholder("Your name")
                    .value(name.get())
                    .width(300.0)
                    .background(Color::rgba(255, 255, 255, 26))
                    .border(Color::rgba(255, 255, 255, 80))
                    .on_change({
                        let name = name.clone();
                        move |v| name.set(v)
                    }),
            )
            .child(
                Dropdown::new(vec!["Aurora", "Plasma", "Nebula"], scene.get(), scene_open.get())
                    .width(300.0)
                    .background(Color::rgba(255, 255, 255, 26))
                    .on_change({
                        let scene = scene.clone();
                        move |i| scene.set(i)
                    }),
            )
            .child(
                Row::new()
                    .spacing(10.0)
                    // A GPU-resident pulse, not `CircularProgress::spinner()`:
                    // the spinner is a CPU-drawn animated arc that forces an
                    // engine repaint every frame (~a full debug core on its
                    // own — the animated-widget damage-granularity debt,
                    // named in PHASE_33 follow-ups). The glow material
                    // animates entirely through the D109 fast path.
                    .child(ShaderPaint::new(
                        materials::glow(Color::rgb(160, 150, 255), 0.45, 2.2),
                    ).size(22.0, 22.0).animated())
                    .child(Text::caption("Rendering through real glass…")),
            );

        // Structured content BEHIND the glass — refraction is invisible
        // over a smooth gradient; the eye needs edges to see them bend.
        let mut backdrop_text = Column::new().spacing(18.0).padding(EdgeInsets::all(16.0));
        for i in 0..9 {
            let c = match i % 3 {
                0 => Color::rgb(255, 214, 140),
                1 => Color::rgb(150, 214, 255),
                _ => Color::rgb(255, 150, 190),
            };
            backdrop_text = backdrop_text.child(
                Text::new("ROSACE · LIQUID · GLASS · ROSACE").size(26.0).color(c),
            );
        }

        Scaffold::new(
            Stack::new()
                .child(ShaderPaint::new(aurora()).animated())
                .child(backdrop_text)
                .child(
                    Positioned::new(
                        Card::new(controls)
                            .material(liquid_glass())
                            .radius(GLASS_RADIUS)
                            .no_border()
                            .elevation(10.0)
                            .padding(EdgeInsets::all(24.0))
                            .width(360.0),
                    )
                    .top(120.0)
                    .left(60.0),
                ),
        )
        .boxed()
    }
}

fn main() {
    env_logger::init();
    register_shader(aurora_id(), ShaderSpec::new(AURORA_WGSL));
    register_shader(glass_id(), ShaderSpec::new(GLASS_WGSL).with_backdrop());

    // The liquid theme: glass text selection registered ONCE, globally —
    // every TextInput/TextArea in the app renders the magnifier-lens
    // selection with no per-widget code. A Material app registers
    // nothing (or SelectionStyle::flat()) and keeps the flat look.
    let theme = rosace::dark_theme().with_ext(SelectionStyle::glass());

    App::new().title("liquid_glass_app").size(480, 640).theme(theme).launch(LiquidGlassApp);
}
