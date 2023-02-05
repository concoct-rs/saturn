use crate::{full_width_button, Currency, CurrencyInputHandler};
use concoct::composable::container;
use concoct::composable::state::State;
use concoct::modify::Gap;
use concoct::modify::{Modifier, Padding};
use concoct::DevicePixels;
use taffy::style::{AlignItems, FlexDirection, Dimension};

#[track_caller]
pub fn currency_input(amount: State<String>, currency: State<Currency>) {
    container(
        Modifier::default()
            .align_items(AlignItems::Stretch)
            .flex_direction(FlexDirection::Column)
            .flex_grow(1.)
            .gap(Gap::default().height(Dimension::Points(20.dp())))
            .padding(Padding::default().vertical(Dimension::Points(40.dp()))),
        move || {
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
        },
    );
}

#[track_caller]
fn currency_input_button(c: char, amount: State<String>, currency: State<Currency>) {
    full_width_button(c, move || {
        CurrencyInputHandler::new(amount, currency).push_char(c);
    });
}

#[track_caller]
fn currency_input_button_row(composable: impl FnMut() + 'static) {
    container(
        Modifier::default()
            .align_items(AlignItems::Stretch)
            .flex_direction(FlexDirection::Row)
            .flex_grow(1.)
            .gap(Gap::default().width(Dimension::Points(20.dp()))),
        composable,
    );
}
