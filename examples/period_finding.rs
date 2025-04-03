use flare::period::{fpw, freq_grid, get_best_freq};
use rand::Rng;

fn main() {
    // let's simulate a sinusoidal with a period of 6 hours, with some noise
    // measured at N random times over a 6 years period.
    let n_points = 1000;
    let period = 0.25; // in days
    // we don't want evenly spaced data, so let's randomly sample times over 6 years
    // so we grab random floats between 0 and 6*365 days (6 years)
    let mut rng = rand::rng();
    let mut t: Vec<f64> = Vec::with_capacity(n_points);
    for _ in 0..n_points {
        // generate a random time between 0 and 6*365 days
        let random_time = rng.random_range(0.0..(6.0 * 365.0));
        t.push(random_time);
    }
    // Sort the time array to ensure it's in ascending order
    t.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

    // Now generate the sinusoidal light curve with some noise
    let noise_level = 0.1; // noise in magnitude
    let y: Vec<f64> = t.iter()
        .map(|&time| {
            // Simulate a sinusoidal light curve with some noise
            let true_value = (2.0 * std::f64::consts::PI * time / period).sin();
            let noise = rng.random_range(-noise_level..noise_level);
            true_value + noise
        })
        .collect();

    
    // Define frequency grid, with a min freq of 10 days
    // and a maximum frequency of 5 minutes
    let f_min = 1.0 / (10.0 * 24.0 * 60.0);
    let f_max = 1.0 / (5.0 / 60.0);
    let freqs = freq_grid(&t, Some(f_min), Some(f_max), 3);
    println!("Using {} frequencies", freqs.len());

    // the FPW algorithm requires a number of bins, typically between 5 and 20
    // in this crate, we've set a maximum of 20 bins. This is enough for most
    // applications, and using a set number of bins let's us allocated memory to the stack
    // which is much faster than heap allocation.
    let n_bins = 10; // choose a number of bins between 5 and 20
    let fpw_stats = fpw(&t, &y, &vec![0.1; n_points], &freqs, n_bins);

    // Find the best period using the FPW statistic
    let (best_freq, best_stat) = get_best_freq(&freqs, &fpw_stats);

    let period = 1.0 / best_freq * 24.0; // convert from frequency in days to a period in hours

    println!(
        "Period: {:.2} hours, statistic: {:.2}",
        period,
        best_stat
    );
    

    
}