use async_trait::async_trait;

#[async_trait]
pub trait Renderable {
    async fn init() -> Self;
    fn update(&mut self, dt: std::time::Duration);
    fn render(&mut self);
}
