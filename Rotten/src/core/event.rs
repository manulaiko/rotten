use std::any::Any;

use bitflags::bitflags;

macro_rules! event_functions {
    () => {
        fn in_category(&self, category: &EventCategory) -> bool {
            self.event.in_category(category)
        }

        fn event_type(&self) -> &EventType {
            &self.event.event_type()
        }

        fn event_category(&self) -> &EventCategory {
            &self.event.event_category()
        }

        fn name(&self) -> &str {
            crate::print_type_of(self)
        }
    };
}

macro_rules! code_function {
    () => {
        pub fn code(&self) -> u32 {
            self.event.code()
        }
    }
}

#[derive(Debug)]
pub enum EventType {
    None,

    WindowClose,
    WindowResize,
    WindowFocus,
    WindowLostFocus,
    WindowMoved,

    AppTick,
    AppUpdate,
    AppRender,

    KeyPressed,
    KeyReleased,

    MouseButtonPressed,
    MouseButtonReleased,
    MouseMove,
    MouseScrolled,
}

bitflags! {
    pub struct EventCategory: u16 {
        const NONE = 0x00;

        const APPLICATION = 0x01;
        const INPUT = 0x02;
        const KEYBOARD = 0x04;
        const MOUSE = 0x08;
        const MOUSE_BUTTON = 0x10;
    }
}

pub trait EventTrait {
    fn event_type(&self) -> &EventType;

    fn event_category(&self) -> &EventCategory;

    fn name(&self) -> &str {
        "Event"
    }

    fn in_category(&self, category: &EventCategory) -> bool {
        (*self.event_category() & *category) != EventCategory::NONE
    }
}

pub trait CodeTrait: EventTrait {
    fn code(&self) -> u32;
}

#[derive(Debug)]
pub struct Event {
    handled: bool,
    event_type: EventType,
    event_category: EventCategory,
}

impl Event {
    fn new(event_type: EventType, event_category: EventCategory) -> Event {
        Event {
            handled: false,
            event_type,
            event_category,
        }
    }
}

impl EventTrait for Event {
    fn event_type(&self) -> &EventType {
        &self.event_type
    }

    fn event_category(&self) -> &EventCategory {
        &self.event_category
    }

    fn name(&self) -> &str {
        "Event"
    }

    fn in_category(&self, category: &EventCategory) -> bool {
        (self.event_category & *category) != EventCategory::NONE
    }
}

#[derive(Debug)]
pub struct KeyEvent {
    event: Event,
    code: u32,
}

impl EventTrait for KeyEvent {
    event_functions!();
}

impl KeyEvent {
    pub fn new(code: u32, event_type: EventType) -> KeyEvent {
        let event = Event::new(event_type, EventCategory::KEYBOARD | EventCategory::INPUT);

        KeyEvent {
            event,
            code,
        }
    }

    pub fn code(&self) -> u32 {
        self.code
    }
}

#[derive(Debug)]
pub struct KeyPressedEvent {
    event: KeyEvent,
    repeat_count: u32,
}

impl EventTrait for KeyPressedEvent {
    event_functions!();
}

impl KeyPressedEvent {
    code_function!();

    pub fn new(code: u32, repeat_count: u32) -> KeyPressedEvent {
        let event = KeyEvent::new(code, EventType::KeyPressed);

        KeyPressedEvent {
            event,
            repeat_count,
        }
    }

    pub fn repeat_count(&self) -> u32 {
        self.repeat_count
    }
}

#[derive(Debug)]
pub struct KeyReleasedEvent {
    event: KeyEvent,
}


impl EventTrait for KeyReleasedEvent {
    event_functions!();
}

impl KeyReleasedEvent {
    code_function!();

    pub fn new(code: u32) -> KeyReleasedEvent {
        let event = KeyEvent::new(code, EventType::KeyReleased);

        KeyReleasedEvent {
            event
        }
    }
}

#[derive(Debug)]
pub struct MouseMoveEvent {
    event: Event,
    x: f64,
    y: f64,
}

impl EventTrait for MouseMoveEvent {
    event_functions!();
}

impl MouseMoveEvent {
    pub fn new(x: f64, y: f64) -> MouseMoveEvent {
        let event = Event::new(EventType::MouseMove, EventCategory::INPUT & EventCategory::MOUSE);

        MouseMoveEvent {
            event,
            x,
            y,
        }
    }

    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn y(&self) -> f64 {
        self.y
    }
}

#[derive(Debug)]
pub struct MouseScrolledEvent {
    event: Event,
    x_offset: f64,
    y_offset: f64,
}

impl EventTrait for MouseScrolledEvent {
    event_functions!();
}

impl MouseScrolledEvent {
    pub fn new(x_offset: f64, y_offset: f64) -> MouseScrolledEvent {
        let event = Event::new(EventType::MouseScrolled, EventCategory::INPUT & EventCategory::MOUSE);

        MouseScrolledEvent {
            event,
            x_offset,
            y_offset,
        }
    }
}

#[derive(Debug)]
pub struct MouseButtonEvent {
    event: Event,
    code: u32,
}

impl EventTrait for MouseButtonEvent {
    event_functions!();
}

impl MouseButtonEvent {
    pub fn new(code: u32, event_type: EventType) -> MouseButtonEvent {
        let event = Event::new(event_type, EventCategory::MOUSE_BUTTON | EventCategory::INPUT);

        MouseButtonEvent {
            event,
            code,
        }
    }

    pub fn code(&self) -> u32 {
        self.code
    }
}

#[derive(Debug)]
pub struct MouseButtonPressedEvent {
    event: MouseButtonEvent,
}

impl EventTrait for MouseButtonPressedEvent {
    event_functions!();
}

impl MouseButtonPressedEvent {
    code_function!();

    pub fn new(code: u32) -> MouseButtonPressedEvent {
        let event = MouseButtonEvent::new(code, EventType::MouseButtonPressed);

        MouseButtonPressedEvent {
            event
        }
    }
}

#[derive(Debug)]
pub struct MouseButtonReleasedEvent {
    event: MouseButtonEvent,
}

impl EventTrait for MouseButtonReleasedEvent {
    event_functions!();
}

impl MouseButtonReleasedEvent {
    code_function!();

    pub fn new(code: u32) -> MouseButtonReleasedEvent {
        let event = MouseButtonEvent::new(code, EventType::MouseButtonReleased);

        MouseButtonReleasedEvent {
            event
        }
    }
}

#[derive(Debug)]
pub struct WindowResizeEvent {
    event: Event,
    width: u32,
    height: u32,
}

impl EventTrait for WindowResizeEvent {
    event_functions!();
}

impl WindowResizeEvent {
    fn new(width: u32, height: u32) -> WindowResizeEvent {
        let event = Event::new(EventType::WindowResize, EventCategory::APPLICATION);

        WindowResizeEvent {
            event,
            width,
            height,
        }
    }
}

#[derive(Debug)]
pub struct WindowCloseEvent {
    event: Event,
}

impl EventTrait for WindowCloseEvent {
    event_functions!();
}

impl WindowCloseEvent {
    fn new() -> WindowCloseEvent {
        let event = Event::new(EventType::WindowClose, EventCategory::APPLICATION);

        WindowCloseEvent {
            event
        }
    }
}

#[derive(Debug)]
pub struct AppTickEvent {
    event: Event,
}

impl EventTrait for AppTickEvent {
    event_functions!();
}

impl AppTickEvent {
    fn new() -> AppTickEvent {
        let event = Event::new(EventType::AppTick, EventCategory::APPLICATION);

        AppTickEvent {
            event
        }
    }
}

#[derive(Debug)]
pub struct AppRenderEvent {
    event: Event,
}

impl EventTrait for AppRenderEvent {
    event_functions!();
}

impl AppRenderEvent {
    fn new() -> AppRenderEvent {
        let event = Event::new(EventType::AppRender, EventCategory::APPLICATION);

        AppRenderEvent {
            event
        }
    }
}

#[derive(Debug)]
pub struct AppUpdateEvent {
    event: Event,
}

impl EventTrait for AppUpdateEvent {
    event_functions!();
}

impl AppUpdateEvent {
    fn new() -> AppUpdateEvent {
        let event = Event::new(EventType::AppUpdate, EventCategory::APPLICATION);

        AppUpdateEvent {
            event
        }
    }
}

// TODO
pub trait EventDispatcher {
    fn dispatch<T: EventTrait + 'static>(&self, event: T) {
        match event.event_type() {
            EventType::None => self.event((&event as &dyn Any).downcast_ref::<Event>().unwrap()),
            EventType::WindowClose => self.window_close_event((&event as &dyn Any).downcast_ref::<WindowCloseEvent>().unwrap()),
            EventType::WindowResize => self.window_resize_event((&event as &dyn Any).downcast_ref::<WindowResizeEvent>().unwrap()),
            EventType::WindowFocus => self.event((&event as &dyn Any).downcast_ref::<Event>().unwrap()),
            EventType::WindowLostFocus => self.event((&event as &dyn Any).downcast_ref::<Event>().unwrap()),
            EventType::WindowMoved => self.event((&event as &dyn Any).downcast_ref::<Event>().unwrap()),
            EventType::AppTick => self.app_tick_event((&event as &dyn Any).downcast_ref::<AppTickEvent>().unwrap()),
            EventType::AppUpdate => self.app_update_event((&event as &dyn Any).downcast_ref::<AppUpdateEvent>().unwrap()),
            EventType::AppRender => self.app_render_event((&event as &dyn Any).downcast_ref::<AppRenderEvent>().unwrap()),
            EventType::KeyPressed => self.key_pressed_event((&event as &dyn Any).downcast_ref::<KeyPressedEvent>().unwrap()),
            EventType::KeyReleased => self.key_released_event((&event as &dyn Any).downcast_ref::<KeyReleasedEvent>().unwrap()),
            EventType::MouseButtonPressed => self.mouse_button_pressed_event((&event as &dyn Any).downcast_ref::<MouseButtonPressedEvent>().unwrap()),
            EventType::MouseButtonReleased => self.mouse_button_released_event((&event as &dyn Any).downcast_ref::<MouseButtonReleasedEvent>().unwrap()),
            EventType::MouseMove => self.mouse_move_event((&event as &dyn Any).downcast_ref::<MouseMoveEvent>().unwrap()),
            EventType::MouseScrolled => self.mouse_scrolled_event((&event as &dyn Any).downcast_ref::<MouseScrolledEvent>().unwrap()),
        }
    }

    fn event(&self, event: &Event) {}
    fn key_event(&self, event: &KeyEvent) {}
    fn key_pressed_event(&self, event: &KeyPressedEvent) {}
    fn key_released_event(&self, event: &KeyReleasedEvent) {}
    fn mouse_move_event(&self, event: &MouseMoveEvent) {}
    fn mouse_scrolled_event(&self, event: &MouseScrolledEvent) {}
    fn mouse_button_event(&self, event: &MouseButtonEvent) {}
    fn mouse_button_pressed_event(&self, event: &MouseButtonPressedEvent) {}
    fn mouse_button_released_event(&self, event: &MouseButtonReleasedEvent) {}
    fn window_resize_event(&self, event: &WindowResizeEvent) {}
    fn window_close_event(&self, event: &WindowCloseEvent) {}
    fn app_tick_event(&self, event: &AppTickEvent) {}
    fn app_render_event(&self, event: &AppRenderEvent) {}
    fn app_update_event(&self, event: &AppUpdateEvent) {}
}