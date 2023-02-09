use concoct::composable::container::Padding;
use concoct::composable::state::State;
use concoct::composable::{container, remember, state, stream, Container};
use concoct::composable::{
    container::Gap,
    material::{
        button,
        icon::{icon, Icon},
        Button,
    },
    Text,
};
use concoct::modify::handler::keyboard_input::KeyboardHandler;
use concoct::modify::{ModifyExt, HandlerModifier};
use concoct::DevicePixels;
use concoct::Modifier;
use futures::{Stream, StreamExt};
use rust_decimal::Decimal;
use serde::Deserialize;
use std::time::Duration;
use taffy::prelude::Size;
use taffy::style::{AlignItems, Dimension, FlexDirection, JustifyContent};
use tokio::time::interval;
use tokio_stream::wrappers::IntervalStream;
use winit::event::{ElementState, VirtualKeyCode};

#[cfg(target_os = "android")]
#[no_mangle]
fn android_main(android_app: android_activity::AndroidApp) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let _guard = rt.enter();

    concoct::render::run(app, android_app);
}

mod currency;
use currency::{currency_text, Currency};

#[cfg(target_os = "android")]
mod mobile;

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
            let res: RateResponse = reqwest::get("http://api.coincap.io/v2/rates/bitcoin")
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

#[track_caller]
pub fn app() {
    Container::build_column(|| {
        let display = state(|| Display::Balance);
        let currency = state(|| Currency::Bitcoin);

        let rate = state(|| Decimal::ZERO);
        /*
        remember([], || {
            stream(make_stream(), move |value| {
                *rate.get().as_mut() = value;
            })
        });
         */

        match display.get().cloned() {
            Display::Balance => {
                let balance = state(|| String::from("100"));

                currency_text(currency, balance, rate);

                Container::build_row(move || {
                    full_width_button("Send", move || {
                        *display.get().as_mut() = Display::Send { address: None };
                    });
                    full_width_button("Request", || {});
                })
                .align_items(AlignItems::Stretch)
                .flex_direction(FlexDirection::Row)
                .gap(Gap::default().width(Dimension::Points(40.)))
                .view()
            }
            Display::Send { address } => {
                if let Some(address) = address {
                    let amount = state(|| String::from("0"));

                    Container::build_column(move || {
                        Text::new(&address);

                        Button::new(
                            move || {
                                *display.get().as_mut() = Display::Balance;
                            },
                            || icon(Modifier, Icon::ArrowBack, "Back"),
                        );

                        currency_text(currency, amount, rate);

                        #[cfg(target_os = "android")]
                        mobile::currency_input(amount, currency);

                        full_width_button("Send", move || {});
                    })
                    .align_items(AlignItems::Stretch)
                    .flex_direction(FlexDirection::Column)
                    .flex_grow(1.)
                    .modifier(
                        Modifier.keyboard_handler(CurrencyInputHandler::new(amount, currency)),
                    ).view()
                } else {
                    full_width_button("Continue", move || {
                        *display.get().as_mut() = Display::Send {
                            address: Some("12345".into()),
                        };
                    });
                }
            }
        }
    })
    .align_items(AlignItems::Stretch)
    .justify_content(JustifyContent::SpaceEvenly)
    .flex_grow(1.)
    .padding(Padding::from(Dimension::Points(16.dp())).top(Dimension::Points(40.dp()))).view();
}

#[track_caller]
fn full_width_button(label: impl Into<String>, on_press: impl FnMut() + 'static) {
    let label = label.into();

    /*
    ).size(Size {
        width: Dimension::Percent(1.),
        height: Dimension::Undefined,
    }) */

    Button::build(on_press, move || Text::new(label.clone())).view();
}

pub struct CurrencyInputHandler {
    value: State<String>,
    currency: State<Currency>,
}

impl CurrencyInputHandler {
    fn new(value: State<String>, currency: State<Currency>) -> Self {
        Self { value, currency }
    }

    fn push_char(&self, c: char) {
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

    fn back(&self) {
        self.value.get().as_mut().pop();

        if self.value.get().as_ref().is_empty() {
            self.value.get().as_mut().push('0');
        }
    }

    fn push_decimal(&self) {
        if !self.value.get().as_ref().contains('.') {
            self.value.get().as_mut().push('.');
        }
    }
}

impl KeyboardHandler for CurrencyInputHandler {
    fn handle_keyboard_input(&mut self, state: ElementState, virtual_keycode: VirtualKeyCode) {
        if state == ElementState::Pressed {
            match virtual_keycode {
                VirtualKeyCode::Key0 | VirtualKeyCode::Numpad0 => self.push_char('0'),
                VirtualKeyCode::Key1 | VirtualKeyCode::Numpad1 => self.push_char('1'),
                VirtualKeyCode::Back => self.back(),
                VirtualKeyCode::Period => self.push_decimal(),
                _ => {}
            }
        }
    }
}
