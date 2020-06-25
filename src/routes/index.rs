use crate::components::header::Header;
use crate::index::city_selector::CitySelector;
use crate::index::date_selector::DateSelector;

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
            date_selector_visible: false,
            city_selector_visible: false,
        };

        let (store, dispatch) =
            use_reducer_with_init(reducer, initail_state, |initail_state: StoreModel| {
                initail_state
            });

        let dispatch = StoreDispatch(dispatch);
        type StoreModelContextProvider = ContextProvider<Rc<StoreModel>>;
        type StoreDispatchContextProvider = ContextProvider<StoreDispatch>;

        // let stroe1 = Rc::clone(&store);
        // let StoreModel {
        //     date_selector_visible,
        //     ..
        // } = &*stroe1;

        return html! {
            <>
                <StoreDispatchContextProvider context=dispatch>
                    <StoreModelContextProvider context=store>
                        <div class="header-wrapper">
                            <Header title="火车票" onback=None />
                        </div>
                        <form action="./query.html" class="form">
                            <Journey/>
                            <DepartDate/>
                            <HighSpeed/>
                            <Submit />
                        </form>
                        <CitySelector
                        // show={*date_selector_visible}
                        // {...dateSelectorCbs}
                        // onSelect={onSelectDate}
                        />
                        <DateSelector
                        // show={*date_selector_visible}
                        // {...dateSelectorCbs}
                        // onSelect={onSelectDate}
                        />

                    </StoreModelContextProvider >
                </StoreDispatchContextProvider >
            </>
        };
    }
}
pub type Index = FunctionComponent<IndexFC>;
