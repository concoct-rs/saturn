use bitcoin_hashes::{sha256, Hash};
use btc::MyWallet;
use concoct::composable::container::Padding;
use concoct::composable::material::{NavigationBar, NavigationBarItem};
use concoct::composable::{material::Button, Text};
use concoct::composable::{remember, state, stream, Container};
use concoct::DevicePixels;
use futures::{Stream, StreamExt};
use lightning::ln::PaymentSecret;
use lightning_invoice::{Invoice, InvoiceBuilder};
use rust_decimal::prelude::ToPrimitive;
use rust_decimal::Decimal;
use secp256k1::{Secp256k1, SecretKey};
use serde::Deserialize;
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
use screen::{balance_screen, request_screen, send_screen, RequestScreen, Screen};

fn new_invoice(amount: Option<Decimal>) -> Invoice {
    let private_key = SecretKey::from_slice(
        &[
            0xe1, 0x26, 0xf6, 0x8f, 0x7e, 0xaf, 0xcc, 0x8b, 0x74, 0xf5, 0x4d, 0x26, 0x9f, 0xe2,
            0x06, 0xbe, 0x71, 0x50, 0x00, 0xf9, 0x4d, 0xac, 0x06, 0x7d, 0x1c, 0x04, 0xa8, 0xca,
            0x3b, 0x2d, 0xb7, 0x34,
        ][..],
    )
    .unwrap();

    let payment_hash = sha256::Hash::from_slice(&[0; 32][..]).unwrap();
    let payment_secret = PaymentSecret([42u8; 32]);

    let mut builder = InvoiceBuilder::new(lightning_invoice::Currency::Bitcoin)
        .description("Coins pls!".into())
        .payment_hash(payment_hash)
        .payment_secret(payment_secret)
        .current_timestamp()
        .min_final_cltv_expiry(144);

    if let Some(btc) = amount {
        let millisatoshis = (btc * Decimal::new(10i64.pow(11), 0)).to_u64().unwrap();
        builder = builder.amount_milli_satoshis(millisatoshis);
    }

    builder
        .build_signed(|hash| Secp256k1::new().sign_ecdsa_recoverable(hash, &private_key))
        .unwrap()
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
        IntervalStream::new(interval(Duration::from_secs(5))).then(|_| async {
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

        let current_rate = rate.get().cloned();
        match display.get().cloned() {
            Screen::Balance => {
                balance_screen(display, currency, current_rate, &*wallet.get().as_ref())
            }
            Screen::Send => send_screen(display, currency, current_rate),
            Screen::Request(request) => {
                request_screen(request, display, currency, current_rate, wallet)
            }
        }

        NavigationBar::new(move || {
            NavigationBarItem::build(
                || Text::new("W"),
                || Text::new("Wallet"),
                move || *display.get().as_mut() = Screen::Balance,
            )
            .view();

            NavigationBarItem::build(
                || Text::new("S"),
                || Text::new("Send"),
                move || *display.get().as_mut() = Screen::Send,
            )
            .view();

            NavigationBarItem::build(
                || Text::new("R"),
                || Text::new("Request"),
                move || *display.get().as_mut() = Screen::Request(RequestScreen::Share),
            )
            .view();

            NavigationBarItem::build(
                || Text::new("H"),
                || Text::new("History"),
                move || *display.get().as_mut() = Screen::Balance,
            )
            .view();
        })
    })
    .align_items(AlignItems::Stretch)
    .justify_content(JustifyContent::SpaceEvenly)
    .flex_grow(1.)
    .padding(Padding::default().top(Dimension::Points(40.dp())))
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
