//! Phase 33 / D124 demo: custom shader materials via `ShaderPaint`.
//!
//! Shows the three starter-library materials (no WGSL written by the app)
//! AND one app-authored raw-WGSL material — proving both the curated layer
//! and the open registry underneath.

use rosace::prelude::*;
use rosace::shader::{register_shader, PipelineId, ShaderSpec, ShaderMaterial};
use rosace::shader::ShaderUniforms;

/// A custom app pipeline id (user range, clear of the 0x100 starter block).
fn plasma_id() -> PipelineId {
    PipelineId::user(0x1000)
}

/// App-authored uniforms — `time` first (the standard animated slot).
#[derive(ShaderUniforms)]
struct PlasmaUniforms {
    time: f32,
    _pad: [f32; 3],
}

const PLASMA_WGSL: &str = r#"
struct Mat { time: f32, };
@group(0) @binding(1) var<uniform> m: Mat;

@fragment
fn fs_main(in: RosaceVsOut) -> @location(0) vec4<f32> {
    let uv = in.uv * 6.0;
    let v = sin(uv.x + m.time) + sin(uv.y + m.time * 0.7)
          + sin((uv.x + uv.y) * 0.7 + m.time * 1.3);
    let t = 0.5 + 0.5 * sin(v);
    let c = vec3<f32>(0.15 + 0.6 * t, 0.1, 0.5 + 0.4 * (1.0 - t));
    return vec4<f32>(c, 1.0); // opaque premultiplied
}
"#;

fn plasma() -> ShaderMaterial {
    ShaderMaterial::new(plasma_id(), PlasmaUniforms { time: 0.0, _pad: [0.0; 3] }.to_bytes())
        .fallback(Color::rgb(40, 20, 70))
}

struct ShaderDemo;

impl Component for ShaderDemo {
    fn build(&self, _ctx: &mut Context) -> Element {
        let tile = |label: &str, w: ShaderPaint| {
            Column::new()
                .spacing(4.0)
                .child(w.size(150.0, 90.0))
                .child(Text::new(label).size(12.0))
        };

        Scaffold::new(ScrollView::new(
            Column::new()
                .padding(EdgeInsets::all(20.0))
                .spacing(16.0)
                .child(Text::new("Starter materials (no WGSL written)").size(15.0))
                .child(
                    Row::new()
                        .spacing(16.0)
                        .child(tile("gradient", ShaderPaint::new(
                            materials::gradient(Color::rgb(120, 90, 255), Color::rgb(60, 200, 220), 0.6, 0.25),
                        ).animated()))
                        .child(tile("noise", ShaderPaint::new(
                            materials::noise(Color::rgb(40, 44, 66), 0.35),
                        ).animated()))
                        .child(tile("glow", ShaderPaint::new(
                            materials::glow(Color::rgb(140, 120, 255), 0.5, 3.0),
                        ).animated())),
                )
                .child(Text::new("App-authored raw WGSL material").size(15.0))
                .child(ShaderPaint::new(plasma()).size(482.0, 120.0).animated())
                .child(Text::new("Material cascade — Container/Card (Phase 33 Step 3)").size(15.0))
                .child(Text::new("Two cards get the theme's CardMaterial 'for free'; the third overrides it per-instance.").size(11.0))
                .child(
                    Row::new()
                        .spacing(16.0)
                        .child(Card::new(Text::new("theme default").size(12.0)).radius(12.0).width(150.0))
                        .child(Card::new(Text::new("theme default").size(12.0)).radius(12.0).width(150.0))
                        .child(
                            Card::new(Text::new("instance override").size(12.0))
                                .radius(12.0)
                                .width(150.0)
                                .material(materials::glow(Color::rgb(255, 140, 60), 0.5, 2.0)),
                        ),
                )
                .child(Text::new("Surface-widget rollout (Phase 33 Step 5)").size(15.0))
                .child(Text::new("The app bar above already carries the theme's AppBarMaterial; this bottom nav overrides it per-instance.").size(11.0))
                .child(
                    BottomNavigationBar::new()
                        .material(materials::noise(Color::rgb(30, 20, 50), 0.25))
                        .item(BottomNavItem::new("Home").active())
                        .item(BottomNavItem::new("Search"))
                        .item(BottomNavItem::new("Profile")),
                ),
        ))
        .app_bar(AppBar::new("shader_demo"))
        .into_element()
    }
}

fn main() {
    env_logger::init();
    // Register once at startup — pipelines compile eagerly (D109), before
    // the first frame that draws with them.
    materials::register_starter_materials();
    register_shader(plasma_id(), ShaderSpec::new(PLASMA_WGSL));

    // "Register once" story: one CardMaterial on the theme → every plain
    // Card in the app gets it, no per-card code (Phase 33 Step 3 exit bar).
    // AppBarMaterial does the same for the app bar (Phase 33 Step 5).
    let theme = rosace::dark_theme()
        .with_ext(rosace::CardMaterial(
            materials::gradient(Color::rgb(90, 60, 200), Color::rgb(200, 80, 160), 0.9, 0.15),
        ))
        .with_ext(rosace::AppBarMaterial(
            materials::gradient(Color::rgb(40, 30, 70), Color::rgb(70, 40, 90), 0.0, 0.1),
        ));

    App::new().title("shader_demo").size(560, 640).theme(theme).launch(ShaderDemo);
}
