use crate::components::header::Header;
use crate::index::depart_date::DepartDate;
use crate::index::high_speed::HighSpeed;
use crate::index::journey::Journey;
use crate::index::submit::Submit;
use crate::store::store::{reducer, StoreDispatch, StoreModel};
use chrono::prelude::*;
use std::rc::Rc;

use yew::{html, Html};
use yew_functional::{use_reducer_with_init, ContextProvider, FunctionComponent, FunctionProvider};

pub struct IndexFC {}
impl FunctionProvider for IndexFC {
    type TProps = ();

    fn run(_: &Self::TProps) -> Html {
        let initail_state = StoreModel {
            from: "北京".to_string(),
            to: "上海".to_string(),
            local_time: Local::now(),
            is_high_speed: true,
        };

        let (store, dispatch) =
            use_reducer_with_init(reducer, initail_state, |initail_state: StoreModel| {
                initail_state
            });

        let dispatch = StoreDispatch { foo: dispatch };
        type StoreModelContextProvider = ContextProvider<Rc<StoreModel>>;
        type StoreDispatchContextProvider = ContextProvider<StoreDispatch>;

        let StoreModel {
            is_high_speed,
            local_time,
            ..
        } = *store;

        return html! {
            <>
                <StoreDispatchContextProvider context=dispatch>
                    <StoreModelContextProvider context=store>
                        <div class="header-wrapper">
                            <Header title="火车票"/>
                        </div>
                        <form action="./query.html" class="form">
                            <Journey
                            // on_exchange_from_to=Callback::from(move |_| on_exchange_from_to() )
                            />
                            <DepartDate date_time={local_time}
                            // {...departDateCbs}
                            />
                            <HighSpeed is_high_speed={is_high_speed}
                            // on_toggle_is_high_speed=Callback::from(move |_| on_toggle_is_high_speed() )
                            //  {...highSpeedCbs}
                            />
                            <Submit />
                        </form>

                    </StoreModelContextProvider >
                </StoreDispatchContextProvider >
            </>
        };
    }
}
pub type Index = FunctionComponent<IndexFC>;