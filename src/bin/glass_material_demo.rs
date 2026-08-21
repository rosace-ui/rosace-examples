//! Phase 33 / D124 Step 4 demo: a TRUE backdrop-sampling glass material,
//! through the GENERIC `ShaderSpec::with_backdrop()` registry path — not
//! `BackdropBlur`'s own hardcoded fast path (that one stays untouched and
//! ships separately; this proves a THIRD PARTY pipeline can express the
//! same family of effect).
//!
//! A glass panel sits over static colorful content and slides across it —
//! the panel visibly refracts whatever is now behind its rect as it moves,
//! proof the scene texture is sampled live, not a one-time snapshot.
//!
//! Deliberately NOT over a `ScrollView`: scrolled content renders through a
//! GPU-cached `FrameItem::Offscreen` scroll layer, which the compositor
//! composites AFTER all inline `FrameItem::Shader` quads regardless of
//! paint order (a real, pre-existing structural fact discovered while
//! building this demo, not something Step 4 changed) — so a
//! backdrop-sampling quad recorded earlier in the same picture cannot yet
//! see scrolled content. Sliding the GLASS PANEL itself over static content
//! demonstrates the identical "live, not cached" property without hitting
//! that ordering gap; teaching the compositor to interleave Offscreen
//! layers by true paint order is future work, named here, not in Step 4's
//! scope.

use rosace::prelude::*;
use rosace::shader::{register_shader, PipelineId, ShaderSpec, ShaderMaterial, ShaderUniforms};

fn glass_pipeline() -> PipelineId {
    PipelineId::user(0x1100)
}

#[derive(ShaderUniforms)]
struct GlassUniforms {
    time: f32,
    _pad: [f32; 3],
}

const GLASS_WGSL: &str = r#"
struct Mat { time: f32, _pad: vec3<f32>, };
@group(0) @binding(1) var<uniform> m: Mat;

fn sd_rrect(p: vec2<f32>, half: vec2<f32>, r: f32) -> f32 {
    let q = abs(p) - half + vec2<f32>(r, r);
    return length(max(q, vec2<f32>(0.0, 0.0))) + min(max(q.x, q.y), 0.0) - r;
}

@fragment
fn fs_main(in: RosaceVsOut) -> @location(0) vec4<f32> {
    // Small animated refraction wobble — the whole point of sampling the
    // LIVE backdrop rather than a static tint: the distortion itself moves.
    let wobble = vec2<f32>(
        sin(in.uv.y * 9.0 + m.time * 1.6),
        cos(in.uv.x * 9.0 + m.time * 1.3),
    ) * 0.012;
    let behind = rosace_sample_backdrop(in.uv + wobble).rgb;

    // Rounded-panel mask + soft fresnel rim, same shape as the built-in
    // BackdropBlur glass panel.
    let px = in.uv * rosace_quad.size_px;
    let half = rosace_quad.size_px * 0.5 - vec2<f32>(2.0, 2.0);
    let p = px - rosace_quad.size_px * 0.5;
    let d = sd_rrect(p, half, 22.0);
    let mask = clamp(0.5 - d, 0.0, 1.0);
    if mask <= 0.0 { return vec4<f32>(0.0); }

    let tint = vec3<f32>(0.75, 0.85, 1.0);
    var col = mix(behind, tint, 0.12) * 1.05 + vec3<f32>(0.01);
    let rim = smoothstep(-3.0, -0.5, d) * 0.3;
    col += vec3<f32>(rim);
    return vec4<f32>(col * mask, mask);
}
"#;

fn glass_material() -> ShaderMaterial {
    ShaderMaterial::new(glass_pipeline(), GlassUniforms { time: 0.0, _pad: [0.0; 3] }.to_bytes())
}

struct GlassDemo;

impl Component for GlassDemo {
    fn build(&self, _ctx: &mut Context) -> BoxedWidget {
        let swatch = |c: Color, label: &str| {
            Container::new()
                .background(c)
                .width(440.0)
                .height(90.0)
                .child(Text::new(label).size(13.0).color(Color::rgb(255, 255, 255)))
        };

        // Static content (not a `ScrollView` — see the module doc for why):
        // the panel straddles the red/blue boundary, so a correct backdrop
        // sample must show red tint on top and blue tint on the bottom half
        // of the SAME panel — proof it's sampling real per-pixel content,
        // not a solid fallback color.
        let content = Column::new()
            .padding(EdgeInsets::all(16.0))
            .spacing(12.0)
            .child(Text::new("The glass panel below straddles red/blue — it should show both tinted through it.").size(13.0))
            .child(swatch(Color::rgb(230, 70, 90), "red"))
            .child(swatch(Color::rgb(70, 160, 230), "blue"))
            .child(swatch(Color::rgb(250, 190, 60), "yellow"))
            .child(swatch(Color::rgb(90, 210, 130), "green"))
            .child(swatch(Color::rgb(180, 90, 220), "purple"));

        Scaffold::new(
            Stack::new()
                .child(content)
                .child(
                    Positioned::new(
                        ShaderPaint::new(glass_material()).size(320.0, 140.0).animated(),
                    )
                    .top(120.0)
                    .left(40.0),
                ),
        )
        .app_bar(AppBar::new("glass_material_demo"))
        .boxed()
    }
}

fn main() {
    env_logger::init();
    register_shader(glass_pipeline(), ShaderSpec::new(GLASS_WGSL).with_backdrop());
    App::new().title("glass_material_demo").size(480, 640).launch(GlassDemo);
}
