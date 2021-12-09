use std::error::Error;
use std::fs::File;
use std::iter::Iterator;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::io;
use std::collections::HashSet;

use serde::{Serialize, Deserialize};
use structopt::StructOpt;

use solana_sdk::pubkey::Pubkey;
use solana_client::rpc_client::RpcClient;

mod defaults {
  pub const RPC_URL: &str = "api.mainnet-beta.solana.com";
  pub const MAPPING_KEY: &str = "\
    AHtgzX45WTKfkPG53L6WYhGEXwQkN1BVknET3sVsLL8J\
  ";
  pub const MARKETS_JSON: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/markets.json",
  );
  pub const MARKETS_URL: &str = "\
    https://raw.githubusercontent.com/\
    project-serum/serum-ts/master/packages/\
    serum/src/markets.json";
}

#[derive(Debug, StructOpt)]
#[structopt(name="pyth-serum-client")]
pub struct Client {

  #[structopt(short, long="outdir", help="\
    Path to output dir containing json files with serum-pyth account info, \
    one per product, named like sol_usd.json. \
  ")]
  pub output_dir: PathBuf,

  #[structopt(short, long="binary", help="\
    Include binary outputs formatted as a serum-pyth program inputs, \
    named like 'sol_usd.bin'. \
  ")]
  pub include_binary: bool,

  #[structopt(short, long="symbol", help="\
    Ticker symbol(s) mapping to 'name' in markets.json \
    and 'symbol' in Pyth product accounts, e.g, 'SOL/USD'. \
    Defaults to all pyth symbols unless --market is provided. \
  ")]
  pub symbols: Vec<String>,

  #[structopt(long="market", help="\
    Pubkey of Serum market account, owned by the dex program. \
    Implies a --symbol. If not set, inferred from --symbol and markets.json. \
  ")]
  pub market_key: Option<Pubkey>,

  #[structopt(long="mapping", default_value=defaults::MAPPING_KEY, help="\
    Pyth mapping account referencing all pyth products. \
    Default value from https://pyth.network/developers/accounts \
  ")]
  pub mapping_key: Pubkey,

  #[structopt(long="rpc", default_value=defaults::RPC_URL, help="\
    RPC URL for querying account data. \
  ")]
  pub rpc_url: String,

  #[structopt(long, default_value=defaults::MARKETS_JSON, help="\
    Json file with a symbol, market account key, and dex program key \
    for each product, in the format of serum-ts/serum/src/markets.json \
  ")]
  pub markets_json: PathBuf,

  #[structopt(long, default_value=defaults::MARKETS_URL, help="\
    URL from which to fetch --markets-json if --fetch-markets is set. \
  ")]
  pub markets_url: String,

  #[structopt(long, help="\
    Fetch from --markets-url and write to --markets-json. \
  ")]
  pub fetch_markets: bool,
}

/// A self-contained [`solana_sdk::keyed_account::KeyedAccount`].
#[derive(Debug, Default, Serialize)]
#[serde(rename_all="snake_case")]
pub struct Account {
  pub key: Pubkey,
  pub is_signer: bool,
  pub is_writable: bool,
  #[serde(flatten)]
  pub acc: solana_sdk::account::Account,
}

/// Input accounts to the `serum-pyth` program.
#[derive(Debug, Default, Serialize)]
pub struct AccountGroup {
  pub payer: Account,
  pub pyth_price: Account,
  pub serum_program: Account,
  pub serum_market: Account,
  pub serum_bids: Account,
  pub serum_asks: Account,
  pub quote_token: Account,
  pub base_token: Account,
  pub sysvar_clock: Account,
  pub pyth_program: Account,
}

/// A market dict deserialized from markets.json.
#[derive(Debug, Deserialize)]
pub struct MarketJson {
  #[serde(rename="programId")]
  program_id: Pubkey,
  address: Pubkey,
  deprecated: bool,
  name: String,
}

pub type BoxResult<T> = Result<T, Box<dyn Error>>;

impl MarketJson {
  pub fn from_path<P: AsRef<Path>>(path: P) -> BoxResult<Vec<Self>> {
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);
    let market = serde_json::from_reader(reader)?;
    Ok(market)
  }
}

fn curl(url: &str, path: &Path) -> io::Result<Output> {
  Command::new("curl")
    .args(["-v", url])
    .stdout(File::create(&path).unwrap())
    .output()
}

impl Client {

  // re-export
  pub fn from_args() -> Self {
    StructOpt::from_args()
  }

  fn symbol_set(&self) -> HashSet<String> {
    HashSet::from_iter(self.symbols.iter().cloned())
  }

  fn matches_market_json(&self, mj: &MarketJson) -> bool {
    let symbols = self.symbol_set();
    match mj.deprecated {
      true => false,
      false => match self.market_key {
        Some(key) => key == mj.address,
        None => symbols.is_empty() || symbols.contains(&mj.name)
      }
    }
  }

  pub fn load_accounts(&self) -> BoxResult<Vec<AccountGroup>> {

    let path = self.markets_json.as_path();
    if self.fetch_markets {
      curl(self.markets_url.as_str(), path)?;
    }

    let mut accounts: Vec<AccountGroup> = vec![];
    for mj in MarketJson::from_path(path)? {
      if self.matches_market_json(&mj) {

      }
    }
    let rpc = RpcClient::new(self.rpc_url.clone());

    Ok(accounts)
  }
}
