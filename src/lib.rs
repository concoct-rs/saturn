use bitcoin_hashes::Hash;
use btc::MyWallet;
use concoct::composable::container::Padding;
use concoct::composable::material::{text, NavigationBar, NavigationBarItem};
use concoct::composable::{material::Button, Text};
use concoct::composable::{remember, state, stream, Container, Icon};
use concoct::DevicePixels;
use futures::{Stream, StreamExt};

use rust_decimal::Decimal;

use serde::Deserialize;
use skia_safe::{Color4f, Paint, RGB};
use std::time::Duration;
use taffy::prelude::Size;
use taffy::style::{AlignItems, Dimension, JustifyContent};
use tokio::time::interval;
use tokio_stream::wrappers::IntervalStream;

#[cfg(target_os = "android")]
#[no_mangle]
fn android_main(android_app: android_activity::AndroidApp) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let _guard = rt.enter();

    concoct::render::run(app, android_app);
}

mod btc;

mod currency;
use currency::Currency;

mod screen;
use screen::{balance_screen, history_screen, request_screen, send_screen, RequestScreen, Screen};

const PRIVATE_KEY: &'static str = "Kzh4KzaKATrfKj6hsMQaWEnza4bAn9WM11JZcqpKR4WymJpPHivU";

#[derive(Deserialize, Debug)]
struct Response {
    address: String,
    n_tx: u64,
    total_received: u64,
    total_sent: u64,
    final_balance: u64,
    txs: Vec<Transaction>,
}

#[derive(Clone, Deserialize, Debug)]
struct Transaction {
    hash: String,
    time: u64,
    result: i64,
}

#[derive(Deserialize, Debug)]
struct Input {
    prev_out: PrevOut,
}

#[derive(Deserialize, Debug)]
struct PrevOut {
    value: u64,
    n: u64,
    addr: String,
}

#[derive(Deserialize, Debug)]
struct Output {
    value: u64,
    n: u64,
    addr: String,
}

#[derive(Deserialize)]
struct RateResponseData {
    #[serde(rename = "rateUsd")]
    rate: Decimal,
}

#[derive(Deserialize)]
struct RateResponse {
    data: RateResponseData,
}

async fn make_stream() -> impl Stream<Item = Decimal> {
    Box::pin(
        IntervalStream::new(interval(Duration::from_secs(60))).then(|_| async {
            let res: RateResponse = reqwest::get("http://api.coincap.io/v2/rates/bitcoin")
                .await
                .unwrap()
                .json()
                .await
                .unwrap();
            res.data.rate
        }),
    )
}

#[track_caller]
pub fn app() {
    Container::build_column(|| {
        let display = state(|| Screen::Balance);
        let currency = state(|| Currency::Bitcoin);

        let rate = state(|| Decimal::ZERO);
        remember([], || {
            stream(make_stream(), move |value| {
                *rate.get().as_mut() = value;
            })
        });

        let wallet = state(MyWallet::new);

        Container::build_column(move || {
            let current_rate = rate.get().cloned();
            match display.get().cloned() {
                Screen::Balance => {
                    balance_screen(display, currency, current_rate, &*wallet.get().as_ref())
                }
                Screen::Send => send_screen(display, currency, current_rate),
                Screen::Request(request) => {
                    request_screen(request, display, currency, current_rate, wallet)
                }
                Screen::History => history_screen(wallet),
            }
        })
        .flex_grow(1.)
        .padding(Padding::default().horizontal(Dimension::Points(12.dp())))
        .view();

        NavigationBar::new(move || {
            NavigationBarItem::build(
                || {
                    Icon::build(
                        include_str!("../assets/wallet.svg"),
                        Paint::new(Color4f::from(RGB::from((0, 0, 0))), None),
                    )
                    .view()
                },
                || text("Wallet"),
                move || *display.get().as_mut() = Screen::Balance,
            )
            .is_selected(display.get().cloned() == Screen::Balance)
            .view();

            NavigationBarItem::build(
                || {
                    Icon::build(
                        include_str!("../assets/outbox.svg"),
                        Paint::new(Color4f::from(RGB::from((0, 0, 0))), None),
                    )
                    .view()
                },
                || text("Send"),
                move || *display.get().as_mut() = Screen::Send,
            )
            .is_selected(display.get().cloned() == Screen::Send)
            .view();

            NavigationBarItem::build(
                || {
                    Icon::build(
                        include_str!("../assets/inbox.svg"),
                        Paint::new(Color4f::from(RGB::from((0, 0, 0))), None),
                    )
                    .view()
                },
                || text("Request"),
                move || *display.get().as_mut() = Screen::Request(RequestScreen::Share),
            )
            .is_selected(if let Screen::Request(_) = display.get().cloned() {
                true
            } else {
                false
            })
            .view();

            NavigationBarItem::build(
                || {
                    Icon::build(
                        include_str!("../assets/history.svg"),
                        Paint::new(Color4f::from(RGB::from((0, 0, 0))), None),
                    )
                    .view()
                },
                || text("History"),
                move || *display.get().as_mut() = Screen::History,
            )
            .is_selected(display.get().cloned() == Screen::History)
            .view();
        })
    })
    .align_items(AlignItems::Stretch)
    .justify_content(JustifyContent::SpaceEvenly)
    .flex_grow(1.)
    .padding(Padding::default().top(Dimension::Points(60.dp())))
    .view()
}

#[track_caller]
fn full_width_button(label: impl Into<String>, on_press: impl FnMut() + 'static) {
    let label = label.into();

    Button::build(on_press, move || Text::new(label.clone()))
        .size(Size {
            width: Dimension::Percent(1.),
            height: Dimension::Points(40.dp()),
        })
        .view()
}
