use super::show::ShowCosmosKeyCmd;
use crate::application::APP;
use abscissa_core::{clap::Parser, Application, Command, Runnable};
use k256::pkcs8::ToPrivateKey;
use rand_core::OsRng;
use signatory::FsKeyStore;
use std::path;

#[derive(Command, Debug, Default, Parser)]
#[clap(
    long_about = "DESCRIPTION \n\n Create a new Cosmos Key.\n This command creates a new Cosmos key when provided an overwrite option, which if set to true, overwrites\n an existing key in the keystore with similar keyname."
)]
pub struct AddCosmosKeyCmd {
    /// Cosmos keyname, takes a String
    pub name: String,

    #[clap(short, long)]
    /// Overwrite key with similar name in the keystore when set to true. Takes a Boolean.
    pub overwrite: bool,
}

// `steward keys cosmos add [name]`
// - [name] required; keyname
impl Runnable for AddCosmosKeyCmd {
    fn run(&self) {
        let config = APP.config();
        let keystore = path::Path::new(&config.keystore);
        let keystore = FsKeyStore::create_or_open(keystore).expect("Could not open keystore");
        let name = self.name.parse().expect("Could not parse name");
        if let Ok(_info) = keystore.info(&name) {
            if !self.overwrite {
                eprintln!("Key already exists, exiting.");
                return;
            }
        }

        let mnemonic = bip32::Mnemonic::random(&mut OsRng, Default::default());
        eprintln!("**Important** record this bip39-mnemonic in a safe place:");
        println!("{}", mnemonic.phrase());

        let seed = mnemonic.to_seed("");

        let path = config.cosmos.key_derivation_path.clone();
        let path = path
            .parse::<bip32::DerivationPath>()
            .expect("Could not parse derivation path");

        let key = bip32::XPrv::derive_from_path(seed, &path).expect("Could not derive key");
        let key = k256::SecretKey::from(key.private_key());
        let key = key
            .to_pkcs8_der()
            .expect("Could not PKCS8 encod private key");

        keystore.store(&name, &key).expect("Could not store key");

        let name = name.to_string();
        let show_cmd = ShowCosmosKeyCmd { name };
        show_cmd.run();
    }
}
