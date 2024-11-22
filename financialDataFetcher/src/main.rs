use serde::Deserialize;
use std::fs::File;
use std::io::Write;
use std::thread;
use std::time::Duration;


#[derive(Deserialize)]
struct Bitcoin{}

#[derive(Deserialize)]
struct Ethereum{}

#[derive(Deserialize)]
struct SP500{}

trait Pricing{
    fn fetchprice(&self) -> Result<f64, String>;
}

 impl Pricing for Bitcoin{
      fn fetchprice(&self) -> Result<f64, String> {
       let response = ureq::get("https://api.coindesk.com/v1/bpi/currentprice/BTC.json")
            .call()
            .map_err(|e| e.to_string())?
            .into_string()
            .map_err(|e| e.to_string())?;
        let json: serde_json::Value = serde_json::from_str(&response).map_err(|e| e.to_string())?;
        json["bpi"]["USD"]["rate_float"]
            .as_f64()
            .ok_or_else(|| "Failed to parse bitcorn".to_string())
      }
 }

 impl Pricing for Ethereum{
    fn fetchprice(&self) -> Result<f64, String> {
     let response = ureq::get("https://api.coingecko.com/api/v3/simple/price?ids=ethereum&vs_currencies=usd")
            .call()
            .map_err(|e| e.to_string())?
            .into_string()
            .map_err(|e| e.to_string())?;
      let json: serde_json::Value = serde_json::from_str(&response).map_err(|e| e.to_string())?;
      json["ethereum"]["usd"]
            .as_f64()
            .ok_or_else(|| "Failed to parse ethereum".to_string())
    }
}

impl Pricing for SP500{
    fn fetchprice(&self) -> Result<f64, String> {
     let response = ureq::get("https://query1.finance.yahoo.com/v8/finance/chart/^GSPC") 
            .call()
            .map_err(|e| e.to_string())?
            .into_string()
            .map_err(|e| e.to_string())?;
        let json: serde_json::Value = serde_json::from_str(&response).map_err(|e| e.to_string())?;     
        json["chart"]["result"][0]["meta"]["regularMarketPrice"]
            .as_f64()
            .ok_or_else(|| "Failed to parse S&P 500".to_string())
    }
}

fn save_to_file(filename: &str, data: f64){
    let mut file = File::create(filename).expect("Unable to create file");
    writeln!(file,"{}",data).expect("Unable to write data");
}

fn main(){
    let bitcoin = Bitcoin {};
    let ethereum = Ethereum {};
    let sp500 = SP500{};

    println!("Starting...");

    loop{
        match bitcoin.fetchprice(){
            Ok(price) => {
                save_to_file("bitcoin_price.txt", price);
                println!("Bitcoin price: ${:.2}", price);
            }
            Err(err) => println!("Failed to fath bitcoin price: {}",err),
        }
        match ethereum.fetchprice(){
            Ok(price) => {
                save_to_file("ethereum_price.txt", price);
                println!("Ethereum price: ${:.2}", price);
            }
            Err(err) => println!("Failed to fath ethereum price: {}",err),
        }
        match sp500.fetchprice(){
            Ok(price) => {
                save_to_file("sp500_price.txt", price);
                println!("S&P 500 price: ${:.2}", price);
            }
            Err(err) => println!("Failed to fath S&P 500 price: {}",err),
        }

        thread::sleep(Duration::from_secs(10));
    }
}