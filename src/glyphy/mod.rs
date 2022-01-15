use futures::executor::LocalSpawner;
use futures::task::SpawnExt;
use wgpu_glyph::{
    ab_glyph::{self, InvalidFont},
    GlyphBrush, GlyphBrushBuilder, Section, Text,
};

pub struct Glyphy {
    staging_belt: wgpu::util::StagingBelt,
    local_pool: futures::executor::LocalPool,
    local_spawner: LocalSpawner,
    brush: GlyphBrush<()>,
}

pub struct TextRenderable<'a> {
    pub text: &'a str,
    pub color: Vec<&'a str>,
}

pub fn max_len_text_in_vec_text_renderable(v: &Vec<TextRenderable>) -> usize {
    v.iter().map(|r| r.text.len()).max().unwrap()
}

impl<'a> TextRenderable<'a> {
    pub fn to_string(&self) {}
}

pub fn hex_str_to_rgba<'a>(s: &'a str) -> [f32; 4] {
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
            hex::decode(chunk)
                .expect(format!("unable to decode chuck {} in hex {}", chunk.as_str(), s).as_str())
                [0] as f32
        })
        .collect();

    [rgb[0], rgb[1], rgb[2], 255.0]
}

pub fn hex_str_to_normalized_rgba<'a>(s: &'a str) -> [f32; 4] {
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

impl Glyphy {
    pub fn init(device: &wgpu::Device, format: wgpu::TextureFormat) -> Result<Self, InvalidFont> {
        // Create staging belt and a local pool
        let staging_belt = wgpu::util::StagingBelt::new(1024);
        let local_pool = futures::executor::LocalPool::new();
        let local_spawner = local_pool.spawner();
        // Prepare glyph_brush
        let inconsolata =
            ab_glyph::FontArc::try_from_slice(include_bytes!("Inconsolata-Regular.ttf"))?;
        let brush = GlyphBrushBuilder::using_font(inconsolata).build(&device, format);

        Ok(Self {
            brush,
            staging_belt,
            local_pool,
            local_spawner,
        })
    }

    pub fn render<'a>(
        &mut self,
        texts: Vec<TextRenderable>,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        size: (u32, u32),
        view: &wgpu::TextureView,
        clear: bool,
    ) {
        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Redraw"),
        });

        // Clear frame
        {
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
        }

        let mut offset_y = 0.0;
        let mut offset_x = 0.0;
        let scale = 35.0;

        for text in texts.iter().rev() {
            self.brush.queue(Section {
                screen_position: (
                    scale + offset_x,
                    size.1 as f32 - (scale * texts.len() as f32 + offset_y),
                ),
                bounds: (size.0 as f32, size.1 as f32),
                text: vec![Text::new(&format!("{}:", text.text))
                    .with_color(hex_str_to_normalized_rgba("#dedede"))
                    .with_scale(scale)],
                ..Section::default()
            });

            offset_y += scale;
        }

        let color_offset_x = max_len_text_in_vec_text_renderable(&texts) as f32 * scale * 0.55;
        offset_x = color_offset_x;
        offset_y = 0.0;

        for text in texts.iter().rev() {
            for color in text.color.iter() {
                self.brush.queue(Section {
                    screen_position: (
                        scale + offset_x,
                        size.1 as f32 - (scale * texts.len() as f32 + offset_y),
                    ),
                    bounds: (size.0 as f32, size.1 as f32),
                    text: vec![Text::new(color)
                        .with_color(hex_str_to_normalized_rgba(color))
                        .with_scale(scale)],
                    ..Section::default()
                });

                offset_x += scale * 4.0 as f32;
            }
            offset_x = color_offset_x;
            offset_y += scale;
        }

        // Draw the text!
        self.brush
            .draw_queued(
                &device,
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
