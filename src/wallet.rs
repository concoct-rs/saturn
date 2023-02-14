use crate::PRIVATE_KEY;
use bdk::bitcoin::util::bip32::ExtendedPrivKey;
use bdk::bitcoin::Network;
use bdk::blockchain::ElectrumBlockchain;
use bdk::electrum_client::Client;
use bdk::template::Bip84;
use bdk::wallet::AddressIndex;
use bdk::{Balance, KeychainKind};
use bitcoin::{Address, PrivateKey};
use std::fmt::Debug;
use std::str::FromStr;
use std::sync::mpsc;
use std::time::Duration;
use tokio::sync::oneshot;
use tokio::task;
use tokio::time::interval;
use tracing::warn;

enum Message {
    Address { tx: oneshot::Sender<Address> },
    Balance { tx: oneshot::Sender<Balance> },
    Transactions { tx: oneshot::Sender<Vec<i64>> },
    Sync,
}

#[derive(Clone, Debug)]
pub struct Wallet {
    tx: mpsc::Sender<Message>,
}

impl Wallet {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel();

        let task_tx = tx.clone();
        task::spawn(async move {
            let mut interval = interval(Duration::from_secs(60));

            loop {
                if task::block_in_place(|| task_tx.send(Message::Sync).is_err()) {
                    break;
                }

                interval.tick().await;
            }
        });

        task::spawn_blocking(move || {
            let private_key = PrivateKey::from_str(PRIVATE_KEY).unwrap();
            let xpriv =
                ExtendedPrivKey::new_master(Network::Bitcoin, &private_key.to_bytes()).unwrap();

            let network = Network::Bitcoin;
            let electrum_url = "ssl://electrum.blockstream.info:60002";
            let blockchain = ElectrumBlockchain::from(Client::new(electrum_url).unwrap());

            let mut path = std::env::current_dir().unwrap();
            path.push("db");

            let db = sled::open(path).unwrap();
            let wallet = bdk::Wallet::new(
                Bip84(xpriv, KeychainKind::External),
                Some(Bip84(xpriv, KeychainKind::Internal)),
                network,
                db.open_tree("wallet").unwrap(),
            )
            .unwrap();

            while let Ok(msg) = rx.recv() {
                match msg {
                    Message::Address { tx } => {
                        let address_info = wallet.get_address(AddressIndex::New).unwrap();
                        send_or_warn(tx, address_info.address);
                    }
                    Message::Balance { tx } => {
                        let balance = wallet.get_balance().unwrap();
                        send_or_warn(tx, balance);
                    }
                    Message::Transactions { tx } => {
                        let transactions = wallet
                            .list_transactions(false)
                            .unwrap()
                            .into_iter()
                            .map(|transaction| {
                                transaction.received as i64 - transaction.sent as i64
                            })
                            .collect();
                        send_or_warn(tx, transactions);
                    }
                    Message::Sync => {
                        wallet.sync(&blockchain, Default::default()).unwrap();
                    }
                }
            }
        });

        Self { tx }
    }

    pub async fn address(self) -> Address {
        let (tx, rx) = oneshot::channel();
        self.tx.send(Message::Address { tx }).unwrap();

        rx.await.unwrap()
    }

    pub async fn balance(self) -> Balance {
        let (tx, rx) = oneshot::channel();
        self.tx.send(Message::Balance { tx }).unwrap();

        rx.await.unwrap()
    }

    pub async fn transactions(self) -> Vec<i64> {
        let (tx, rx) = oneshot::channel();
        self.tx.send(Message::Transactions { tx }).unwrap();

        rx.await.unwrap()
    }
}

fn send_or_warn<T: Debug>(tx: oneshot::Sender<T>, value: T) {
    if let Err(error) = tx.send(value) {
        warn!("Dropped message: {:?}", error);
    }
}
