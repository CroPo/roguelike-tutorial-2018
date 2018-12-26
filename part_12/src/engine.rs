use std::cell::RefCell;

use tcod::console::{Console, Root};

use game::{Game, state::GameState};
use render::render_all;
use savegame;
use settings::Settings;

pub enum EngineAction {
    ToggleFullscreen,
    MousePos(i32, i32),
    StartGame(bool),
    QuitGame(bool),
    CreateNextFloor,
    Exit,
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

        Engine {
            game: RefCell::new(Game::new()),
            settings,
            root_console: RefCell::new(root_console),
            state: GameState::MainMenu,
            mouse_pos: (0, 0),
        }
    }

    fn start_game(&self, game: &mut Game , load_game: bool) {
        if load_game {
            match savegame::load() {
                Some(game_json) => {
                    game.load(&self.settings, game_json);
                    return;
                }
                None => ()
            };
        }
        game.start_new(&self.settings);
    }

    fn game_loop(&mut self) {

        let mut game = self.game.borrow_mut();

        'game_loop: while !self.root_console.borrow().window_closed() {

            render_all(&self, &game);

            let result = self.state.run(&self, &game);

            if let Some(engine_action) = result.engine_action {
                match engine_action {
                    EngineAction::Exit => {
                        break 'game_loop;
                    }
                    EngineAction::QuitGame(save) => {
                        if save {
                            savegame::save(&game);
                        } else {
                            savegame::delete();
                        }
                    }
                    EngineAction::StartGame(load_game) => {
                        self.root_console.borrow_mut().clear();
                        self.start_game(&mut game, load_game);
                    }
                    EngineAction::ToggleFullscreen => {
                        let fullscreen = self.root_console.borrow().is_fullscreen();
                        self.root_console.borrow_mut().set_fullscreen(!fullscreen)
                    }
                    EngineAction::MousePos(x, y) => {
                        self.mouse_pos = (x as i32, y as i32);
                    }
                    EngineAction::CreateNextFloor => {
                        self.root_console.borrow_mut().clear();
                        game.next_floor(&self.settings);
                    }
                }
            }

            self.state = result.next_state;
        }
    }
}
