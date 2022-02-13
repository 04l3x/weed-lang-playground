use crate::utils::*;
use yew::prelude::*;
use yew::Properties;

pub struct Console;

pub enum Msg {}

#[derive(Properties, PartialEq)]
pub struct Props {
	pub output: String,
}

impl Component for Console {
	type Message = Msg;
	type Properties = Props;

	fn create(_ctx: &Context<Self>) -> Self {
		Self {}
	}

	fn view(&self, ctx: &Context<Self>) -> Html {
		let output = ctx.props().output.clone();

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

		html! {
			<div class={ctn_style}>
				<textArea class={textarea_style} readonly={true} value={output} />
			</div>
		}
	}
}
