extern crate termimage;
extern crate image;

use std::process::exit;
use image::GenericImage;
use std::io::{stdout, stderr};


fn main() {
    let result = actual_main();
    exit(result);
}

fn actual_main() -> i32 {
    let err = result_main().err().unwrap_or(termimage::Outcome::NoError);
    err.print_error(&mut stderr());
    err.exit_value()
}

fn result_main() -> Result<(), termimage::Outcome> {
    let opts = termimage::Options::parse();

    let format = try!(termimage::ops::guess_format(&opts.image));
    let img = try!(termimage::ops::load_image(&opts.image, format));

    let img_s = termimage::ops::image_resized_size(img.dimensions(), opts.size, opts.preserve_aspect);
    let resized = termimage::ops::resize_image(&img, img_s);

    match opts.ansi_out {
        Some(true) => termimage::ops::write_ansi_truecolor(&mut stdout(), &resized),
        Some(false) => termimage::ops::write_ansi(&mut stdout(), &resized),
        None => termimage::ops::write_no_ansi(&resized),
    }

    Ok(())
}
