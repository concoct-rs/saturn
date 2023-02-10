use crate::btc::MyWallet;
use concoct::composable::{
    key,
    state::{state, State},
    Container, Text,
};

#[track_caller]
pub fn history_screen(wallet: State<MyWallet>) {
    Container::column(move || {
        let transactions = state(|| wallet.get().as_ref().get_transactions());

        Container::column(move || {
            for (idx, transaction) in transactions.get().cloned().into_iter().enumerate() {
                key(idx as _, move || {
                    Text::new((transaction.received as i64 - transaction.sent as i64).to_string())
                })
            }
        })
    });
}
