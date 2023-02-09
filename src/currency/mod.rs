use concoct::{
    composable::{material::Button, state::State, Container, Text},
    modify::{handler::keyboard_input::KeyboardHandler, HandlerModifier},
    DevicePixels, Modifier,
};
use rust_decimal::Decimal;
use std::{
    fmt::{self, Write},
    ops::Not,
};
use taffy::{
    prelude::{Rect, Size},
    style::{AlignItems, Dimension, FlexDirection, JustifyContent},
};
use winit::event::{ElementState, VirtualKeyCode};

mod flex_text;
use flex_text::flex_text;

#[derive(Clone, Copy)]
pub enum Currency {
    Bitcoin,
    USD,
}

impl Currency {
    pub fn convert(self, value: &str, rate: Decimal) -> Decimal {
        let value: Decimal = value.parse().unwrap_or_default();
        match self {
            Currency::Bitcoin => (value * rate).round_dp(2),
            Currency::USD => (value.checked_div(rate).unwrap_or_default()).round_dp(8),
        }
    }
}

impl Not for Currency {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Self::Bitcoin => Self::USD,
            Self::USD => Self::Bitcoin,
        }
    }
}

impl fmt::Display for Currency {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let c = match self {
            Self::Bitcoin => '₿',
            Self::USD => '$',
        };
        f.write_char(c)
    }
}

#[track_caller]
pub fn currency_text(currency: State<Currency>, value: State<String>, rate: Decimal) {
    Container::build_column(move || {
        Container::build_column(move || {
            flex_text(format!(
                "{}{}",
                currency.get().cloned(),
                value.get().as_ref()
            ));
        })
        .align_items(AlignItems::Center)
        .justify_content(JustifyContent::Center)
        .flex_direction(FlexDirection::Column)
        .margin(Rect::from_points(20., 20., 50., 50.))
        .size(Size {
            width: Dimension::Percent(1.),
            height: Dimension::Points(200.dp()),
        })
        .view();

        Button::new(
            move || {
                let converted = currency
                    .get()
                    .cloned()
                    .convert(&*value.get().as_ref(), rate)
                    .to_string();
                *value.get().as_mut() = converted;
                *currency.get().as_mut() = !currency.get().cloned();
            },
            move || {
                Text::new(format!(
                    "{}{}",
                    !currency.get().cloned(),
                    currency
                        .get()
                        .cloned()
                        .convert(&*value.get().as_ref(), rate)
                ))
            },
        );
    })
    .align_items(AlignItems::Center)
    .justify_content(JustifyContent::Center)
    .flex_direction(FlexDirection::Column)
    .flex_grow(1.)
    .view()
}

#[track_caller]
pub fn currency_input(amount: State<String>, currency: State<Currency>, rate: Decimal) {
    Container::build_column(move || {
        currency_text(currency, amount, rate);

        #[cfg(target_os = "android")]
        mobile::currency_input(amount, currency);
    })
    .align_items(AlignItems::Stretch)
    .flex_direction(FlexDirection::Column)
    .flex_grow(1.)
    .modifier(Modifier.keyboard_handler(CurrencyInputHandler::new(amount, currency)))
    .view()
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
