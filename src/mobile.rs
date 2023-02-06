use crate::{full_width_button, Currency, CurrencyInputHandler};
use concoct::composable::container;
use concoct::composable::material::button::{button, ButtonColors, ButtonModifier};
use concoct::composable::state::State;
use concoct::composable::text;
use concoct::modify::container::ContainerModifier;
use concoct::modify::container::{Gap, Padding};
use concoct::modify::Modifier;
use concoct::modify::ModifyExt;
use concoct::DevicePixels;
use skia_safe::Color4f;
use taffy::prelude::Size;
use taffy::style::{AlignItems, Dimension, FlexDirection};

#[track_caller]
pub fn currency_input(amount: State<String>, currency: State<Currency>) {
    container(
        Modifier
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
    button(
        Modifier
            .colors(ButtonColors::from(Color4f::new(0., 0., 0., 0.)))
            .size(Size {
                width: Dimension::Percent(1.),
                height: Dimension::Undefined,
            }),
        move || text(Modifier, c),
        move || {
            CurrencyInputHandler::new(amount, currency).push_char(c);
        },
    );
}

#[track_caller]
fn currency_input_button_row(composable: impl FnMut() + 'static) {
    container(
        Modifier
            .align_items(AlignItems::Stretch)
            .flex_direction(FlexDirection::Row)
            .flex_grow(1.)
            .gap(Gap::default().width(Dimension::Points(20.dp()))),
        composable,
    );
}
