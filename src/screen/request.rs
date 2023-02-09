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

                let label = if let Some(amount) = amount.get().cloned() {
                    amount
                } else {
                    String::from("Add amount")
                };
                full_width_button(label, move || {
                    *display.get().as_mut() = Screen::Request(RequestScreen::Amount);
                });

                #[cfg(target_os = "android")]
                full_width_button("Share", || {
                    use android_intent::{with_current_env, Action, Extra, Intent};

                    with_current_env(|env| {
                        Intent::new(env, Action::Send)
                            .with_type("text/plain")
                            .with_extra(Extra::Text, "Hello World!")
                            .into_chooser()
                            .start_activity()
                            .unwrap()
                    })
                });

                #[cfg(not(target_os = "android"))]
                full_width_button("Copy", || {});
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
