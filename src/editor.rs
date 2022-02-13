use crate::utils::*;
use yew::events::Event;
use yew::prelude::*;

pub struct Editor;

#[derive(Properties, PartialEq)]
pub struct Props {
	pub input: String,
	pub set_input: Callback<Event>,
}

pub enum Msg {
	SetInput(Event),
}

impl Component for Editor {
	type Message = Msg;
	type Properties = Props;

	fn create(_ctx: &Context<Self>) -> Self {
		Self {}
	}

	fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
		match msg {
			Msg::SetInput(event) => {
				ctx.props().set_input.emit(event);
				true
			}
		}
	}

	fn view(&self, ctx: &Context<Self>) -> Html {
		let ctn_style = parser(new_style(
			"div",
			"
				width: 100%;
				margin: 0.1rem;
				padding: 0.1rem;
			",
		));

		let textarea_style = parser(new_style(
			"div",
			"
				width: 100%;
				height: 100%;
				resize: none;
			",
		));

		let on_input = ctx.link().callback(|e: Event| Msg::SetInput(e));

		html! {
			<div class={ctn_style} >
				<textarea class={textarea_style} value={ctx.props().input.clone()} onchange={on_input} />
			</div>
		}
	}
}
