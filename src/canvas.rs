use super::{WinitAppTrait, winit::WinitWindow};

use image::RgbaImage;

use wgpu_canvas::CanvasAtlas;
pub use wgpu_canvas::{Font, Image};

use std::time::Instant;

mod structs;
pub use structs::{Area, CanvasItem, ShapeType, Text};
use structs::Size;

mod renderer;
use renderer::Canvas;

#[derive(Default)]
pub struct CanvasContext{
    components: Vec<(wgpu_canvas::Area, wgpu_canvas::CanvasItem)>,
    pub atlas: CanvasAtlas,
    pub size: Size,
    pub position: (u32, u32),
}

impl CanvasContext {
    pub fn new_font(&mut self, font: Vec<u8>) -> Font {
        Font::new(&mut self.atlas, font)
    }
    pub fn new_image(&mut self, image: RgbaImage) -> Image {
        Image::new(&mut self.atlas, image)
    }

  //fn parse_svg(&mut self, bytes: &[u8], min_size: u32, color: &'static str) -> RgbaImage {
  //    let size = self.size.scale_physical(size);
  //    let mut content = std::str::from_utf8(bytes).unwrap();
  //    content = content.replace("fill=\"white\"", &format!("fill=\"#{}\"", color));
  //    let svg = nsvg::parse_str(&content, nsvg::Units::Pixel, 96.0).unwrap();
  //    let rgba = svg.rasterize(min_size as f32/ svg.width().min(svg.height).ceil()).unwrap();
  //    RgbaImage::from_raw(rgba.dimensions().0, rgba.dimensions().1, rgba.into_raw()).unwrap()
  //}

    pub fn width(&self) -> u32 {self.size.logical().0}
    pub fn height(&self) -> u32 {self.size.logical().1}

    pub fn clear(&mut self, color: &'static str) {
        self.components.clear();
        self.components.push((
            Area((0, 0), None).into_inner(u16::MAX, &self.size),
            CanvasItem::Shape(
                ShapeType::Rectangle(0, self.size.logical()),
                color, 255
            ).into_inner(&self.size)
        ));
    }

    pub fn draw(&mut self, area: Area, item: CanvasItem) {
        let z = u16::MAX-1-(self.components.len()) as u16;
        let area = area.into_inner(z, &self.size);
        self.components.push((area, item.into_inner(&self.size)));
    }
}

pub trait CanvasAppTrait {
    fn new(ctx: &mut CanvasContext) -> impl std::future::Future<Output = Self> where Self: Sized;
    fn on_tick(&mut self, ctx: &mut CanvasContext) -> impl std::future::Future<Output = ()>;

    fn on_tick(&mut self, ctx: &mut CanvasContext) -> impl std::future::Future<Output = ()>;
    fn on_click(&mut self, ctx: &mut CanvasContext) -> impl std::future::Future<Output = ()>;
    fn on_move(&mut self, ctx: &mut CanvasContext) -> impl std::future::Future<Output = ()>;
    fn on_press(&mut self, ctx: &mut CanvasContext, t: String) -> impl std::future::Future<Output = ()>;
}

pub struct CanvasApp<A: CanvasAppTrait> {
    context: CanvasContext,
    canvas: Canvas,
    app: A,
    time: Instant
}

impl<A: CanvasAppTrait> WinitAppTrait for CanvasApp<A> {
    async fn new(window: WinitWindow, width: u32, height: u32, scale_factor: f64) -> Self {
        let mut canvas = Canvas::new(window).await;
        let (width, height) = canvas.resize(width, height);
        let mut context = CanvasContext{
            size: Size::new(width, height, scale_factor),
            ..Default::default()
        };
        let app = A::new(&mut context).await;

        CanvasApp{
            context,
            canvas,
            app,
            time: Instant::now()
        }
    }

    async fn prepare(&mut self, width: u32, height: u32, scale_factor: f64) {
        let (width, height) = self.canvas.resize(width, height);
        self.context.size = Size::new(width, height, scale_factor);

        self.app.on_tick(&mut self.context).await;
        let items = self.context.components.drain(..).collect();

        self.canvas.prepare(&mut self.context.atlas, items);
    }

    async fn render(&mut self) {
        log::error!("last_frame: {}", self.time.elapsed().as_millis());
        self.time = Instant::now();
        self.canvas.render();
    }

    async fn on_click(&mut self) {
        self.app.on_click(&mut self.context).await
    }
    async fn on_move(&mut self, x: u32, y: u32) {
        self.context.position = self.context.size.new_logical(x, y);
        self.app.on_move(&mut self.context).await
    }
    async fn on_press(&mut self, t: String) {
        self.app.on_press(&mut self.context, t).await
    }
}

#[macro_export]
macro_rules! create_canvas_entry_points {
    ($app:ty) => {
        create_winit_entry_points!(CanvasApp::<$app>);
    };
}
