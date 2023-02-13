use std::sync::mpsc;

use crate::{
    btc::Message,
    currency::{currency_text, Currency},
};
use concoct::composable::{
    remember,
    state::{state, State},
    stream, Container, Text,
};
use futures::stream;
use rust_decimal::Decimal;
use tokio::sync::oneshot;

pub fn balance_screen(currency: State<Currency>, rate: Decimal, wallet: mpsc::Sender<Message>) {
    Container::column(move || {
        let balance = state(|| None);

        let wallet = wallet.clone();
        remember([], move || {
            let wallet = wallet.clone();
            stream(
                Box::pin(async move {
                    stream::once(Box::pin(async move {
                        let (tx, rx) = oneshot::channel();
                        wallet.send(Message::Balance { tx }).unwrap();

                        rx.await.unwrap()
                    }))
                }),
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
