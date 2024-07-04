mod scene;
mod shader;

use cgmath::Vector4;
use glfw::{
    fail_on_errors, Action, Context, Glfw, GlfwReceiver, Key, OpenGlProfileHint, PWindow,
    WindowEvent,
};
use scene::Scene;
use shader::Program;

pub struct Application {
    glfw_api: Glfw,
    window: PWindow,
    events: GlfwReceiver<(f64, WindowEvent)>,
    parameters: Parameters,

    scene: Scene,
    test_program: Program,
}

struct Parameters {
    pub gl_version_major: u32,
    pub gl_version_minor: u32,
    pub gl_profile: OpenGlProfileHint,
    pub window_width: u32,
    pub window_height: u32,
    pub window_title: String,
    pub background_color: Vector4<f32>,
}

impl Default for Parameters {
    fn default() -> Self {
        Self {
            gl_version_major: 4,
            gl_version_minor: 6,
            gl_profile: glfw::OpenGlProfileHint::Core,
            window_width: 1280,
            window_height: 720,
            window_title: String::from("rugl"),
            background_color: Vector4 {
                x: 0.2,
                y: 0.3,
                z: 0.3,
                w: 1.,
            },
        }
    }
}

impl Application {
    pub fn new() -> Self {
        let app_params: Parameters = Default::default();
        let mut glfw_api = Self::init_glfw(&app_params);
        let (mut window, events) = Self::init_window(&mut glfw_api, &app_params);

        Self::init_opengl(&mut window);

        let test_program = Program::graphics(
            &format!("{}/vert.glsl", shader::SHADER_DIR),
            &format!("{}/frag.glsl", shader::SHADER_DIR),
        );
        let test_scene = Scene::new(&test_program);

        Application {
            glfw_api: glfw_api,
            window: window,
            events: events,
            parameters: app_params,
            test_program: test_program,
            scene: test_scene,
        }
    }

    pub fn run(&mut self) {
        self.center_window();

        while !self.window.should_close() {
            self.process_events();
            self.render();
            self.poll_events();
        }
    }

    fn render(&mut self) {
        unsafe {
            gl::ClearColor(
                self.parameters.background_color.x,
                self.parameters.background_color.y,
                self.parameters.background_color.z,
                self.parameters.background_color.w,
            );
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
        self.scene.render(&self.test_program);
        self.window.swap_buffers();
    }

    fn poll_events(&mut self) {
        self.glfw_api.poll_events();
    }

    fn process_events(&mut self) {
        for (_, event) in glfw::flush_messages(&self.events) {
            match event {
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    self.window.set_should_close(true)
                }
                _ => {}
            }
        }
    }
}

// Initializers
impl Application {
    fn init_glfw(parameters: &Parameters) -> Glfw {
        let mut glfw_api = glfw::init(fail_on_errors!())
            .unwrap_or_else(|err| panic!("Failed to initialize glfw: {:?}\n", err));

        glfw_api.window_hint(glfw::WindowHint::ContextVersion(
            parameters.gl_version_major,
            parameters.gl_version_minor,
        ));
        glfw_api.window_hint(glfw::WindowHint::OpenGlProfile(parameters.gl_profile));
        glfw_api.window_hint(glfw::WindowHint::Visible(true));
        glfw_api.window_hint(glfw::WindowHint::Resizable(false));
        glfw_api.window_hint(glfw::WindowHint::DoubleBuffer(true));

        glfw_api
    }

    fn init_opengl(window: &mut PWindow) {
        gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);
    }

    fn init_window(
        glfw_api: &mut Glfw,
        parameters: &Parameters,
    ) -> (PWindow, GlfwReceiver<(f64, WindowEvent)>) {
        let (mut window, events) = glfw_api
            .create_window(
                parameters.window_width,
                parameters.window_height,
                &parameters.window_title,
                glfw::WindowMode::Windowed,
            )
            .expect("Failed to create the window!\n");

        // Make the window's context current
        window.set_key_polling(true);
        window.make_current();

        (window, events)
    }
}

impl Application {
    fn center_window(&mut self) {
        // Center the window
        let monitor_width;
        let monitor_height;
        unsafe {
            let monitor = glfw::ffi::glfwGetPrimaryMonitor();
            assert!(!monitor.is_null(), "Failed to get primary monitor!\n");
            let video_mode = glfw::ffi::glfwGetVideoMode(monitor);
            assert!(!video_mode.is_null(), "Failed to get the video mode!\n");
            monitor_width = (*video_mode).width as i32;
            monitor_height = (*video_mode).height as i32;
        }
        let window_x = (monitor_width - self.parameters.window_width as i32) / 2;
        let window_y = (monitor_height - self.parameters.window_height as i32) / 2;
        self.window.set_pos(window_x, window_y);
    }
}
