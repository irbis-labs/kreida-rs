use yew::prelude::*;
use yew_router::prelude::*;

use crate::app::canvas::CanvasView;
use crate::app::route::Fun;

mod canvas;
mod message;
mod route;

pub struct DemoApp {}

pub enum AppMsg {}

impl Component for DemoApp {
    type Message = AppMsg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        DemoApp {}
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {}
    }

    fn changed(&mut self, _ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <BrowserRouter>
                <Switch<Fun> render={move |route| switch(route)} />
            </BrowserRouter>
        }
    }
}

fn switch(fun: Fun) -> Html {
    html! { <CanvasView {fun} /> }
}
