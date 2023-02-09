use super::CurrencyInputHandler;
use crate::{full_width_button, Currency};
use concoct::composable::container::{Gap, Padding};
use concoct::composable::material::button::ButtonColors;
use concoct::composable::material::Button;
use concoct::composable::state::State;
use concoct::composable::{Container, Text};
use concoct::DevicePixels;
use skia_safe::Color4f;
use taffy::prelude::Size;
use taffy::style::{AlignItems, Dimension};

#[track_caller]
pub fn currency_input(amount: State<String>, currency: State<Currency>) {
    Container::build_column(move || {
        currency_input_button_row(move || {
            currency_input_button('1', amount, currency);
            currency_input_button('2', amount, currency);
            currency_input_button('3', amount, currency);
        });

        currency_input_button_row(move || {
            currency_input_button('4', amount, currency);
            currency_input_button('5', amount, currency);
            currency_input_button('6', amount, currency);
        });

        currency_input_button_row(move || {
            currency_input_button('7', amount, currency);
            currency_input_button('8', amount, currency);
            currency_input_button('9', amount, currency);
        });

        currency_input_button_row(move || {
            full_width_button(".", move || {
                CurrencyInputHandler::new(amount, currency).push_decimal()
            });
            currency_input_button('0', amount, currency);
            full_width_button("<", move || {
                CurrencyInputHandler::new(amount, currency).back()
            });
        });
    })
    .align_items(AlignItems::Stretch)
    .flex_grow(1.)
    .gap(Gap::default().height(Dimension::Points(20.dp())))
    .padding(Padding::default().vertical(Dimension::Points(40.dp())))
    .view();
}

#[track_caller]
fn currency_input_button(c: char, amount: State<String>, currency: State<Currency>) {
    Button::build(
        move || {
            CurrencyInputHandler::new(amount, currency).push_char(c);
        },
        move || Text::new(c),
    )
    .colors(ButtonColors::from(Color4f::new(0., 0., 0., 0.)))
    .size(Size {
        width: Dimension::Percent(1.),
        height: Dimension::Undefined,
    })
    .view();
}

#[track_caller]
fn currency_input_button_row(composable: impl FnMut() + 'static) {
    Container::build_row(composable)
        .align_items(AlignItems::Stretch)
        .flex_grow(1.)
        .gap(Gap::default().width(Dimension::Points(20.dp())))
        .view();
}
