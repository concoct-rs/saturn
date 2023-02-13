use crate::PRIVATE_KEY;
use bdk::bitcoin::util::bip32::ExtendedPrivKey;
use bdk::bitcoin::Network;
use bdk::blockchain::ElectrumBlockchain;
use bdk::electrum_client::Client;
use bdk::template::Bip84;
use bdk::wallet::AddressIndex;
use bdk::{Balance, KeychainKind};
use bitcoin::{Address, PrivateKey};
use std::str::FromStr;
use std::sync::mpsc;
use tokio::sync::oneshot;
use tokio::task;

enum Message {
    Address { tx: oneshot::Sender<Address> },
    Balance { tx: oneshot::Sender<Balance> },
    Transactions { tx: oneshot::Sender<Vec<i64>> },
}

#[derive(Clone)]
pub struct Wallet {
    tx: mpsc::Sender<Message>,
}

impl Wallet {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel();

        task::spawn_blocking(move || {
            let private_key = PrivateKey::from_str(PRIVATE_KEY).unwrap();
            let xpriv =
                ExtendedPrivKey::new_master(Network::Bitcoin, &private_key.to_bytes()).unwrap();

            let network = Network::Bitcoin;
            let electrum_url = "ssl://electrum.blockstream.info:60002";
            let _blockchain = ElectrumBlockchain::from(Client::new(electrum_url).unwrap());

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
                        tx.send(address_info.address).unwrap();
                    }
                    Message::Balance { tx } => {
                        let balance = wallet.get_balance().unwrap();
                        tx.send(balance).unwrap();
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
                        tx.send(transactions).unwrap();
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
