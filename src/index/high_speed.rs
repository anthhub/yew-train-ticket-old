// use crate::routes::AppRoute;
use yew::{html, Callback, Html};
use yew_functional::{use_context, FunctionComponent, FunctionProvider};
// use yew_router::prelude::*;
use crate::store::store::{Action, StoreDispatch, StoreModel};
use std::rc::Rc;
pub struct HighSpeedFC {}
pub type HighSpeed = FunctionComponent<HighSpeedFC>;

impl FunctionProvider for HighSpeedFC {
    type TProps = ();

    fn run(_: &Self::TProps) -> Html {
        let context = use_context::<Rc<StoreModel>>();
        let ctx = &context.unwrap();
        let StoreModel { is_high_speed, .. } = &***ctx;

        let checked = if *is_high_speed { "checked" } else { "" };

        let context_dispatch = use_context::<StoreDispatch>();
        let onclick = Callback::from(move |_| match &context_dispatch {
            Some(dispatch) => {
                let dispatch = &*dispatch;
                dispatch.emit(Action::ToggleHighSpeed);
                return ();
            }
            _ => (),
        });

        return html! {
            <div class="high-speed">
            <div class="high-speed-label">{"只看高铁/动车"}</div>
            <div class="high-speed-switch"
            onclick=onclick

            >
                <input type="hidden" name="high_speed" value={is_high_speed} />
                <div
                    class=format!("{} {}", "high-speed-track", checked)
                >
                    <span
                    class=format!("{} {}", "high-speed-handle", checked)
                    ></span>
                </div>
            </div>
        </div>
        };
    }
}
