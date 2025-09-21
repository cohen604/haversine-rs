use anyhow::Result;

pub fn reference_haversine(x0: f64, y0: f64, x1: f64, y1: f64, earth_radius: f64) -> Result<f64> {
    let lat1 = y0.to_radians();
    let lat2 = y1.to_radians();
    let lon1 = x0.to_radians();
    let lon2 = x1.to_radians();

    let d_lat = lat2 - lat1;
    let d_lon = lon2 - lon1;

    let a = f64::powi(f64::sin(d_lat / 2.0), 2);
    let b = f64::cos(lat1) * f64::cos(lat2) * f64::powi(f64::sin(d_lon / 2.0), 2);
    let c = 2.0 * f64::asin(f64::sqrt(a + b));

    Ok(earth_radius * c)
}
