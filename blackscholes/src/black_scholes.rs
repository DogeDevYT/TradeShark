use statrs::distribution::{Normal, ContinuousCDF};

pub struct BS 
{
    price: f64, // w - price of option
    security_price: f64, // x - price of underlying security
    strike_price: f64, // K - price of strike
    risk_free_interest_rate: f64, // r - risk free interest rate
    time_to_maturity: f64, // this will represent T - t i.e. time of maturity minus current time
    volatility: f64,  // sigma - volatility of the underlying security
}

impl BS 
{
    /*
    Constructor for the BS type using:

    - security_price: the current price of the underlying security (S)
    - strike_price: the strike price of the option (K)
    - risk_free_interest_rate: the risk-free interest rate (r)
    - time_to_maturity: the time to maturity of the option (T - t)
    - volatility: the volatility of the underlying security (sigma)

    */
    pub fn new (
        security_price: f64, 
        strike_price: f64, 
        risk_free_interest_rate: f64, 
        time_to_maturity: f64, 
        volatility: f64
    ) -> Self 
    {
        BS {
            price: 0.0, // This will be calculated later using the call_option_price method
            security_price,
            strike_price,
            risk_free_interest_rate,
            time_to_maturity,
            volatility,
        }
    }

    /*
    This function represents the "solved" Black-Scholes formula for a call option. Specifically, the 
    variable "d_1".
    */

    pub fn call_option_d1(&self) -> f64 
    {
        return ((self.security_price / self.strike_price).ln() + 
        (self.risk_free_interest_rate + self.volatility * self.volatility / 2.0) * 
        self.time_to_maturity) / (self.volatility * self.time_to_maturity.sqrt());
    }

    /*
    This function represents the "solved" Black-Scholes formula for a call option. Specifically, the 
    variable "d_2".
    */
    pub fn call_option_d2(&self) -> f64 
    {
        return self.call_option_d1() - self.volatility * self.time_to_maturity.sqrt();
    }

    /*
    This function prices a CALL option using the solved Black-Scholes formula. 
    It uses the cumulative distribution function (CDF) of the standard normal distribution 
    to calculate the price of the call option based on the parameters provided in the struct. 
    The formula is derived from the Black-Scholes model and is used to determine the fair price of a call option given 
    the underlying security's price, strike price, risk-free interest rate, time to maturity, and volatility.
    */
    pub fn call_option_price(&self) -> f64 
    {
        let normal = Normal::new(0.0, 1.0).unwrap(); // Standard normal distribution

        // Calculate d1 and d2 once to save CPU cycles
        let d1 = self.call_option_d1();
        let d2 = d1 - self.volatility * self.time_to_maturity.sqrt();
        
        let discounted_strike = self.strike_price * (-self.risk_free_interest_rate * self.time_to_maturity).exp();
        
        self.security_price * normal.cdf(d1) - discounted_strike * normal.cdf(d2)
    }
}

/*
Lets build some unit tests for this call option pricing using black-scholes
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_call_option_price() {
        let bs = BS::new(100.0, 105.0, 0.05, 1.0, 0.2);
        let price = bs.call_option_price();
        // known Black-Scholes value for these parameters
        assert!((price - 8.0214).abs() < 0.001);
    }

    #[test]
    fn test_d1() {
        let bs = BS::new(100.0, 105.0, 0.05, 1.0, 0.2);
        assert!((bs.call_option_d1() - 0.10609).abs() < 0.001);
    }

    #[test]
    fn test_d2() {
        let bs = BS::new(100.0, 105.0, 0.05, 1.0, 0.2);
        assert!((bs.call_option_d2() - (-0.09395)).abs() < 0.001);
    }
}