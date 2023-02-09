mod balance;
pub use balance::balance_screen;

mod send;
pub use send::send_screen;

mod request;
pub use request::request_screen;

#[derive(Clone, Copy)]
pub enum RequestScreen {
    Share,
    Amount,
}

#[derive(Clone, Copy)]
pub enum Screen {
    Balance,
    Send,
    Request(RequestScreen),
}
