
mod state;
use state::State;
use winit::{application::ApplicationHandler, event::WindowEvent, event_loop::{ActiveEventLoop, EventLoop}, window::{Window, WindowId}};


struct StateApplication<'a> {
    state: Option<State<'a>>,
}

impl<'a> StateApplication<'a> {
    pub fn new() -> Self {
        Self {
            state: None,
        }
    }
}

impl<'a> ApplicationHandler for StateApplication<'a>{
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = event_loop.create_window(Window::default_attributes().with_title("Hello!")).unwrap();
        self.state = Some(State::new(window));
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, window_id: WindowId, event: WindowEvent) {
        let window = self.state.as_ref().unwrap().window();

        if window.id() == window_id {
            match event {
                WindowEvent::CloseRequested => {
                    event_loop.exit();
                }
                WindowEvent::Resized(physical_size) => {
                    self.state.as_mut().unwrap().resize(physical_size);
                }
                WindowEvent::RedrawRequested => {
                    self.state.as_mut().unwrap().render().unwrap();
                }
                _ => {}
            }
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        let window = self.state.as_ref().unwrap().window();
        window.request_redraw();
    }
}


pub async fn run() {
    let event_loop = EventLoop::new().unwrap();

    let mut window_state = StateApplication::new();
    let _ = event_loop.run_app(&mut window_state);
}