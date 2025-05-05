use anyhow::Result;
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::{Window, WindowId},
};

pub struct App {
    app_data: Option<AppData>,
}

impl App {
    fn new() -> Self {
        Self { app_data: None }
    }

    pub fn run_app() -> Result<()> {
        let event_loop = EventLoop::new()?;
        event_loop.set_control_flow(ControlFlow::Wait);

        let mut app = App::new();
        event_loop.run_app(&mut app)?;

        Ok(())
    }
}

struct AppData {
    window: Window,
}

impl AppData {
    fn new(event_loop: &ActiveEventLoop) -> Self {
        Self {
            window: event_loop
                .create_window(Window::default_attributes())
                .expect("can create window"),
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.app_data.is_none() {
            self.app_data = Some(AppData::new(event_loop))
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        let app_data = match self.app_data {
            Some(ref mut app_data) => app_data,
            None => return,
        };

        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                if app_data.window.id() == window_id {
                    app_data.window.request_redraw();
                }
            }
            _ => {}
        }
    }
}
