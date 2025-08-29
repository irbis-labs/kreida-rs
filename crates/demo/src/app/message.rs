use derive_more::From;

#[derive(Clone, Copy)]
pub enum Fun {
    Sinusoid1,
    Sinusoid2,
    Lines,
    Spirograph,
    Wave,
}

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
    Select(Fun),
    Start,
}
