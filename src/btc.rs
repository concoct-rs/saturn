use crate::PRIVATE_KEY;
use bdk::bitcoin::util::bip32::ExtendedPrivKey;
use bdk::bitcoin::Network;
use bdk::blockchain::ElectrumBlockchain;
use bdk::electrum_client::Client;
use bdk::sled::Tree;
use bdk::template::Bip84;
use bdk::wallet::AddressIndex;
use bdk::{Balance, KeychainKind, SyncOptions, TransactionDetails, Wallet};
use bitcoin::{Address, PrivateKey};
use std::str::FromStr;
use std::sync::mpsc;
use tokio::sync::oneshot;
use tokio::task;

pub enum Message {
    Address { tx: oneshot::Sender<Address> },
    Balance { tx: oneshot::Sender<Balance> },
    Transactions { tx: oneshot::Sender<Vec<i64>> },
}

pub fn wallet() -> mpsc::Sender<Message> {
    let (tx, rx) = mpsc::channel();

    task::spawn_blocking(move || {
        let wallet = MyWallet::new();
        while let Ok(msg) = rx.recv() {
            match msg {
                Message::Address { tx } => {
                    let address = wallet.get_address();
                    tx.send(address).unwrap();
                }
                Message::Balance { tx } => {
                    let balance = wallet.get_balance();
                    tx.send(balance).unwrap();
                }
                Message::Transactions { tx } => {
                    let transactions = wallet
                        .get_transactions()
                        .into_iter()
                        .map(|transaction| transaction.received as i64 - transaction.sent as i64)
                        .collect();
                    tx.send(transactions).unwrap();
                }
            }
        }
    });

    tx
}

pub struct MyWallet {
    blockchain: ElectrumBlockchain,
    wallet: Wallet<Tree>,
}

impl MyWallet {
    pub fn new() -> Self {
        let private_key = PrivateKey::from_str(PRIVATE_KEY).unwrap();
        let xpriv = ExtendedPrivKey::new_master(Network::Bitcoin, &private_key.to_bytes()).unwrap();

        let network = Network::Bitcoin;
        let electrum_url = "ssl://electrum.blockstream.info:60002";
        let blockchain = ElectrumBlockchain::from(Client::new(electrum_url).unwrap());

        let mut path = std::env::current_dir().unwrap();
        path.push("db");
        dbg!(&path);

        let db = sled::open(path).unwrap();
        let wallet = Wallet::new(
            Bip84(xpriv, KeychainKind::External),
            Some(Bip84(xpriv, KeychainKind::Internal)),
            network,
            db.open_tree("wallet").unwrap(),
        )
        .unwrap();

        Self { blockchain, wallet }
    }

    pub fn get_balance(&self) -> Balance {
        self.wallet
            .sync(&self.blockchain, SyncOptions::default())
            .unwrap();

        self.wallet.get_balance().unwrap()
    }

    pub fn get_address(&self) -> Address {
        self.wallet.get_address(AddressIndex::New).unwrap().address
    }

    pub fn get_transactions(&self) -> Vec<TransactionDetails> {
        self.wallet.list_transactions(false).unwrap()
    }
}
