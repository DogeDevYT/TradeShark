use yahoo_finance_api as yahoo;

pub async fn get_historical_volatility(ticker: &str) -> f64 
{
    let provider = yahoo::YahooConnector::new().unwrap();

    //fetch 1 year of daily data
    let response = provider
        .get_quote_range(ticker, "1d", "1y")
        .await
        .unwrap();

    let quotes = response.quotes().unwrap();

    //calculate daily log returns
    let returns: Vec<f64> = quotes
        .windows(2)
        .map(|w| (w[1].close / w[0].close).ln())
        .collect();
    
    //mean return
    let mean = returns.iter().sum::<f64>() / returns.len() as f64;

    //standard deviation of returns
    let variance = returns.iter()
        .map(|r| (r - mean).powi(2))
        .sum::<f64>() / (returns.len() as f64 - 1.0) as f64;
    

    let daily_volatility = variance.sqrt();

    // annualize by multiplying by sqrt(252) trading days
    daily_volatility * (252.0_f64).sqrt() //no semicolon since this is the return value of the function
}

pub async fn get_current_price(ticker: &str) -> f64 
{
    let provider = yahoo::YahooConnector::new().unwrap();
    let response = provider.get_latest_quotes(ticker, "1d").await.unwrap();
    let quote = response.last_quote().unwrap();
    quote.close
}

