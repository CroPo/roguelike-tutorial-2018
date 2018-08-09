use std::cell::RefCell;

use tcod::console::Root;

use game::{Game, state::GameState};
use render::render_all;
use savegame;
use settings::Settings;

pub enum EngineAction {
    ToggleFullscreen,
    MousePos(i32, i32),
    Exit(bool),
}

pub struct Engine {
    pub game: RefCell<Game>,
    pub settings: Settings,
    pub root_console: RefCell<Root>,

    pub state: GameState,
    pub mouse_pos: (i32, i32),
}

impl Engine {
    pub fn run() {
        let mut engine = Engine::initialize();

        engine.game_loop();
    }

    fn initialize() -> Self {
        let settings = Settings::new();

        let root_console = Root::initializer()
            .size(settings.screen_width(), settings.screen_height())
            .title(settings.title())
            .font(settings.font_path(), settings.font_layout())
            .font_type(settings.font_type())
            .init();

        let game = match savegame::load(&settings) {
            Some(game) => game,
            None => Game::new(&settings)
        };

        Engine {
            game: RefCell::new(game),
            settings,
            root_console: RefCell::new(root_console),
            state: GameState::PlayersTurn,
            mouse_pos: (0, 0),
        }
    }

    fn game_loop(&mut self) {
        'game_loop: while !self.root_console.borrow().window_closed() {

            let game = self.game.borrow();

            render_all(&self, &game);

            let result = self.state.run(&self, &game);

            if let Some(engine_action) = result.engine_action {
                match engine_action {
                    EngineAction::Exit(save) => {
                        if save {
                            savegame::save(&game);
                        } else {
                            savegame::delete();
                        }
                        break 'game_loop;
                    }
                    EngineAction::ToggleFullscreen => {
                        let fullscreen = self.root_console.borrow().is_fullscreen();
                        self.root_console.borrow_mut().set_fullscreen(!fullscreen)
                    }
                    EngineAction::MousePos(x, y) => {
                        self.mouse_pos = (x as i32, y as i32);
                    }
                }
            }

            self.state = result.next_state;
        }
    }
}
