use std::sync::mpsc::Sender;

use crate::btc::Message;
use concoct::composable::{key, remember, state::state, stream, Container, Text};
use futures::stream;
use tokio::sync::oneshot;

#[track_caller]
pub fn history_screen(wallet: Sender<Message>) {
    Container::column(move || {
        let transactions = state(|| None);

        let wallet = wallet.clone();
        remember([], move || {
            let wallet = wallet.clone();
            stream(
                Box::pin(async move {
                    stream::once(Box::pin(async move {
                        let (tx, rx) = oneshot::channel();
                        wallet.send(Message::Transactions { tx }).unwrap();

                        rx.await.unwrap()
                    }))
                }),
                move |txs| {
                    *transactions.get().as_mut() = Some(txs);
                },
            )
        });

        Container::column(move || {
            if let Some(transactions) = transactions.get().cloned() {
                for (idx, transaction) in transactions.into_iter().enumerate() {
                    key(idx as _, move || Text::new(transaction.to_string()))
                }
            } else {
                Text::new("Loading...")
            }
        })
    });
}
