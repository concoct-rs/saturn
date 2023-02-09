use super::Screen;
use crate::{
    currency::{currency_input, Currency},
    full_width_button,
};
use concoct::{
    composable::{
        material::{
            icon::{icon, Icon},
            Button,
        },
        state::{state, State},
        Container, Text,
    },
    Modifier,
};
use rust_decimal::Decimal;
use taffy::style::{AlignItems, FlexDirection};

pub fn send_screen(display: State<Screen>, currency: State<Currency>, rate: Decimal) {
    let address = state(|| None::<String>);

    if let Some(address) = address.get().cloned() {
        let amount = state(|| String::from("0"));

        Container::build_column(move || {
            Button::new(
                move || {
                    *display.get().as_mut() = Screen::Balance;
                },
                || icon(Modifier, Icon::ArrowBack, "Back"),
            );

            Text::new(address.clone());

            currency_input(amount, currency, rate);

            full_width_button("Send", move || {});
        })
        .align_items(AlignItems::Stretch)
        .flex_direction(FlexDirection::Column)
        .flex_grow(1.)
        .view()
    } else {
        full_width_button("Continue", move || {
            *address.get().as_mut() = Some(String::from("12345"));
        });
    }
}
