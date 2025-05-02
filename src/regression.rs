// Regression Module -- Takes slices of x and y values to calculate and return slope and intercept, along with added capabilities for calculating r-squared
pub fn lin_regression(x: &[f64], y: &[f64]) -> Option<(f64, f64)> {
    // Linear Regression Function:
    // Do a linear regression using x and y points and return slope + intercept
    //
    // Inputs:
    // • x - slice of x (degrees from a degree distribution)
    // • y - slice of y (counts from a degree distribution)
    //
    // Outputs:
    // • Option enum type with tuple of slope and intercept
    if x.len() != y.len() || x.is_empty() {
        return None;
    } // if degrees and counts don't match or one is empty, return None immediately

    let n = x.len() as f64;
    let mean_x = (x.iter().sum::<f64>()) / n;
    let mean_y = (y.iter().sum::<f64>()) / n;

    let mut num = 0.0;
    let mut den = 0.0;

    for (x1, y1) in x.iter().zip(y.iter()) {
        num += (x1 - mean_x) * (y1 - mean_y);
        den += (x1 - mean_x).powi(2);
    } // zip x and y values, iterate through them in pairs, and use for calculating numerator and denominator

    if den == 0.0 {
        return None;
    } // if denominator is 0, then return None because can't have denominator of 0

    let slope = num / den;
    let intercept = mean_y - slope * mean_x;

    Some((slope, intercept)) // return Some() type with slope and intercept tuple
}

pub fn r_squared(x: &[f64], y: &[f64], slope: f64, intercept: f64) -> Option<f64> {
    // R Squared Function:
    // Take x and y points along with slope and intercept to calculate R-Squared metric to indicate fit of regression
    //
    // Inputs:
    // • x - slice of x (degrees from a degree distribution)
    // • y - slice of y (counts from a degree distribution)
    // • slope - f64 value indicating slope from linear regression of x and y
    // • intercept - f64 value indicating intercept from linear regression of x and y
    //
    // Outputs:
    // • Option enum type with f64 value (R-Squared value)
    if x.len() != y.len() || x.is_empty() {
        return None;
    }

    let mean_y = (y.iter().sum::<f64>()) / y.len() as f64;
    let mut sst = 0.0;
    let mut sse = 0.0;

    for (x2, y2) in x.iter().zip(y.iter()) {
        let predicted_val = intercept + slope * x2;
        sse += (y2 - predicted_val).powi(2);
        sst += (y2 - mean_y).powi(2);
    } // zipping x and y and iterating through them, coming up with predicted value,
      // and then calculating and incrementing SSE (sum of squared errors) and SST (sum of squared total)

    if sst == 0.0 {
        return None;
    } // if SST = 0, then return None because SST is our denominator, so can't divide by undefined.

    let rsq = 1.0 - (sse / sst); // 1 - (SSE/SST) calculation for R Squared value
    Some(rsq)
}
