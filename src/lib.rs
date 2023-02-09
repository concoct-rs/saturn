use concoct::composable::container::Padding;
use concoct::composable::{material::Button, Text};
use concoct::composable::{remember, state, stream, Container};
use concoct::DevicePixels;
use futures::{Stream, StreamExt};
use rust_decimal::Decimal;
use screen::{balance_screen, request_screen, send_screen, Screen};
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

mod currency;
use currency::Currency;

mod screen;

#[cfg(target_os = "android")]
mod mobile;

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

        let current_rate = rate.get().cloned();
        match display.get().cloned() {
            Screen::Balance => balance_screen(display, currency, current_rate),
            Screen::Send => send_screen(display, currency, current_rate),
            Screen::Request(request) => request_screen(request, display, currency, current_rate),
        }
    })
    .align_items(AlignItems::Stretch)
    .justify_content(JustifyContent::SpaceEvenly)
    .flex_grow(1.)
    .padding(Padding::from(Dimension::Points(16.dp())).top(Dimension::Points(40.dp())))
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
