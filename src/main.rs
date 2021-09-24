use ggez::{conf, event, Context, GameResult};
use std::fs::File;
use std::io::Read;
use std::path::Path;
use fontdue::{Font, FontResult};

// This struct will hold all our game state
// For now there is nothing to be held, but we'll add
// things shortly.
struct Game {}

// This is the main event loop. ggez tells us to implement
// two things:
// - updating
// - rendering
impl event::EventHandler for Game {
    fn update(&mut self, _context: &mut Context) -> GameResult {
        // TODO: update game logic here
        Ok(())
    }

    fn draw(&mut self, _context: &mut Context) -> GameResult {
        // TODO: update draw here
        Ok(())
    }
}

fn load_file (path: &str) -> Vec<u8> {
    let file_path = Path::new(path);

    let mut file = match File::open(file_path) {
        Err(why) => {
            panic!("couldn't open {}: {}", file_path.display(), why)
        },
        Ok(file) => {file}
    };

    let mut buffer = Vec::new();

    // read the whole file
    let file_size = match file.read_to_end(&mut buffer) {
        Err(why) => {
            panic!("couldn't read_to_end {}: {}", file_path.display(), why)
        }
        Ok(size) => {size}
    };

    println!("read {} bytes from {}", file_size, file_path.display());
    buffer
}

fn ttf_font (buffer: Vec<u8>) -> Font {
    let font = Font::from_bytes(buffer, fontdue::FontSettings::default());
    match font {
        Ok(font) => {font}
        Err(why) => {
            panic!("ttf_font error: {}", why)
        }
    }
}

// pub fn main() -> GameResult {
pub fn main() {
    let buffer = load_file("./resources/LiberationMono-Regular.ttf");
    let font = ttf_font(buffer);
    println!("glyph_count {}", font.glyph_count());
    println!("scale_factor {}", font.scale_factor(8.0));
    /*
    // Create a game context and event loop
    let context_builder = ggez::ContextBuilder::new("rust_sokoban", "sokoban")
        .window_setup(conf::WindowSetup::default().title("Rust Sokoban!"))
        .window_mode(conf::WindowMode::default().dimensions(800.0, 600.0))
        .add_resource_path(path::PathBuf::from("./resources"));

    let (context, event_loop) = &mut context_builder.build()?;
    // Create the game state
    let game = &mut Game {};
    // Run the main event loop
    event::run(context, event_loop, game)
    */
}
