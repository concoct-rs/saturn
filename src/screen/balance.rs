use crate::{
    currency::{currency_text, Currency},
    wallet::Wallet,
};
use concoct::composable::{
    remember,
    state::{state, State},
    stream, Container, Text,
};
use futures::stream;
use rust_decimal::Decimal;

pub fn balance_screen(currency: State<Currency>, rate: Decimal, wallet: Wallet) {
    Container::column(move || {
        let balance = state(|| None);

        let wallet = wallet.clone();
        remember([], move || {
            let wallet = wallet.clone();
            stream(
                Box::pin(async move { stream::once(Box::pin(wallet.balance())) }),
                move |bal| {
                    *balance.get().as_mut() = Some(bal);
                },
            )
        });

        if let Some(balance) = balance.get().cloned() {
            currency_text(currency, balance.confirmed.to_string(), rate);
        } else {
            Text::new("Loading...")
        }
    })
}
