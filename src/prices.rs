use super::*;

/// Struct holding the last 24 hours of prices
#[derive(Debug)]
pub struct Prices {
    /// What Trading pair the prices reference
    pub ticker: String,
    /// Array holding the prices for the last 24 hours, stored as a float
    pub values: [f64; NUM_HOURS],
    /// Mean of the set
    pub mean: f64,
    /// Standard deviation of the set
    pub stdev: f64,
}

impl Prices {
    /// Create a new struct holding the prices from the trading pair and retrieved values
    pub fn new(trading_pair: &str, values: [f64; NUM_HOURS]) -> Result<Prices> {
        let mean = Self::mean(&values);
        Ok(Prices {
            ticker: trading_pair.to_string(),
            values,
            mean,
            stdev: Self::standard_deviation(mean, &values),
        })
    }

    /// Computes the average
    fn mean(values: &[f64]) -> f64 {
        values.iter().sum::<f64>() / NUM_HOURS as f64
    }

    /// Compute the standard deviation of the set
    pub fn standard_deviation(mean: f64, values: &[f64]) -> f64 {
        // Standard deviation formula is sqrt(SUM((x_i - mu)^2)/N)
        // where x_i is each individual data point, mu is the mean, and N is the number of samples

        // Compute SUM((x_i - mu)^2)
        let numerator = values
            .iter()
            .map(|x_i| {
                let diff = x_i - mean;
                diff * diff
            })
            .collect::<Vec<f64>>()
            .iter()
            .sum::<f64>();
        let div = numerator / NUM_HOURS as f64;
        div.sqrt()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // Verify our standard deviation against Python's implementation
    fn standard_deviation() {
        // Generated from python with random_floats = [random.uniform(0.0, 1.0) for _ in range(24)]
        let samples = [
            0.4446292681363997,
            0.08622604657288269,
            0.6681295425177487,
            0.9925437907133752,
            0.22926441360128558,
            0.4945593219621459,
            0.279208761371058,
            0.21251040762049533,
            0.07209038891307085,
            0.29344331390317535,
            0.9766845304126169,
            0.4590230368587641,
            0.67967388638089,
            0.7488823589574871,
            0.9549562795490785,
            0.14736190860613185,
            0.3414857392230989,
            0.3110228106481253,
            0.3317744278127088,
            0.30751689201737287,
            0.1867191384387742,
            0.20877219637411193,
            0.5809838041458467,
            0.10103381024949076,
        ];
        let prices = Prices::new("BTCUSD", samples).unwrap();
        // Computed with statistics.stdev(random_floats)
        let python_stdev = 0.27808248618893017;
        assert_eq!(prices.stdev, python_stdev);
    }
}
