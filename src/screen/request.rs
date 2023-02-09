use super::{RequestScreen, Screen};
use crate::currency::{currency_input, Currency};
use crate::full_width_button;
use concoct::composable::state::State;
use concoct::composable::{material::Button, Text};
use concoct::composable::{state, Container};
use rust_decimal::Decimal;
use taffy::style::{AlignItems, JustifyContent};

#[track_caller]
pub fn request_screen(
    request: RequestScreen,
    display: State<Screen>,
    currency: State<Currency>,
    rate: Decimal,
) {
    Container::build_column(move || {
        let amount = state(|| None::<String>);

        match request {
            RequestScreen::Share => {
                Button::new(
                    move || {
                        *display.get().as_mut() = Screen::Balance;
                    },
                    || Text::new("Back"),
                );

                Text::new("12345");

                Button::new(|| {}, || Text::new("Share"));

                Button::new(
                    move || {
                        *display.get().as_mut() = Screen::Request(RequestScreen::Amount);
                    },
                    move || {
                        if let Some(amount) = amount.get().cloned() {
                            Text::new(amount)
                        } else {
                            Text::new("Add amount")
                        }
                    },
                )
            }
            RequestScreen::Amount => {
                let new_amount = state(|| String::from("0"));

                Button::new(
                    move || {
                        *display.get().as_mut() = Screen::Request(RequestScreen::Share);
                    },
                    || Text::new("Back"),
                );

                currency_input(new_amount, currency, rate);

                full_width_button("Request", move || {
                    *amount.get().as_mut() = Some(new_amount.get().cloned());
                    *display.get().as_mut() = Screen::Request(RequestScreen::Share);
                });
            }
        }
    })
    .align_items(AlignItems::Stretch)
    .justify_content(JustifyContent::SpaceBetween)
    .flex_grow(1.)
    .view()
}
