use wgpu_canvas::CanvasAtlas;
pub use crate::winit::{MouseEvent, MouseState, KeyboardEvent, KeyboardState, NamedKey, Key, SmolStr};
use crate::{WinitAppTrait, winit::{WinitWindow, Scheduler, Callback}};

use std::time::{Duration, Instant};
use std::future::Future;

mod structs;
use structs::Size;
pub use structs::{Area, Color, CanvasItem, Shape, Text, Image, Font};

use crate::state::{State, Field};

mod renderer;
use renderer::Canvas;

pub struct CanvasContext{
    components: Vec<(wgpu_canvas::Area, wgpu_canvas::CanvasItem)>,
    scheduler: Scheduler,
    atlas: CanvasAtlas,
    size: Size,
    state: State,
    triggered_keyboard: Vec<KeyboardEvent>
}

impl CanvasContext {
    pub fn clear(&mut self, color: Color) {
        self.components.clear();
        self.components.push((
            Area((0, 0), None).into_inner(u16::MAX, &self.size),
            CanvasItem::Shape(
                Shape::Rectangle(0, self.size.logical()),
                color
            ).into_inner(&self.size)
        ));
    }

    pub fn draw(&mut self, area: Area, item: CanvasItem) {
        let z = u16::MAX-1-(self.components.len()) as u16;
        let area = area.into_inner(z, &self.size);
        self.components.push((area, item.into_inner(&self.size)));
    }

    pub fn state(&mut self) -> &mut State {&mut self.state}

    pub fn schedule_task<
        F: Fn() -> Fut + Send + 'static,
        Fut: Future<Output = (Option<Duration>, Callback)> + 'static + Send
    >(
        &self, duration: Duration, task: F
    ) {
        self.scheduler.schedule_task(duration, task);
    }

    pub fn trigger_keyboard(&mut self, event: KeyboardEvent) {
        self.triggered_keyboard.push(event);
    }
}

pub trait CanvasAppTrait {
    fn new(ctx: &mut CanvasContext, width: u32, height: u32) -> impl std::future::Future<Output = Self> where Self: Sized;

    fn on_resize(&mut self, ctx: &mut CanvasContext, width: u32, height: u32);
    fn on_tick(&mut self, ctx: &mut CanvasContext);
    fn on_mouse(&mut self, ctx: &mut CanvasContext, event: MouseEvent);
    fn on_keyboard(&mut self, ctx: &mut CanvasContext, event: KeyboardEvent);
}

pub struct CanvasApp<A: CanvasAppTrait> {
    context: CanvasContext,
    canvas: Canvas,
    app: A,
    time: Instant
}

// #[cfg(any(target_os = "macos", target_os = "ios"))]
// extern "C" {
//     fn get_application_support_dir() -> *const std::os::raw::c_char;
// }

// #[cfg(any(target_os = "macos", target_os = "ios"))]
// fn get_app_support_path() -> Option<String> {
//     unsafe {
//         let ptr = get_application_support_dir();
//         if ptr.is_null() {
//             println!("COULD NOT GET APPLICATION DIRECTORY");
//             return None;
//         }
//         let c_str = std::ffi::CStr::from_ptr(ptr);
//         Some(c_str.to_string_lossy().into_owned())
//     }
// }

//  #[cfg(not(target_os = "macos"))]
//  fn get_app_support_path() -> Option<String> {
//      unsafe {
//          let ptr = get_application_support_dir();
//          if ptr.is_null() {
//              println!("COULD NOT GET APPLICATION DIRECTORY");
//              return None;
//          }
//          let c_str = std::ffi::CStr::from_ptr(ptr);
//          Some(c_str.to_string_lossy().into_owned())
//      }
//  }

impl<A: CanvasAppTrait> WinitAppTrait for CanvasApp<A> {
    async fn new(window: WinitWindow, scheduler: Scheduler, width: u32, height: u32, scale_factor: f64) -> Self {
        let mut canvas = Canvas::new(window).await;
        let (width, height) = canvas.resize(width, height);

        // #[cfg(any(target_os = "macos", target_os = "ios"))]
        // let path = if let Some(new_path) = get_app_support_path() {
        //     println!("GOT NEW PATH");
        //     new_path
        // } else {
        //     panic!("COULD NOT GET NEW PATH");
        //     "test_dir".to_string()
        // };

        let mut context = CanvasContext{
            components: Vec::new(),
            scheduler,
            atlas: CanvasAtlas::default(),
            size: Size::new(width, height, scale_factor),
            state: State::default(),
            triggered_keyboard: Vec::new()
        };
        let app = A::new(&mut context, width, height).await;

        CanvasApp{
            context,
            canvas,
            app,
            time: Instant::now()
        }
    }

    fn on_resumed(&mut self, window: WinitWindow) {
        self.canvas.resumed(window);
    }

    fn on_resize(&mut self, width: u32, height: u32, scale_factor: f64) {
        let (width, height) = self.canvas.resize(width, height);
        let size = Size::new(width, height, scale_factor);
        self.app.on_resize(&mut self.context, size.logical().0, size.logical().1);
        self.context.size = size;
    }

    fn prepare(&mut self) {
        //std::thread::sleep(std::time::Duration::from_secs(1));
        self.context.triggered_keyboard.drain(..).collect::<Vec<_>>().into_iter().for_each(|event|
            self.app.on_keyboard(&mut self.context, event)
        );
        self.app.on_tick(&mut self.context);
        let items = self.context.components.drain(..).collect();

        self.canvas.prepare(&mut self.context.atlas, items);
    }

    fn render(&mut self) {
        self.canvas.render();
        //log::error!("last_frame: {:?}", self.time.elapsed());
        self.time = Instant::now();
        //println!("FREEZE");
        //std::thread::sleep(std::time::Duration::from_secs(1000));
    }

    fn process_callback(&mut self, callback: Callback) {
        callback(&mut self.context.state);
    }

    fn on_mouse(&mut self, mut event: MouseEvent) {
        event.position = (
            self.context.size.scale_logical(event.position.0),
            self.context.size.scale_logical(event.position.1)
        );
        self.app.on_mouse(&mut self.context, event)
    }
    fn on_keyboard(&mut self, event: KeyboardEvent) {
        self.app.on_keyboard(&mut self.context, event)
    }
}

#[macro_export]
macro_rules! create_canvas_entry_points {
    ($app:ty) => {
        create_winit_entry_points!(CanvasApp::<$app>);
    };
}
