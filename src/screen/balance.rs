use super::Screen;
use crate::{
    btc::MyWallet,
    currency::{currency_text, Currency},
};
use concoct::composable::state::{state, State};
use rust_decimal::Decimal;

pub fn balance_screen(
    _display: State<Screen>,
    currency: State<Currency>,
    rate: Decimal,
    wallet: &MyWallet,
) {
    let balance = state(|| wallet.get_balance().to_string());

    currency_text(currency, balance, rate);
}
