use super::{RequestScreen, Screen};
use crate::wallet::Wallet;
use concoct::View;
use concoct::{
    composable::{
        key,
        material::Button,
        remember,
        state::{state, State},
        stream, Container, Text,
    },
    dimension::{DevicePixels, Size},
};
use futures::stream;
use taffy::style::{AlignItems, Dimension, JustifyContent};

#[track_caller]
pub fn history_screen(screen: State<Screen>, wallet: Wallet) {
    Container::build_column(move || {
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

        if let Some(transactions) = transactions.get().cloned() {
            if transactions.is_empty() {
                Container::build_column(move || {
                    Text::new("No transactions...yet!");
                    Button::new(|| Text::new("Create a request"))
                        .on_press(move || {
                            *screen.get().as_mut() = Screen::Request(RequestScreen::Share)
                        })
                        .view();
                })
                .align_items(AlignItems::Center)
                .justify_content(JustifyContent::Center)
                .flex_grow(1.)
                .gap(Size::default().height(Dimension::Points(20.dp())))
                .view()
            } else {
                for (idx, transaction) in transactions.into_iter().enumerate() {
                    key(idx as _, move || Text::new(transaction.to_string()))
                }
            }
        } else {
            Text::new("Loading...")
        }
    })
    .flex_grow(1.)
    .view();
}
