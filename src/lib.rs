
mod base;
pub use base::*;

//  mod runtime;
//  pub use runtime::*;

pub mod prelude {
    pub use crate::*;
    pub use base::BaseAppTrait as App;
    pub use base::BaseBackgroundAppTrait as BackgroundApp;
    pub use base::BaseContext as Context;
    pub use base::BaseAsyncContext as AsyncContext;
    pub use crate::create_base_entry_points as create_entry_points;
}



//  mod state;
//  pub use state::{State, Field};
//  #[cfg(target_os = "android")]
//  pub use winit::AndroidApp;
//  #[cfg(target_arch = "wasm32")]
//  pub use wasm_bindgen::prelude::*;
//  #[cfg(target_arch = "wasm32")]
//  pub use wasm_bindgen;

//  pub mod canvas;
//  pub use crate::canvas::CanvasApp;

//  #[cfg(feature = "canvas")]
//  pub mod prelude {
//      pub use crate::*;
//      pub use crate::canvas::{MouseEvent, MouseState, KeyboardEvent, KeyboardState, NamedKey, Key};
//      pub use crate::canvas::{CanvasItem, Area, Color, Shape, Text, Image, Font, State, Field};
//      pub use crate::canvas::CanvasAppTrait as App;
//      pub use crate::canvas::CanvasContext as Context;
//      pub use crate::create_canvas_entry_points as create_entry_points;
//  }

//  #[cfg(feature = "components")]
//  mod components;
//  #[cfg(feature = "components")]
//  pub use components::ComponentApp;

//  #[cfg(feature = "components")]
//  pub mod prelude {
//      pub use crate::*;
//      pub use crate::components::{Events, Event, TickEvent, MouseEvent, MouseState, KeyboardEvent, KeyboardState, NamedKey, Key};
//      pub use components::{resources, Plugin, Color, Shape, ShapeType, Image, Text, Layout, Component, Drawable, SizeRequest, State, Field, Area};

//      pub use crate::components::ComponentAppTrait as App;
//      pub use crate::components::ComponentContext as Context;
//      pub use crate::create_component_entry_points as create_entry_points;
//      pub use include_dir;
//      pub use include_dir::include_dir as include_assets;
//      pub use downcast_rs::DowncastSync;

//      pub use proc::{Component, Plugin};
//  }
