termimage(1) -- Display images in your terminal, kind of
========================================================

## SYNOPSIS

`termimage` [OPTIONS] &lt;<IMAGE>&gt;

## DESCRIPTION

Show images in your terminal.

The images are automatically downscaled to the terminal's size and their
colours are approximated to match the terminal's display colours.

With ANSI output this means a 3-bit colour resolution, with WinAPI - 4-bit.

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

  -f --force

    By default the image's aspect ratio will be preserved when downscaling,
    use this option to override that behaviour.

  -a --ansi

    Force ANSI output.

    This really applies only on Windows, as there's no non-ANSI alternatives
    on other platforms.

## EXAMPLES

  `checksums` [`-s` *NxM*] [`-f`] *assets/image.png*

    Display assets/image.png in the terminal, optionally not preserving
    the aspect ratio.

## AUTHOR

Written by nabijaczleweli &lt;<nabijaczleweli@gmail.com>&gt;

## REPORTING BUGS

&lt;<https://github.com/nabijaczleweli/termimage/issues>&gt;

## SEE ALSO

&lt;<https://github.com/nabijaczleweli/termimage>&gt;
