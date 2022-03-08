use weresocool::error::Error;

pub fn make_shader(device: &wgpu::Device, path: &str) -> Result<wgpu::ShaderModule, Error> {
    Ok(device.create_shader_module(&wgpu::ShaderModuleDescriptor {
        label: Some("Shader"),
        source: wgpu::ShaderSource::Wgsl(
            std::fs::read_to_string(path)
                .expect(format!("unable to open: {}", path).as_str())
                .into(),
        ),
    }))
}
