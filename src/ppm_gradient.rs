use std::io::{stderr, Write};

fn main() {
    const IMG_WIDTH: i32 = 256;
    const IMG_HEIGHT: i32 = 256;

    println!("P3\n{} {}\n255\n",
             IMG_WIDTH, IMG_HEIGHT);

    for j in (0..IMG_WIDTH).rev() {
        eprint!("\rScanlines remaining: {}", j);
        stderr().flush().unwrap();

        for i in 0..IMG_HEIGHT {
            let r = (i as f32 / (IMG_WIDTH - 1) as f32)  as f32;
            let g = (j as f32 / (IMG_HEIGHT - 1) as f32) as f32;
            let b = 0.25;

            let ir = (255.999 * r as f32) as i32;
            let ig = (255.999 * g as f32) as i32;
            let ib = (255.999 * b as f32) as i32;

            println!("{} {} {}", ir, ig, ib);
        }
    }
    eprintln!("\nDone.\n")
}
