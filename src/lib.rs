use wasm_bindgen::prelude::*;
use yew::prelude::*;
use chrono::prelude::*;

struct Counter {
    link: ComponentLink<Self>,
    value: i64,
    base_time: DateTime<Local>
}

enum Msg {
    AddOne,
    UpdateBaseTime
}

impl Component for Counter {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            value: 0,
            base_time: Local::now()
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddOne => self.value += 1,
            Msg::UpdateBaseTime => {
                ()
            }
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self) -> Html {
        let add_value = self.link.callback(|_| Msg::AddOne);
        let update_base_time = self.link.callback(|_| Msg::UpdateBaseTime);
        html! {
            <div>
                <button onclick=add_value>{ "+1" }</button>
                <p>{ self.value }</p>
                <p>{"Base time"}</p>
                <input type="text" placeholder="00:00" onchange=update_base_time value={self.base_time.to_string()}/>
                <p>{"Overlap"}</p>
                <input type="number" placeholder="4" />
                <p>{"SF time"}</p>
                <input type="text" placeholder="00:00" />
            </div>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Counter>::new().mount_to_body();
}