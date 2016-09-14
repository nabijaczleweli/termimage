extern crate termimage;
extern crate image;

use image::GenericImage;
use std::process::exit;
use std::io::stderr;


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
    println!("{:#?}", opts);

    let format = try!(termimage::ops::guess_format(&opts.image));
    let img = termimage::ops::load_image(&opts.image, format);

    println!("{:?}", img.dimensions());
    let resized = termimage::ops::resize_image(&img, opts.size, opts.preserve_aspect);
    println!("{:?}", resized.dimensions());

    if opts.ansi_out {
        // TODO
        println!("ANSI output unimplemented");
    } else {
        termimage::ops::write_no_ansi(&resized);
    }

    Ok(())
}
