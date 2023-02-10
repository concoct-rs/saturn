use super::{RequestScreen, Screen};
use crate::{
    btc::MyWallet,
    currency::{currency_text, Currency},
    full_width_button,
};
use concoct::composable::{
    container::Gap,
    state::{state, State},
    Container,
};
use rust_decimal::Decimal;
use taffy::style::{Dimension, FlexDirection};

pub fn balance_screen(
    display: State<Screen>,
    currency: State<Currency>,
    rate: Decimal,
    wallet: &MyWallet,
) {
    let balance = state(|| wallet.get_balance().to_string());

    currency_text(currency, balance, rate);
}
