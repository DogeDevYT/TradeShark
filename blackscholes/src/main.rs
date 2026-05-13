mod black_scholes;
mod data;

use black_scholes::BS;

#[tokio::main]
async fn main() {
    let ticker = "AAPL";

    let spot_price = data::get_current_price(ticker).await;
    let volatility = data::get_historical_volatility(ticker).await;

    println!("AAPL spot price:  {:.2}", spot_price);
    println!("Historical vol:   {:.4}", volatility);

    // example: price a call option 5% out of the money, 30 days to expiry
    let strike_price = spot_price * 1.05;
    let time_to_maturity = 30.0 / 365.0;
    let risk_free_rate = 0.053; // approximate current 3-month T-bill rate

    let bs = BS::new(spot_price, strike_price, risk_free_rate, time_to_maturity, volatility);

    println!("Strike price:     {:.2}", strike_price);


    for days in (0..=30).rev() 
    {
        let t_remaining = days as f64 / 365.0;
        let bs_test = BS::new(spot_price, strike_price, risk_free_rate, t_remaining, volatility);
        println!("Days: {}, Price: {:.4}", days, bs_test.call_option_price());
    }
}
