use super::{RequestScreen, Screen};
use crate::btc::MyWallet;
use crate::currency::{currency_input, Currency};
use crate::full_width_button;
use concoct::composable::state::State;
use concoct::composable::{material::Button, Text};
use concoct::composable::{state, Container};
use concoct::dimension::{DevicePixels, Size};
use concoct::modify::ModifyExt;
use concoct::{Modifier, View};
use image::png::PngEncoder;
use image::Rgb;
use qrcode::QrCode;
use rust_decimal::Decimal;
use skia_safe::{Data, Image};
use taffy::style::{AlignItems, Dimension, JustifyContent};

#[track_caller]
pub fn request_screen(
    request: RequestScreen,
    display: State<Screen>,
    currency: State<Currency>,
    rate: Decimal,
    wallet: State<MyWallet>,
) {
    Container::build_column(move || {
        let amount = state(|| None::<String>);
        let address = state(move || wallet.get().as_ref().get_address());

        match request {
            RequestScreen::Share => {
                Button::new(|| Text::new("Back"))
                    .on_press(move || {
                        *display.get().as_mut() = Screen::Balance;
                    })
                    .view();

                let label = if let Some(amount) = amount.get().cloned() {
                    amount
                } else {
                    String::from("Add amount")
                };
                full_width_button(label, move || {
                    *display.get().as_mut() = Screen::Request(RequestScreen::Amount);
                });

                Container::build_column(move || {
                    let qr_uri = address.get().as_ref().to_qr_uri();
                    Container::build_row(|| {})
                        .size(Size::from(Dimension::Points(200.dp())))
                        .modifier(Modifier.draw(move |layout, canvas| {
                            let qr_code = QrCode::new(&qr_uri).unwrap();
                            let image_buffer = qr_code
                                .render::<Rgb<u8>>()
                                .min_dimensions(layout.size.width as _, layout.size.height as _)
                                .build();

                            let mut png_data = Vec::new();
                            PngEncoder::new(&mut png_data)
                                .encode(
                                    &image_buffer,
                                    image_buffer.width(),
                                    image_buffer.height(),
                                    image::ColorType::Rgb8,
                                )
                                .unwrap();

                            let image = Image::from_encoded(Data::new_copy(&png_data)).unwrap();

                            canvas.draw_image(image, (layout.location.x, layout.location.y), None);
                        }))
                        .view();
                })
                .flex_grow(1.)
                .align_items(AlignItems::Center)
                .view();

                #[cfg(target_os = "android")]
                full_width_button(address.get().as_ref().to_string(), move || {
                    use android_intent::{with_current_env, Action, Extra, Intent};

                    with_current_env(|env| {
                        Intent::new(env, Action::Send)
                            .with_type("text/plain")
                            .with_extra(Extra::Text, address.get().as_ref().to_string())
                            .into_chooser()
                            .start_activity()
                            .unwrap()
                    })
                });

                #[cfg(not(target_os = "android"))]
                full_width_button(address.get().as_ref().to_string(), || {});
            }
            RequestScreen::Amount => {
                let new_amount = state(|| String::from("0"));

                Button::new(|| Text::new("Back"))
                    .on_press(move || {
                        *display.get().as_mut() = Screen::Request(RequestScreen::Share);
                    })
                    .view();

                currency_input(new_amount, currency, rate);

                full_width_button("Request", move || {
                    *amount.get().as_mut() = Some(new_amount.get().cloned());
                    *display.get().as_mut() = Screen::Request(RequestScreen::Share);
                });
            }
        }
    })
    .align_items(AlignItems::Stretch)
    .justify_content(JustifyContent::SpaceBetween)
    .flex_grow(1.)
    .view()
}
