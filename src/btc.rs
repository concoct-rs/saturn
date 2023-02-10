use crate::PRIVATE_KEY;
use bdk::bitcoin::util::bip32::ExtendedPrivKey;
use bdk::bitcoin::Network;
use bdk::blockchain::ElectrumBlockchain;
use bdk::database::MemoryDatabase;
use bdk::electrum_client::Client;
use bdk::template::Bip84;

use bdk::wallet::AddressIndex;
use bdk::{Balance, KeychainKind, SyncOptions, TransactionDetails, Wallet};

use bitcoin::{Address, PrivateKey};
use std::str::FromStr;

pub struct MyWallet {
    blockchain: ElectrumBlockchain,
    wallet: Wallet<MemoryDatabase>,
}

impl MyWallet {
    pub fn new() -> Self {
        let private_key = PrivateKey::from_str(PRIVATE_KEY).unwrap();
        let xpriv = ExtendedPrivKey::new_master(Network::Bitcoin, &private_key.to_bytes()).unwrap();

        let network = Network::Bitcoin;
        let electrum_url = "ssl://electrum.blockstream.info:60002";
        let blockchain = ElectrumBlockchain::from(Client::new(electrum_url).unwrap());

        let wallet = Wallet::new(
            Bip84(xpriv, KeychainKind::External),
            Some(Bip84(xpriv, KeychainKind::Internal)),
            network,
            MemoryDatabase::default(),
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
