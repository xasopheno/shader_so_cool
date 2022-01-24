use futures::executor::LocalSpawner;
use futures::task::SpawnExt;
use wgpu_glyph::{
    ab_glyph::{self, InvalidFont},
    GlyphBrush, GlyphBrushBuilder, Section, Text,
};

use crate::{renderable::GlyphyConfig, NamedValue};

pub struct Glyphy {
    config: GlyphyConfig,
    staging_belt: wgpu::util::StagingBelt,
    local_pool: futures::executor::LocalPool,
    local_spawner: LocalSpawner,
    brush: GlyphBrush<()>,
}

type TextRenderable<'a> = NamedValue<'a, Vec<&'a str>>;

impl GlyphyConfig {
    pub fn render(&mut self, brush: &mut GlyphBrush<()>, size: (u32, u32)) {
        match self {
            GlyphyConfig::GlyphyNamedColorSetConfig { text: t, location } => {
                let mut offset_x = location.0 * size.0 as f32;
                let mut offset_y = location.1 * size.1 as f32;
                let scale = 35.0;

                for text in t.iter().rev() {
                    brush.queue(Section {
                        screen_position: (scale + offset_x, scale * t.len() as f32 + offset_y),
                        bounds: (size.0 as f32, size.1 as f32),
                        text: vec![Text::new(&format!("{}:", text.0))
                            .with_color(hex_str_to_normalized_rgba("#dedede"))
                            .with_scale(scale)],
                        ..Section::default()
                    });

                    offset_y += scale;
                }

                let color_offset_x =
                    max_len_text_in_vec_text_renderable(&t) as f32 * scale * 0.7 + offset_x;
                offset_x = color_offset_x;
                offset_y = location.1 * size.1 as f32;

                for text in t.iter().rev() {
                    for color in text.1.iter() {
                        brush.queue(Section {
                            screen_position: (scale + offset_x, scale * t.len() as f32 + offset_y),
                            bounds: (size.0 as f32, size.1 as f32),
                            text: vec![Text::new(color)
                                .with_color(hex_str_to_normalized_rgba(color))
                                .with_scale(scale)],
                            ..Section::default()
                        });

                        offset_x += scale * 4.0_f32;
                    }
                    offset_x = color_offset_x;
                    offset_y += scale;
                }
            }
            _ => {
                todo!()
            }
        }
    }
}

impl Glyphy {
    pub fn init(
        device: &wgpu::Device,
        format: wgpu::TextureFormat,
        config: GlyphyConfig,
    ) -> Result<Self, InvalidFont> {
        // Create staging belt and a local pool
        let staging_belt = wgpu::util::StagingBelt::new(1024);
        let local_pool = futures::executor::LocalPool::new();
        let local_spawner = local_pool.spawner();
        // Prepare glyph_brush
        let inconsolata =
            ab_glyph::FontArc::try_from_slice(include_bytes!("Inconsolata-Regular.ttf"))?;
        let brush = GlyphBrushBuilder::using_font(inconsolata).build(device, format);

        Ok(Self {
            config,
            brush,
            staging_belt,
            local_pool,
            local_spawner,
        })
    }

    pub fn render(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        size: (u32, u32),
        view: &wgpu::TextureView,
        clear: bool,
    ) {
        let encoder = self.prepare_render(device, view, clear);

        self.config.render(&mut self.brush, size);

        self.finalize_render(device, queue, size, view, encoder);
    }

    fn prepare_render(
        &self,
        device: &wgpu::Device,
        view: &wgpu::TextureView,
        clear: bool,
    ) -> wgpu::CommandEncoder {
        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Redraw"),
        });

        let _ = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render pass"),
            color_attachments: &[wgpu::RenderPassColorAttachment {
                view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: if clear {
                        wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.0,
                            g: 0.0,
                            b: 0.0,
                            a: 1.0,
                        })
                    } else {
                        wgpu::LoadOp::Load
                    },
                    store: true,
                },
            }],
            depth_stencil_attachment: None,
        });

        encoder
    }

    fn finalize_render(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        size: (u32, u32),
        view: &wgpu::TextureView,
        mut encoder: wgpu::CommandEncoder,
    ) {
        self.brush
            .draw_queued(
                device,
                &mut self.staging_belt,
                &mut encoder,
                view,
                size.0,
                size.1,
            )
            .expect("Draw queued");

        // Submit the work
        self.staging_belt.finish();
        queue.submit(Some(encoder.finish()));

        // Recall unused staging buffers
        self.local_spawner
            .spawn(self.staging_belt.recall())
            .expect("Recall staging belt");

        self.local_pool.run_until_stalled();
    }
}

pub fn max_len_text_in_vec_text_renderable(v: &[TextRenderable]) -> usize {
    v.iter().map(|r| r.0.len()).max().unwrap()
}

pub fn hex_str_to_rgba(s: &str) -> [f32; 4] {
    let re = regex::Regex::new(r"#([a-fA-F0-9]{6})").unwrap();
    if !re.is_match(s) {
        panic!("{} is not in hex format", s);
    };

    let rgb: Vec<f32> = s[1..]
        .chars()
        .collect::<Vec<char>>()
        .chunks(2)
        .map(|c| c.iter().collect::<String>())
        .collect::<Vec<String>>()
        .iter()
        .map(|chunk| {
            hex::decode(chunk).unwrap_or_else(|_| {
                panic!("unable to decode chuck {} in hex {}", chunk.as_str(), s)
            })[0] as f32
        })
        .collect();

    [rgb[0], rgb[1], rgb[2], 255.0]
}

pub fn hex_str_to_normalized_rgba(s: &str) -> [f32; 4] {
    let rgba = hex_str_to_rgba(s)
        .iter()
        .map(|v| v / 255.0)
        .collect::<Vec<f32>>();

    [rgba[0], rgba[1], rgba[2], rgba[3]]
}

#[test]
#[should_panic]
fn test_bad_hex_str_to_rgba() {
    let bad_hex_str = "af4573";
    hex_str_to_rgba(bad_hex_str);
}

#[test]
#[should_panic]
fn test_bad_hex_str_to_rgba_2() {
    let bad_hex_str = "#af457";
    hex_str_to_rgba(bad_hex_str);
}

#[test]
fn test_hex_str_to_rgba() {
    let hex_str = "#af4573";
    let rgba = hex_str_to_rgba(hex_str);
    assert_eq!(rgba, [175.0, 69.0, 115.0, 255.0]);
}

#[test]
fn test_hex_str_to_normalized_rgba() {
    let hex_str = "#af4573";
    let rgba = hex_str_to_normalized_rgba(hex_str);
    assert_eq!(rgba, [0.6862745, 0.27058825, 0.4509804, 1.0,])
}
