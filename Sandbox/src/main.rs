use rotten::core::Application;
use rotten::core::event::KeyPressedEvent;

fn main() {
    let app = Application{};

    let event = KeyPressedEvent::new(42, 0);

    println!("name: {}", event.name());

    app.run();
}