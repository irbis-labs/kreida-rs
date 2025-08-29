use yew::html::ImplicitClone;
use yew_router::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Routable)]
pub enum Fun {
    #[at("/")]
    Home,
    #[at("/sinusoid1")]
    Sinusoid1,
    #[at("/sinusoid2")]
    Sinusoid2,
    #[at("/lines")]
    Lines,
    #[at("/spirograph")]
    Spirograph,
    #[at("/wave")]
    Wave,
}

impl ImplicitClone for Fun {}
