#![recursion_limit="256"]
use wasm_bindgen::prelude::*;
use yew::prelude::*;
// use chrono::prelude::*;

struct Counter {
    link: ComponentLink<Self>,
    value: i64,
    local_timezone: i32,
    desired_overlap: u32,
    overlap: u32,
    target_timezone: i32
}

enum Msg {
    AddOne,
    UpdateLocalTime(i32),
    UpdateTargetTimezone(i32),
    UpdateDesiredOverlap(u32)
}

impl Component for Counter {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            value: 0,
            desired_overlap: 8,
            overlap: 0,
            local_timezone: -3,
            target_timezone: -5
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddOne => self.value += 1,
            Msg::UpdateLocalTime(time) => {
                self.local_timezone = time
            },
            Msg::UpdateTargetTimezone(time) => {
                self.target_timezone = time
            },
            Msg::UpdateDesiredOverlap(hours) => {
                if hours < 0 {
                    self.desired_overlap = 0
                } else {
                    self.desired_overlap = hours
                }
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
        let update_local_timezone = self.link.callback(|e: InputData| Msg::UpdateLocalTime(e.value.parse().unwrap()));
        let update_target_timezone = self.link.callback(|e: InputData| Msg::UpdateTargetTimezone(e.value.parse().unwrap()));
        let update_desired_overlap = self.link.callback(|e: InputData| Msg::UpdateDesiredOverlap(e.value.parse().unwrap()));
        html! {
            <div>
                <button onclick=add_value>{ "+1" }</button>
                <p>{ self.value }</p>
                <p>{"Base timezone"}</p>
                <input type="text" placeholder="0" oninput=update_local_timezone value={self.local_timezone}/>
                <p>{"Target timezone"}</p>
                <input type="text" placeholder="0" oninput=update_target_timezone value={self.target_timezone}/>
                <p>{"Desired Overlap (hours)"}</p>
                <input type="number" placeholder="4" oninput=update_desired_overlap value={self.desired_overlap} />
                <p>{"Working works overlap (hours)"}</p>
                <input type="text" placeholder="00:00" readonly=true value={self.overlap}/>
            </div>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Counter>::new().mount_to_body();
}