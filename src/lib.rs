#![recursion_limit="256"]
use wasm_bindgen::prelude::*;
use yew::prelude::*;
// use chrono::prelude::*;

struct Counter {
    link: ComponentLink<Self>,
    local_timezone: i32,
    desired_overlap: u32,
    overlap: i32,
    target_timezone: i32
}

enum Msg {
    UpdateLocalTime(i32),
    UpdateTargetTimezone(i32),
    UpdateDesiredOverlap(u32),
    UpdateOverlap()
}

impl Component for Counter {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            desired_overlap: 8,
            overlap: 0,
            local_timezone: -3,
            target_timezone: -4
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::UpdateLocalTime(time) => {
                self.local_timezone = time;
                // Msg::UpdateOverlap();
                self.overlap = self.target_timezone - self.local_timezone;
            },
            Msg::UpdateOverlap() => {
                self.overlap = self.target_timezone - self.local_timezone;
            },
            Msg::UpdateTargetTimezone(time) => {
                self.target_timezone = time;
                // Msg::UpdateOverlap();
                let overlap = self.target_timezone - self.local_timezone;
                if overlap.is_negative() {
                    self.overlap = overlap.wrapping_abs();
                } else {
                    self.overlap = overlap;
                }
            },
            Msg::UpdateDesiredOverlap(hours) => {
                self.desired_overlap = hours
            }
        }
        true
    } 

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        self.overlap = self.target_timezone - self.local_timezone;
        true
    }

    fn view(&self) -> Html {
        let update_local_timezone = self.link.callback(|e: InputData| Msg::UpdateLocalTime(e.value.parse().unwrap()));
        let update_target_timezone = self.link.callback(|e: InputData| Msg::UpdateTargetTimezone(e.value.parse().unwrap()));
        let update_desired_overlap = self.link.callback(|e: InputData| Msg::UpdateDesiredOverlap(e.value.parse().unwrap()));
        Msg::UpdateOverlap();
        html! {
            <div>
                <p>{"Base timezone"}</p>
                <input type="text" placeholder="0" oninput=update_local_timezone value={self.local_timezone}/>
                <p>{"Target timezone"}</p>
                <input type="text" placeholder="0" oninput=update_target_timezone value={self.target_timezone}/>
                <p>{"Desired Overlap (hours)"}</p>
                <input type="number" placeholder="4" oninput=update_desired_overlap value={self.desired_overlap} />
                <p>{"Working works overlap (hours)"}</p>
                <input type="text" readonly=true value={self.overlap}/>
            </div>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Counter>::new().mount_to_body();
}