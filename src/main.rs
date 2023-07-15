extern crate termimage;
extern crate image;

use std::process::exit;
use image::GenericImageView;
use std::io::{BufWriter, Write, stdout, stderr};


fn main() {
    let result = actual_main();
    exit(result);
}

fn actual_main() -> i32 {
    if let Err(err) = result_main() {
        err.print_error(&mut stderr());
        err.exit_value()
    } else {
        0
    }
}

fn result_main() -> Result<(), termimage::Error> {
    let opts = termimage::Options::parse();

    let format = termimage::ops::guess_format(&opts.image)?;
    let img = termimage::ops::load_image(&opts.image, format)?;

    let img_s = termimage::ops::image_resized_size(img.dimensions(), opts.size, opts.preserve_aspect);
    let resized = termimage::ops::resize_image(&img, img_s);

    match opts.ansi_out {
        Some(ansi) => {
            let mut out = BufWriter::new(stdout().lock());
            match ansi {
                termimage::AnsiOutputFormat::Truecolor => termimage::ops::write_ansi_truecolor(&mut out, &resized),
                termimage::AnsiOutputFormat::SimpleWhite => termimage::ops::write_ansi(&mut out, &resized, &termimage::util::ANSI_COLOURS_WHITE_BG),
                termimage::AnsiOutputFormat::SimpleBlack => termimage::ops::write_ansi(&mut out, &resized, &termimage::util::ANSI_COLOURS_BLACK_BG),
            }
            out.flush().unwrap();
        }
        None => termimage::ops::write_no_ansi(&resized),
    }

    Ok(())
}
