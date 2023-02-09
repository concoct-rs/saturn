use super::{RequestScreen, Screen};
use crate::{
    currency::{currency_text, Currency},
    full_width_button,
};
use concoct::composable::{
    container::Gap,
    state::{state, State},
    Container,
};
use rust_decimal::Decimal;
use taffy::style::{Dimension, FlexDirection};

pub fn balance_screen(display: State<Screen>, currency: State<Currency>, rate: Decimal) {
    let balance = state(|| String::from("100"));

    currency_text(currency, balance, rate);

    Container::build_row(move || {
        full_width_button("Send", move || {
            *display.get().as_mut() = Screen::Send;
        });
        full_width_button("Request", move || {
            *display.get().as_mut() = Screen::Request(RequestScreen::Share);
        });
    })
    .flex_direction(FlexDirection::Row)
    .gap(Gap::default().width(Dimension::Points(40.)))
    .view()
}
