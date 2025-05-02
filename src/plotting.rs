// Plotting Module -- take slope and intercept and log of degree + counts to create log-log plot for power-law distribution analysis
use plotters::prelude::*;

pub fn log_log_plot(
    filename: &str,
    log_degrees: &[f64],
    log_counts: &[f64],
    slope: f64,
    intercept: f64,
) -> Result<(), Box<dyn std::error::Error>> {
    // Log Log Plot Function:
    // Takes several parameters and graphs a log log plot and returns the plot as an image file
    //
    // Inputs:
    // • filename - a string reference that stores the name of the image file (on which the plot will be returned)
    // • log_degrees - a slice of f64 degree values (from a degree distribution) that have undergone a log-log transformation
    // • log_counts - a slice of f64 count values (from a degree distribution) that have undergone a log-log transformation
    // • slope - f64 value indicating slope from linear regression between two sets of points
    // • intercept - f64 value indicating intercept from linear regression between two sets of points
    //
    // Outputs:
    // • Result enum that returns nothing or an error
    let root = BitMapBackend::new(filename, (800, 600)).into_drawing_area();
    let _ = root.fill(&WHITE);
    let x_min = log_degrees.iter().cloned().fold(f64::INFINITY, f64::min);
    let x_max = log_degrees
        .iter()
        .cloned()
        .fold(f64::NEG_INFINITY, f64::max);
    let y_min = log_counts.iter().cloned().fold(f64::INFINITY, f64::min);
    let y_max = log_counts.iter().cloned().fold(f64::NEG_INFINITY, f64::max);

    let mut plt = ChartBuilder::on(&root)
        .caption("Log-Log Degree Dist. Plot", ("sans-serif", 30))
        .margin(20)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(x_min..x_max, y_min..y_max)?;
    // create a plot using ChartBuilder on the filename str reference and build a cartesian 2D plane

    plt.configure_mesh()
        .x_desc("Log Degree")
        .y_desc("Log Count")
        .draw()?;
    plt.draw_series(
        log_degrees
            .iter()
            .zip(log_counts.iter())
            .map(|(&x, &y)| Circle::new((x, y), 2, BLUE.filled())),
    )?; // draw the plot with the log degrees and log counts points

    let line_of_fit = [
        (x_min, intercept + slope * x_min),
        (x_max, intercept + slope * x_max),
    ];
    plt.draw_series(LineSeries::new(line_of_fit, &RED))?;
    // create the line of best fit using intercept and slope vals and then draw the line of best fit

    Ok(())
}
