use crate::utils::*;
use crate::{console::Console, editor::Editor};
use std::panic::catch_unwind;
use web_sys::HtmlTextAreaElement;
use weed_lexer::tokenize;
use yew::prelude::*;
use yew::{events::Event, TargetCast};

pub struct App {
	input: String,
	output: String,
}

pub enum Msg {
	Run,
	ClearConsole,
	SetInput(String),
}

impl Component for App {
	type Message = Msg;
	type Properties = ();

	fn create(_ctx: &Context<Self>) -> Self {
		Self {
			input: String::from("burn fire\\/[\n\tconsole_output\\\"hello world\"/;\n]"),
			output: String::from("Output"),
		}
	}

	fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
		match msg {
			Msg::Run => {
				let tokens = catch_unwind(|| tokenize(self.input.chars()));

				self.output = "Output\nCompiling...\n\nToken list\n\n".to_string();

				match tokens {
					Ok(toks) => {
						for t in toks {
							self.output.push_str(&format!("Token, {:?}\n", t));
						}
					}
					Err(e) => self.output.push_str(&format!("\n{:?}", e)),
				}
				true
			}
			Msg::SetInput(input) => {
				self.input = input;
				true
			}
			Msg::ClearConsole => {
				self.output = String::from("Output");
				true
			}
		}
	}

	fn view(&self, ctx: &Context<Self>) -> Html {
		let on_click_run = ctx.link().callback(|_| Msg::Run);
		let on_click_clear = ctx.link().callback(|_| Msg::ClearConsole);

		let set_input = ctx.link().callback(|e: Event| {
			let input = e.target_dyn_into::<HtmlTextAreaElement>();

			Msg::SetInput(input.expect("").value())
		});

		let ctn_style = parser(new_style(
			"div",
			"
				display: flex;
				width: 100%;
				height: 90vh;

			",
		));

		let btn_ctn = parser(new_style(
			"div",
			"
				padding: 0.5rem;
			",
		));

		let mut btn_style = parser(new_style(
			"button",
			"
				border-radius: 0px;
				margin-left: 2px;
				margin-right: 2px;
			",
		));
		btn_style.push("button");
		btn_style.push("is-large");

		html! {
			<div>
				<div class={btn_ctn}>
					<button class={btn_style.clone()} onclick={on_click_run}>
						<span>{"run"}</span>
					</button>
					<button class={btn_style} onclick={on_click_clear}>
						<span>{"clear console"}</span>
					</button>
				</div>
				<div class={ctn_style}>
					<Editor input={self.input.clone()} set_input={set_input} />
					<Console output={self.output.clone()} />
				</div>
			</div>
		}
	}
}


