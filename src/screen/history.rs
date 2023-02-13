use crate::btc::Wallet;
use concoct::composable::{key, remember, state::state, stream, Container, Text};
use futures::stream;

#[track_caller]
pub fn history_screen(wallet: Wallet) {
    Container::column(move || {
        let transactions = state(|| None);

        let wallet = wallet.clone();
        remember([], move || {
            let wallet = wallet.clone();
            stream(
                Box::pin(async move { stream::once(Box::pin(wallet.transactions())) }),
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
