fn main() {

    let image_width: i32 = 256;
    let image_height: i32 = 256;

    println!("P3\n{} {}\n255",  image_width.to_string(), image_height.to_string());

    for j in (0..image_height).rev() {
        for i in 0 .. image_width {

            let r =  (i as f64 / (image_width-1) as f64);
            let g = (j as f64 / (image_height-1) as f64);
            let b: f64 = 0.25;

            let ir = (255.999 * r) as i64;
            let ig = (255.999 * g) as i64;
            let ib = (255.999 * b) as i64;

            println!("{} {} {}", ir, ig, ib);
        }
    }
}
