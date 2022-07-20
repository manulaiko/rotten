pub enum EventType {
    NONE,

    WINDOW_CLOSE,
    WINDOW_RESIZE,
    WINDOW_FOCUS,
    WINDOW_LOST_FOCUS,
    WINDOW_MOVED,

    APP_TICK,
    APP_UPDATE,
    APP_RENDER,

    KEY_PRESSED,
    KEY_RELEASED,

    MOUSE_BUTTON_PRESSED,
    MOUSE_BUTTON_RELEASED,
    MOUSE_MOVE,
    MOUSE_SCROLLED,
}

pub enum EventCategory {
    NONE = 0,

    APPLICATION = 1 << 1,
    INPUT = 1 << 2,
    KEYBOARD = 1 << 3,
    MOUSE = 1 << 4,
    MOUSE_BUTTON = 1 << 5
}

pub struct Event {
    pub eventType: EventType,
    pub eventCategory: EventCategory,
    pub name: str,
    handled: bool
}

impl Event {
    pub fn isInCategory(&self, category: &EventCategory) -> bool {
        return self.eventCategory & category;
    }
}