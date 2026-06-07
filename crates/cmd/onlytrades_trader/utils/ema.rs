pub struct EmaEngine;

impl EmaEngine {
  pub fn calculate(
    prices: &[i64], // Assuming close_price() returns raw i64 values
    period: usize,
  ) -> Option<i64> {
    if prices.len() < period {
      return None;
    }

    // FIX 2: Perform calculations using float math so fractions work
    let k = 2.0 / (period as f64 + 1.0);

    // FIX 3: Seed SMA using floats
    let initial_sma: f64 =
      prices.iter().take(period).map(|&p| p as f64).sum::<f64>() / (period as f64);

    let mut ema = initial_sma;

    // Iterates forward chronologically from the 21st bar onward
    for &price in prices.iter().skip(period) {
      ema = (price as f64) * k + ema * (1.0 - k);
    }

    Some(ema as i64)
  }
}
