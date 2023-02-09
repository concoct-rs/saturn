use concoct::{
    composable::{
        container,
        material::{button, Button},
        state::State,
        Container, Text,
    },
    modify::ModifyExt,
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

mod flex_text;
use flex_text::flex_text;

#[track_caller]
pub fn currency_text(currency: State<Currency>, value: State<String>, rate: State<Decimal>) {
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
                    .convert(&*value.get().as_ref(), rate.get().cloned())
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
                        .convert(&*value.get().as_ref(), rate.get().cloned())
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
