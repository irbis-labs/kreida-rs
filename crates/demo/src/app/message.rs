use derive_more::From;

#[derive(From)]
pub enum Msg {
    Command(Cmd),
    Event(Evt),
}

pub enum Evt {
    Fullscreen(bool),
    Resize,
    Step(f64),
}

pub enum Cmd {
    ToggleDark,
    ToggleFullscreen,
    Start,
}
