termimage(1) -- Display images in your terminal, kind of
========================================================

## SYNOPSIS

`termimage` [OPTIONS] &lt;<IMAGE>&gt;

## DESCRIPTION

Show images in your terminal.

The images are automatically downscaled to the terminal's size and their
colours are approximated to match the terminal's display colours.

With simple ANSI output this means a 3-bit colour resolution, with WinAPI - 4-bit.

With WinAPI output the output colours are acquired from the console itself,
with ANSI output a sane default is assumed.

## OPTIONS

  &lt;<IMAGE>&gt;

  Image to display.

  The format is guessed from the extension or magic.

  -s --size &lt;<size>&gt;

    Output image resolution.

    By default this is autodetected to match the output terminal's resolution,
    but is required when outputting to a file.

    Format: NxM

  -a --ansi &lt;<ANSI_type>&gt;

    Force ANSI output of the specified kind,

    The accepted values are "simple" and "truecolor", truecolor is the default
    on non-Windows.

    Simple ANSI output uses 3-bit background colours, while truecolor supports
    the whole 24-bit pallette.

  -f --force

    By default the image's aspect ratio will be preserved when downscaling,
    use this option to override that behaviour.

## EXAMPLES

  `termimage` [`-s` *NxM*] [`-f`] *assets/image.png*

    Display assets/image.png in the terminal using the default output type,
    optionally not preserving the aspect ratio.

  `termimage` [`-s` *NxM*] [`-f`] [`-a` *simple*] *assets/image.png*

    Display assets/image.png in the terminal using the simple ANSI output type,
    optionally not preserving the aspect ratio.

  (for f in $(find *image_dir* -type f); do `termimage -s` *NxM* [`-f`] [`-a` *ANSI_type*] $f; done) > *out_file*

    Print all images in image_dir to out_file.

    Note the --size option being specified, since it's required when outputting to a file.

## AUTHOR

Written by nabijaczleweli &lt;<nabijaczleweli@gmail.com>&gt;,
           Josh Triplett &lt;<josh@joshtriplett.org>&gt;,
       and Aaron Hill &lt;<aa1ronham@gmail.com>&gt;

## SPECIAL THANKS

To all who support further development, in particular:

  * ThePhD

## REPORTING BUGS

&lt;<https://github.com/nabijaczleweli/termimage/issues>&gt;

## SEE ALSO

&lt;<https://github.com/nabijaczleweli/termimage>&gt;
