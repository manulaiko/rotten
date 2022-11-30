use rotten::core::Application;
use rotten::core::event::{EventTrait, KeyPressedEvent};

fn main() {
    let app = Application{};

    let event = KeyPressedEvent::new(42, 0);

    println!("name: {}, code: {}", event.name(), event.code());

    app.run();
}