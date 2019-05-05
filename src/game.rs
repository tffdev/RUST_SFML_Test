extern crate sfml;

use sfml::graphics::*;
use sfml::window::*;

pub fn run() {
	let mut window = create_window();

	// Prepare cat image
	let img = Texture::from_file("test.jpg").unwrap();
	let mut sprite = Sprite::new();
	sprite.set_texture(&img, false);
	sprite.set_position((50.0,50.0));

	// Impact text
	let font = Font::from_file("impact.ttf").unwrap();
    let string = String::from("Funny kitten eat spaghetti\n\n\n\n\n\n\nBottom text");
    let mut text = Text::new(&string.to_uppercase(), &font, 44);
    text.set_fill_color(&Color::WHITE);
    text.set_outline_color(&Color::BLACK);
    text.set_outline_thickness(3.0);
    text.set_position((30.0, 30.0));

    // Event / Draw loop
	while window.is_open() {
		while let Some(event) = window.poll_event() {
			match event { Event::Closed | Event::KeyPressed { code: Key::Escape, .. } 
            	=> window.close(),
                _ => {}
            }
		}
		window.clear(&Color::BLACK);
		window.draw(&sprite);
		window.draw(&text);
		// Draw more stuff here
		window.display();
	}
}

fn create_window() -> RenderWindow {
	return RenderWindow::new((800,600), "Rust SFML Image Test", 
		Style::DEFAULT, &Default::default());
}