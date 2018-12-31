use tcod::map::FovAlgorithm;
use tcod::FontLayout;
use tcod::FontType;

pub struct Settings {
    screen_width: i32,
    screen_height: i32,

    title: String,

    font_path: String,
    font_layout: FontLayout,
    font_type: FontType,

    bar_width: i32,
    panel_height: i32,

    message_x_offset: i32,
    message_y_offset: i32,

    map_width: i32,
    map_height: i32,

    room_max_size: i32,
    room_min_size: i32,
    max_rooms: i32,

    fov_algorithm: FovAlgorithm,
    fov_light_walls: bool,
    fov_radius: i32,

    ai_radius: i32,

    max_monsters_per_room: i32,
    max_items_per_room: i32,
}

impl Settings {
    pub fn new() -> Settings {
        Settings {
            screen_width: 80,
            screen_height: 50,
            title: "/r/roguelikedev Tutorial Part 12: Monster and Item Progression".to_string(),
            font_path: "arial10x10.png".to_string(),
            font_layout: FontLayout::Tcod,
            font_type: FontType::Greyscale,
            bar_width: 20,
            panel_height: 7,
            message_x_offset: 2,
            message_y_offset: 1,
            map_width: 80,
            map_height: 43,
            room_max_size: 10,
            room_min_size: 6,
            max_rooms: 30,
            fov_algorithm: FovAlgorithm::Basic,
            fov_light_walls: true,
            fov_radius: 10,
            ai_radius: 12,
            max_monsters_per_room: 3,
            max_items_per_room: 2,
        }
    }

    pub fn screen_width(&self) -> i32 {
        self.screen_width
    }

    pub fn screen_height(&self) -> i32 {
        self.screen_height
    }

    pub fn title(&self) -> String {
        self.title.clone()
    }

    pub fn font_path(&self) -> String {
        self.font_path.clone()
    }

    pub fn font_layout(&self) -> FontLayout {
        self.font_layout
    }

    pub fn font_type(&self) -> FontType {
        self.font_type
    }

    pub fn bar_width(&self) -> i32 {
        self.bar_width
    }

    pub fn panel_height(&self) -> i32 {
        self.panel_height
    }

    pub fn panel_y(&self) -> i32 {
        self.screen_height - self.panel_height
    }

    pub fn panel_pos(&self) -> (i32, i32) {
        (0, self.panel_y())
    }

    pub fn message_width(&self) -> i32 {
        self.screen_width - self.bar_width - self.message_x_offset
    }

    pub fn message_height(&self) -> i32 {
        self.panel_height - self.message_y_offset
    }

    pub fn message_dimensions(&self) -> (i32, i32) {
        (self.message_width(), self.message_height())
    }

    pub fn message_x(&self) -> i32 {
        self.bar_width + self.message_x_offset
    }

    pub fn message_pos(&self) -> (i32, i32) {
        (self.message_x(), self.message_y_offset)
    }

    pub fn map_width(&self) -> i32 {
        self.map_width
    }

    pub fn map_height(&self) -> i32 {
        self.map_height
    }

    pub fn room_max_size(&self) -> i32 {
        self.room_max_size
    }

    pub fn room_min_size(&self) -> i32 {
        self.room_min_size
    }

    pub fn max_rooms(&self) -> i32 {
        self.max_rooms
    }

    pub fn fov_algorithm(&self) -> FovAlgorithm {
        self.fov_algorithm
    }

    pub fn fov_light_walls(&self) -> bool {
        self.fov_light_walls
    }

    pub fn fov_radius(&self) -> i32 {
        self.fov_radius
    }

    pub fn ai_radius(&self) -> i32 {
        self.ai_radius
    }

    pub fn max_monsters_per_room(&self) -> i32 {
        self.max_monsters_per_room
    }

    pub fn max_items_per_room(&self) -> i32 {
        self.max_items_per_room
    }
}