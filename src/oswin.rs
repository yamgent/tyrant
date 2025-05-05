use std::{collections::HashMap, num::NonZeroUsize, sync::Arc};

use vello::{
    AaConfig, AaSupport, RenderParams, Renderer, RendererOptions, Scene,
    util::{RenderContext, RenderSurface},
    wgpu::{Maintain, PresentMode},
};
use winit::{
    dpi::PhysicalSize,
    event_loop::ActiveEventLoop,
    window::{Window, WindowId},
};

use crate::core::Core;

pub struct OswinManager {
    render_context: RenderContext,
    renderers: HashMap<usize, Renderer>,
    oswins: HashMap<WindowId, Oswin>,
}

impl OswinManager {
    pub fn new(event_loop: &ActiveEventLoop) -> Self {
        // at start, there will only be one main window
        let main_oswin = Oswin::new(event_loop);

        let mut oswins = HashMap::new();
        oswins.insert(main_oswin.id(), main_oswin);

        Self {
            render_context: RenderContext::new(),
            renderers: HashMap::new(),
            oswins,
        }
    }

    pub fn resume(&mut self) {
        self.oswins.values_mut().for_each(|oswin| {
            oswin.resume(&mut self.render_context, &mut self.renderers);
        });
    }

    pub fn suspend(&mut self) {
        self.oswins.values_mut().for_each(|oswin| oswin.suspend());
    }

    pub fn close(&mut self, window_id: WindowId, event_loop: &ActiveEventLoop) {
        let Some(oswin) = self.oswins.remove(&window_id) else {
            return;
        };

        oswin.close();

        if self.oswins.is_empty() {
            event_loop.exit();
        }
    }

    pub fn resize(&mut self, window_id: WindowId, size: PhysicalSize<u32>) {
        if let Some(oswin) = self.oswins.get_mut(&window_id) {
            oswin.resize(&mut self.render_context, size);
        }
    }

    pub fn redraw(&mut self, window_id: WindowId, core: &Core) {
        if let Some(oswin) = self.oswins.get_mut(&window_id) {
            oswin.redraw(&self.render_context, &mut self.renderers, core);
        }
    }
}

struct Oswin {
    state: OswinState,
    window: Arc<Window>,
    scene: Scene,
}

enum OswinState {
    Active { surface: RenderSurface<'static> },
    Suspended,
}

impl Oswin {
    fn new(event_loop: &ActiveEventLoop) -> Self {
        Self {
            state: OswinState::Suspended,
            window: Arc::new(
                event_loop
                    .create_window(Window::default_attributes())
                    .expect("can create window"),
            ),
            scene: Scene::new(),
        }
    }

    fn resume(
        &mut self,
        render_context: &mut RenderContext,
        renderers: &mut HashMap<usize, Renderer>,
    ) {
        if matches!(self.state, OswinState::Active { .. }) {
            return;
        }

        let size = self.window.inner_size();
        let surface_future = render_context.create_surface(
            self.window.clone(),
            size.width,
            size.height,
            PresentMode::AutoVsync,
        );
        let surface = pollster::block_on(surface_future).expect("can create surface");

        renderers.entry(surface.dev_id).or_insert_with(|| {
            Renderer::new(
                &render_context.devices[surface.dev_id].device,
                RendererOptions {
                    surface_format: Some(surface.format),
                    use_cpu: false,
                    antialiasing_support: AaSupport::all(),
                    num_init_threads: NonZeroUsize::new(1),
                },
            )
            .expect("can create renderer")
        });

        self.state = OswinState::Active { surface };
    }

    fn suspend(&mut self) {
        if matches!(self.state, OswinState::Suspended) {
            return;
        }

        self.state = OswinState::Suspended;
    }

    fn id(&self) -> WindowId {
        self.window.id()
    }

    fn close(self) {
        // we close the window by dropping ourselves
        //
        // note that this won't work if there are dangling references to
        // self.window (since our self.window is backed by a Arc). So make
        // sure not to leak self.window to outside objects by accident
        drop(self);
    }

    fn resize(&mut self, render_context: &RenderContext, size: PhysicalSize<u32>) {
        let OswinState::Active { surface } = &mut self.state else {
            return;
        };

        render_context.resize_surface(surface, size.width, size.height);
    }

    fn redraw(
        &mut self,
        render_context: &RenderContext,
        renderers: &mut HashMap<usize, Renderer>,
        core: &Core,
    ) {
        let OswinState::Active { surface } = &mut self.state else {
            return;
        };

        // instead of re-creating scene every frame, we just
        // reset the same scene to save memory allocation
        self.scene.reset();

        core.render(&mut self.scene);

        let width = surface.config.width;
        let height = surface.config.height;
        let device_handle = &render_context.devices[surface.dev_id];
        let surface_texture = surface
            .surface
            .get_current_texture()
            .expect("can get surface texture");

        renderers
            .get_mut(&surface.dev_id)
            .expect("have valid renderer")
            .render_to_surface(
                &device_handle.device,
                &device_handle.queue,
                &self.scene,
                &surface_texture,
                &RenderParams {
                    base_color: vello::peniko::color::palette::css::BLACK,
                    width,
                    height,
                    antialiasing_method: AaConfig::Msaa16,
                },
            )
            .expect("can render to surface");

        surface_texture.present();

        device_handle.device.poll(Maintain::Poll);
    }
}
