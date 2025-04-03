#[inline(always)]
fn fpw_statistic(times: &[f64], ivar: &[f64], ivar_y: &[f64], f: f64, n_bins: usize) -> f64 {
    let mut vt_cinv_v = [0.0; 20];
    let mut yt_cinv_v = [0.0; 20];

    let scale = f * n_bins as f64;

    // Accumulate inverse variance and ivar * y products for each bin
    for i in 0..times.len() {
        let index = ((times[i] * scale) as usize) % n_bins;
        vt_cinv_v[index] += ivar[i];
        yt_cinv_v[index] += ivar_y[i];
    }

    // Compute the final statistic
    let mut sum = 0.0;
    for i in 0..n_bins {
        sum += yt_cinv_v[i].powi(2) / vt_cinv_v[i];
    }
    sum * 0.5
}

pub fn fpw(t: &[f64], y: &[f64], dy: &[f64], freqs: &[f64], n_bins: usize) -> Vec<f64> {
    if n_bins > 20 {
        panic!("n_bins must be less than or equal to 20");
    }

    let mut ivar = vec![0.0; t.len()];
    let mut ivar_y = vec![0.0; t.len()];
    for i in 0..t.len() {
        ivar[i] = 1.0 / (dy[i] * dy[i]);
        ivar_y[i] = ivar[i] * y[i];
    }

    // Calculate FPW statistic for each frequency
    freqs
        .iter()
        .map(|&freq| fpw_statistic(t, &ivar, &ivar_y, freq, n_bins))
        .collect()
}

pub fn freq_grid(t: &[f64], fmin: Option<f64>, fmax: Option<f64>, oversample: usize) -> Vec<f64> {
    let trange =
        t.iter().cloned().fold(f64::NAN, f64::max) - t.iter().cloned().fold(f64::NAN, f64::min);
    let texp = t.windows(2).map(|w| w[1] - w[0]).fold(f64::NAN, f64::min);
    let fres = 1.0 / trange / oversample as f64;

    let fmax = fmax.unwrap_or(0.5 / texp);
    let fmin = fmin.unwrap_or(fres);

    let mut fgrid = Vec::new();
    let mut f = fmin;
    while f < fmax {
        fgrid.push(f);
        f += fres;
    }
    fgrid
}

pub fn get_best_freq(freqs: &[f64], result: &[f64]) -> (f64, f64) {
    let max_index = result
        .iter()
        .enumerate()
        .filter(|&(_, &x)| !x.is_nan())
        .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
        .map(|(i, _)| i)
        .unwrap_or(0);

    (freqs[max_index], result[max_index])
}

pub fn get_best_freqs(freqs: &[f64], result: &[f64], n: usize) -> Vec<(f64, f64)> {
    let mut freq_stat_pairs: Vec<(f64, f64)> = freqs
        .iter()
        .zip(result.iter())
        .filter(|&(_, &x)| !x.is_nan())
        .map(|(&f, &s)| (f, s))
        .collect();

    // Sort by statistic in descending order
    freq_stat_pairs.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    // Return the top n frequencies
    freq_stat_pairs.into_iter().take(n).collect()
}
