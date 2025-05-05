use anyhow::Result;
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::WindowId,
};

use crate::{core::Core, oswin::OswinManager};

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
    core: Core,
    oswin_manager: OswinManager,
}

impl AppData {
    fn new(event_loop: &ActiveEventLoop) -> Self {
        Self {
            core: Core { dummy: 6.0 },
            oswin_manager: OswinManager::new(event_loop),
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let mut app_data = self
            .app_data
            .take()
            .unwrap_or_else(|| AppData::new(event_loop));

        app_data.oswin_manager.resume();

        self.app_data = Some(app_data);
    }

    fn suspended(&mut self, event_loop: &ActiveEventLoop) {
        let mut app_data = self
            .app_data
            .take()
            .unwrap_or_else(|| AppData::new(event_loop));

        app_data.oswin_manager.suspend();

        self.app_data = Some(app_data);
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        let Some(app_data) = &mut self.app_data else {
            return;
        };

        match event {
            WindowEvent::CloseRequested => {
                app_data.oswin_manager.close(window_id, event_loop);
            }
            WindowEvent::Resized(size) => {
                app_data.oswin_manager.resize(window_id, size);
            }
            WindowEvent::RedrawRequested => {
                app_data.oswin_manager.redraw(window_id, &app_data.core);
            }
            _ => {}
        }
    }
}
