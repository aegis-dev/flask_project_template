use flask::input::Input;
use flask::palette::FlaskColor;
use flask::scene::Scene;
use flask::renderer::Renderer;
use flask::game_status::GameStatus;
use flask::font::Font;

pub struct ExampleScene {
    font: Font,
}

impl ExampleScene {
    pub fn new() -> ExampleScene {
        ExampleScene {
            font: Font::load_3x5().unwrap()
        }
    }
}

impl Scene for ExampleScene {
    fn on_start(&mut self, renderer: &mut Renderer) {
        renderer.set_background_color(FlaskColor::Teal as u8).unwrap();
    }

    fn on_update(&mut self, game_status: &mut GameStatus, renderer: &mut Renderer, input: &Input, _delta_time: f64) -> Option<Box<dyn Scene>> {

        None
    }

    fn on_destroy(&mut self) {

    }
}
