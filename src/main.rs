use concoct::composable::material::button;
use concoct::composable::state::State;
use concoct::composable::{container, remember, state, stream, text};
use concoct::modify::keyboard_input::KeyboardHandler;
use concoct::modify::{Gap, Padding};
use concoct::{render::run, Modifier};
use futures::{Stream, StreamExt};
use rust_decimal::Decimal;
use serde::Deserialize;
use std::time::Duration;
use taffy::prelude::Size;
use taffy::style::{AlignItems, Dimension, FlexDirection, JustifyContent};
use tokio::time::interval;
use tokio_stream::wrappers::IntervalStream;
use winit::event::{ElementState, VirtualKeyCode};

mod currency;
use currency::{currency_text, Currency};

#[derive(Deserialize)]
struct RateResponseData {
    #[serde(rename = "rateUsd")]
    rate: Decimal,
}

#[derive(Deserialize)]
struct RateResponse {
    data: RateResponseData,
}

async fn make_stream() -> impl Stream<Item = Decimal> {
    Box::pin(
        IntervalStream::new(interval(Duration::from_secs(5))).then(|_| async {
            let res: RateResponse = reqwest::get("https://api.coincap.io/v2/rates/bitcoin")
                .await
                .unwrap()
                .json()
                .await
                .unwrap();
            res.data.rate
        }),
    )
}

#[derive(Clone)]
enum Display {
    Balance,
    Send { address: Option<String> },
}

#[tokio::main]
async fn main() {
    run(app)
}

#[track_caller]
fn app() {
    container(
        Modifier::default()
            .align_items(AlignItems::Stretch)
            .justify_content(JustifyContent::SpaceEvenly)
            .flex_direction(FlexDirection::Column)
            .flex_grow(1.)
            .padding(Padding::from(Dimension::Points(40.))),
        || {
            let display = state(|| Display::Balance);
            let currency = state(|| Currency::Bitcoin);

            let rate = state(|| Decimal::ZERO);
            remember(&[], || {
                stream(make_stream(), move |value| {
                    *rate.get().as_mut() = value;
                })
            });

            match display.get().cloned() {
                Display::Balance => {
                    let balance = state(|| String::from("100"));

                    currency_text(currency, balance, rate);

                    container(
                        Modifier::default()
                            .align_items(AlignItems::Stretch)
                            .flex_direction(FlexDirection::Row)
                            .gap(Gap::default().width(Dimension::Points(40.)))
                            .size(Size {
                                width: Dimension::Percent(1.),
                                height: Dimension::Undefined,
                            }),
                        move || {
                            full_width_button("Send", move || {
                                *display.get().as_mut() = Display::Send { address: None };
                            });
                            full_width_button("Request", || {});
                        },
                    )
                }
                Display::Send { address } => {
                    if let Some(address) = address {
                        let amount = state(|| String::from("0"));

                        container(
                            Modifier::default()
                                .align_items(AlignItems::Center)
                                .flex_direction(FlexDirection::Column)
                                .flex_grow(1.)
                                .keyboard_handler(CurrencyInputKeyboardHandler::new(
                                    amount, currency,
                                )),
                            move || {
                                text(Modifier::default(), &address);

                                button(Modifier::default(), "Cancel", move || {
                                    *display.get().as_mut() = Display::Balance;
                                });

                                currency_text(currency, amount, rate);

                                full_width_button("Send", move || {});
                            },
                        );
                    } else {
                        full_width_button("Continue", move || {
                            *display.get().as_mut() = Display::Send {
                                address: Some("12345".into()),
                            };
                        });
                    }
                }
            }
        },
    );
}

#[track_caller]
fn full_width_button(label: impl Into<String>, on_press: impl FnMut() + 'static) {
    button(
        Modifier::default().size(Size {
            width: Dimension::Percent(1.),
            height: Dimension::Undefined,
        }),
        label,
        on_press,
    );
}

pub struct CurrencyInputKeyboardHandler {
    value: State<String>,
    currency: State<Currency>,
}

impl CurrencyInputKeyboardHandler {
    fn new(value: State<String>, currency: State<Currency>) -> Self {
        Self { value, currency }
    }

    fn push_char(&mut self, c: char) {
        let (max_integer, max_decimal_places) = match self.currency.get().cloned() {
            Currency::Bitcoin => (2, 8),
            Currency::USD => (10, 2),
        };

        if let Some(pos) = self
            .value
            .get()
            .cloned()
            .chars()
            .rev()
            .position(|c| c == '.')
        {
            if pos >= max_decimal_places {
                return;
            }
        } else if self.value.get().as_ref().len() > max_integer {
            return;
        }

        if &*self.value.get().as_ref() == "0" {
            self.value.get().as_mut().pop();
        }

        self.value.get().as_mut().push(c)
    }
}

impl KeyboardHandler for CurrencyInputKeyboardHandler {
    fn handle_keyboard_input(&mut self, state: ElementState, virtual_keycode: VirtualKeyCode) {
        if state == ElementState::Pressed {
            match virtual_keycode {
                VirtualKeyCode::Key0 | VirtualKeyCode::Numpad0 => self.push_char('0'),
                VirtualKeyCode::Key1 | VirtualKeyCode::Numpad1 => self.push_char('1'),
                VirtualKeyCode::Back => {
                    self.value.get().as_mut().pop();

                    if self.value.get().as_ref().is_empty() {
                        self.value.get().as_mut().push('0');
                    }
                }
                VirtualKeyCode::Period => {
                    if !self.value.get().as_ref().contains('.') {
                        self.value.get().as_mut().push('.');
                    }
                }
                _ => {}
            }
        }
    }
}
