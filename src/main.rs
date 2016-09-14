extern crate termimage;
extern crate image;

use std::process::exit;
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
    let img = termimage::ops::load_image(&opts.image, format);

    let resized = termimage::ops::resize_image(&img, opts.size, opts.preserve_aspect);

    if opts.ansi_out {
        termimage::ops::write_ansi(&mut stdout(), &resized);
    } else {
        termimage::ops::write_no_ansi(&resized);
    }

    Ok(())
}
