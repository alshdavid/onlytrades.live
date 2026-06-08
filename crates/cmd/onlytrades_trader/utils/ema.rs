use std::collections::VecDeque;

use kit_ctrader_socket::*;
use ta::Next;
use ta::indicators::ExponentialMovingAverage;
use ta::indicators::RelativeStrengthIndex;

pub trait ExponentialMovingAverageExt {
  fn calculate(
    &mut self,
    items: &VecDeque<Trendbar>,
  ) -> Vec<f64>;
}

impl ExponentialMovingAverageExt for ExponentialMovingAverage {
  fn calculate(
    &mut self,
    series: &VecDeque<Trendbar>,
  ) -> Vec<f64> {
    let prices = series.iter().map(|v| v.close_price()).collect::<Vec<i64>>();

    prices
      .iter()
      .map(|&price| self.next(price as f64))
      .collect::<Vec<f64>>()
  }
}

impl ExponentialMovingAverageExt for RelativeStrengthIndex {
  fn calculate(
    &mut self,
    series: &VecDeque<Trendbar>,
  ) -> Vec<f64> {
    let prices = series.iter().map(|v| v.close_price()).collect::<Vec<i64>>();

    prices
      .iter()
      .map(|&price| self.next(price as f64))
      .collect::<Vec<f64>>()
  }
}
