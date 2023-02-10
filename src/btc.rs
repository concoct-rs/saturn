use std::str::FromStr;

use bdk::bitcoin::util::bip32::ExtendedPrivKey;
use bdk::bitcoin::Network;
use bdk::blockchain::{Blockchain, ElectrumBlockchain};
use bdk::database::{BatchDatabase, MemoryDatabase};
use bdk::template::Bip84;
use bdk::wallet::export::FullyNodedExport;
use bdk::{Balance, KeychainKind, SignOptions, SyncOptions, Wallet};

use bdk::electrum_client::Client;
use bdk::wallet::AddressIndex;
use bitcoin::util::bip32;

use bitcoin::{Address, Transaction};

pub fn build_signed_tx<D: BatchDatabase>(
    wallet: &Wallet<D>,
    recipient_address: &str,
    amount: u64,
) -> Transaction {
    // Create a transaction builder
    let mut tx_builder = wallet.build_tx();

    let to_address = Address::from_str(recipient_address).unwrap();

    // Set recipient of the transaction
    tx_builder.set_recipients(vec![(to_address.script_pubkey(), amount)]);

    // Finalise the transaction and extract PSBT
    let (mut psbt, _) = tx_builder.finish().unwrap();

    // Sign the above psbt with signing option
    wallet.sign(&mut psbt, SignOptions::default()).unwrap();

    // Extract the final transaction
    psbt.extract_tx()
}

pub struct MyWallet {
    blockchain: ElectrumBlockchain,
    wallet: Wallet<MemoryDatabase>,
}

impl MyWallet {
    pub fn new() -> Self {
        let xpriv = "tprv8ZgxMBicQKsPcx5nBGsR63Pe8KnRUqmbJNENAfGftF3yuXoMMoVJJcYeUw5eVkm9WBPjWYt6HMWYJNesB5HaNVBaFc1M6dRjWSYnmewUMYy";
        let xpriv = bip32::ExtendedPrivKey::from_str(xpriv).unwrap();

        let network = Network::Testnet;
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
}
