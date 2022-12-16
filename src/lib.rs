#![feature(async_closure)]

pub mod markdown;

use std::rc::Rc;

use log::info;
use yew::prelude::*;
use yew_agent::{Bridge, Bridged};
use yew_router::prelude::*;

use crate::markdown::{frame::MarkdownFrame, Markdown, MarkdownInput, MarkdownOutput};

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/markdown")]
    Markdown,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! {
            <div class="header-container">
                <header>{ "Work In Progress" }</header>
                <small>{ "Probably forever..." }</small>
            </div>
        },
        Route::Markdown => html! {
            <MarkdownFrame />
        },
        Route::NotFound => html! {
            <div class="header-container">
                <header>{ "404" }</header>
                <h2>{ "Page not found!" }</h2>
            </div>
        },
    }
}

pub struct App {
    markdown: Box<dyn Bridge<Markdown>>,
    n: u32,
}

pub enum AppMessage {
    MarkdownWorker(MarkdownOutput),
}

impl Component for App {
    type Message = AppMessage;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let cb = {
            let link = ctx.link().clone();
            move |e| link.send_message(AppMessage::MarkdownWorker(e))
        };
        let mut markdown = Markdown::bridge(Rc::new(cb));
        markdown.send(MarkdownInput::N(5));

        Self { markdown, n: 6 }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            AppMessage::MarkdownWorker(msg) => match msg {
                MarkdownOutput::Value(v) => {
                    info!("Value: {v}")
                }
                _ => {}
            },
        }

        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="page-container">
                <BrowserRouter>
                    <Switch<Route> render={switch} /> // <- must be child of <BrowserRouter>
                </BrowserRouter>
            </div>
        }
    }
}
