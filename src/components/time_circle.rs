use yew::prelude::*;

pub struct TimeCircle {

}

impl Component for TimeCircle {
  type Message = ();
  type Properties = ();

  fn create(_: Self::Properties, _link: ComponentLink<Self>) -> Self {
    Self {
    }
  }

  fn update(&mut self, _msg: Self::Message) -> ShouldRender {
    false
  }

  fn change(&mut self, _props: Self::Properties) -> ShouldRender {
    false
  }

  fn view(&self) -> Html {
    html! {
      <div>
        <svg height="250" width="250" viewBox="0 0 160 160" class="donut-chart">
          <g>
              <circle cx="80" cy="80" r="60" stroke="#6495ED" stroke-width="30" stroke-dasharray="374.99111843077515" stroke-dashoffset="361.2831551628262" fill="transparent" transform="rotate(-90, 80, 80)"></circle> <text text-anchor="middle" dy="3px" x="87.8315715332031" y="20.51330831757138">{"0"}</text>
          </g>
          <g>
              <circle cx="80" cy="80" r="60" stroke="goldenrod" stroke-width="30" stroke-dasharray="374.99111843077515" stroke-dashoffset="361.2831551628262" fill="transparent" transform="rotate(-74.99999999999999, 80, 80)"></circle> <text text-anchor="middle" dy="3px" x="102.9610059419054" y="24.5672280493228">{"1"}</text>
          </g>
          <g>
              <circle cx="80" cy="80" r="60" stroke="#cd5c5c" stroke-width="30" stroke-dasharray="374.99111843077515" stroke-dashoffset="361.2831551628262" fill="transparent" transform="rotate(-59.99999999999998, 80, 80)"></circle> <text text-anchor="middle" dy="3px" x="116.52568574052327" y="32.39879958252591">{"2"}</text>
          </g>
          <g>
              <circle cx="80" cy="80" r="60" stroke="thistle" stroke-width="30" stroke-dasharray="374.99111843077515" stroke-dashoffset="361.2831551628262" fill="transparent" transform="rotate(-44.99999999999997, 80, 80)"></circle> <text text-anchor="middle" dy="3px" x="127.60120041747413" y="43.47431425947679">{"3"}</text>
          </g>
          <g>
              <circle cx="80" cy="80" r="60" stroke="lightgray" stroke-width="30" stroke-dasharray="374.99111843077515" stroke-dashoffset="361.2831551628262" fill="transparent" transform="rotate(-29.999999999999964, 80, 80)"></circle> <text text-anchor="middle" dy="3px" x="135.43277195067722" y="57.03899405809465">{"4"}</text>
          </g>
          <g>
              <circle cx="80" cy="80" r="60" stroke-width="30" stroke-dasharray="374.99111843077515" stroke-dashoffset="361.2831551628262" fill="transparent" transform="rotate(-14.999999999999956, 80, 80)"></circle> <text text-anchor="middle" dy="3px" x="139.4866916824286" y="72.16842846679695">{"5"}</text>
          </g>
          <g>
              <circle cx="80" cy="80" r="60" stroke-width="30" stroke-dasharray="374.99111843077515" stroke-dashoffset="361.2831551628262" fill="transparent" transform="rotate(5.3290705182007514e-14, 80, 80)"></circle> <text text-anchor="middle" dy="3px" x="139.4866916824286" y="87.83157153320316">{"6"}</text>
          </g>
          <g>
              <circle cx="80" cy="80" r="60" stroke-width="30" stroke-dasharray="374.99111843077515" stroke-dashoffset="361.2831551628262" fill="transparent" transform="rotate(15.000000000000062, 80, 80)"></circle> <text text-anchor="middle" dy="3px" x="135.4327719506772" y="102.96100594190546">{"7"}</text>
          </g>
          <g>
              <circle cx="80" cy="80" r="60" stroke-width="30" stroke-dasharray="374.99111843077515" stroke-dashoffset="361.2831551628262" fill="transparent" transform="rotate(30.00000000000007, 80, 80)"></circle> <text text-anchor="middle" dy="3px" x="127.60120041747406" y="116.5256857405233">{"8"}</text>
          </g>
          <g>
              <circle cx="80" cy="80" r="60" stroke-width="30" stroke-dasharray="374.99111843077515" stroke-dashoffset="361.2831551628262" fill="transparent" transform="rotate(45.00000000000008, 80, 80)"></circle> <text text-anchor="middle" dy="3px" x="116.52568574052316" y="127.60120041747416">{"9"}</text>
          </g>
          <g>
              <circle cx="80" cy="80" r="60" stroke-width="30" stroke-dasharray="374.99111843077515" stroke-dashoffset="361.2831551628262" fill="transparent" transform="rotate(60.000000000000085, 80, 80)"></circle> <text text-anchor="middle" dy="3px" x="102.9610059419053" y="135.43277195067725">{"10"}</text>
          </g>
          <g>
              <circle cx="80" cy="80" r="60" stroke-width="30" stroke-dasharray="374.99111843077515" stroke-dashoffset="361.2831551628262" fill="transparent" transform="rotate(75.0000000000001, 80, 80)"></circle> <text text-anchor="middle" dy="3px" x="87.831571533203" y="139.48669168242864">{"11"}</text>
          </g>
          <g>
              <circle cx="80" cy="80" r="60" stroke-width="30" stroke-dasharray="374.99111843077515" stroke-dashoffset="361.2831551628262" fill="transparent" transform="rotate(90.00000000000011, 80, 80)"></circle> <text text-anchor="middle" dy="3px" x="72.16842846679678" y="139.4866916824286">{"12"}</text>
          </g>
          <g>
              <circle cx="80" cy="80" r="60" stroke-width="30" stroke-dasharray="374.99111843077515" stroke-dashoffset="361.2831551628262" fill="transparent" transform="rotate(105.00000000000013, 80, 80)"></circle> <text text-anchor="middle" dy="3px" x="57.038994058094495" y="135.43277195067716">{"13"}</text>
          </g>
          <g>
              <circle cx="80" cy="80" r="60" stroke-width="30" stroke-dasharray="374.99111843077515" stroke-dashoffset="361.2831551628262" fill="transparent" transform="rotate(120.00000000000014, 80, 80)"></circle> <text text-anchor="middle" dy="3px" x="43.47431425947666" y="127.60120041747402">{"14"}</text>
          </g>
          <g>
              <circle cx="80" cy="80" r="60" stroke-width="30" stroke-dasharray="374.99111843077515" stroke-dashoffset="361.2831551628262" fill="transparent" transform="rotate(135.00000000000014, 80, 80)"></circle> <text text-anchor="middle" dy="3px" x="32.398799582525804" y="116.52568574052313">{"15"}</text>
          </g>
          <g>
              <circle cx="80" cy="80" r="60" stroke-width="30" stroke-dasharray="374.99111843077515" stroke-dashoffset="361.2831551628262" fill="transparent" transform="rotate(150.00000000000014, 80, 80)"></circle> <text text-anchor="middle" dy="3px" x="24.567228049322736" y="102.96100594190524">{"16"}</text>
          </g>
          <g>
              <circle cx="80" cy="80" r="60" stroke-width="30" stroke-dasharray="374.99111843077515" stroke-dashoffset="361.2831551628262" fill="transparent" transform="rotate(165.00000000000014, 80, 80)"></circle> <text text-anchor="middle" dy="3px" x="20.513308317571358" y="87.83157153320296">{"17"}</text>
          </g>
          <g>
              <circle cx="80" cy="80" r="60" stroke-width="30" stroke-dasharray="374.99111843077515" stroke-dashoffset="361.2831551628262" fill="transparent" transform="rotate(180.00000000000014, 80, 80)"></circle> <text text-anchor="middle" dy="3px" x="20.5133083175714" y="72.16842846679677">{"18"}</text>
          </g>
          <g>
              <circle cx="80" cy="80" r="60" stroke-width="30" stroke-dasharray="374.99111843077515" stroke-dashoffset="361.2831551628262" fill="transparent" transform="rotate(195.00000000000014, 80, 80)"></circle> <text text-anchor="middle" dy="3px" x="24.567228049322857" y="57.038994058094474">{"19"}</text>
          </g>
          <g>
              <circle cx="80" cy="80" r="60" stroke-width="30" stroke-dasharray="374.99111843077515" stroke-dashoffset="361.2831551628262" fill="transparent" transform="rotate(210.00000000000014, 80, 80)"></circle> <text text-anchor="middle" dy="3px" x="32.39879958252597" y="43.47431425947666">{"20"}</text>
          </g>
          <g>
              <circle cx="80" cy="80" r="60" stroke-width="30" stroke-dasharray="374.99111843077515" stroke-dashoffset="361.2831551628262" fill="transparent" transform="rotate(225.00000000000014, 80, 80)"></circle> <text text-anchor="middle" dy="3px" x="43.47431425947688" y="32.398799582525804">{"21"}</text>
          </g>
          <g>
              <circle cx="80" cy="80" r="60" stroke-width="30" stroke-dasharray="374.99111843077515" stroke-dashoffset="361.2831551628262" fill="transparent" transform="rotate(240.00000000000014, 80, 80)"></circle> <text text-anchor="middle" dy="3px" x="57.03899405809473" y="24.56722804932275">{"22"}</text>
          </g>
          <g>
              <circle cx="80" cy="80" r="60" stroke-width="30" stroke-dasharray="374.99111843077515" stroke-dashoffset="361.2831551628262" fill="transparent" transform="rotate(255.00000000000014, 80, 80)"></circle> <text text-anchor="middle" dy="3px" x="72.16842846679705" y="20.513308317571358">{"23"}</text>
          </g>
        </svg>
      </div>
    }
  }
}