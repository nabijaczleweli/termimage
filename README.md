# termimage [![TravisCI build status](https://travis-ci.org/nabijaczleweli/termimage.svg?branch=master)](https://travis-ci.org/nabijaczleweli/termimage) [![AppVeyorCI build status](https://ci.appveyor.com/api/projects/status/kk34veg25wre0gqe/branch/master?svg=true)](https://ci.appveyor.com/project/nabijaczleweli/termimage/branch/master) [![Licence](https://img.shields.io/badge/license-MIT-blue.svg?style=flat)](LICENSE) [![Crates.io version](http://meritbadge.herokuapp.com/termimage)](https://crates.io/crates/termimage)
Display images in your terminal, kind of

## [Documentation](https://cdn.rawgit.com/nabijaczleweli/termimage/doc/termimage/index.html)
## [Manpages](https://cdn.rawgit.com/nabijaczleweli/termimage/man/termimage.1.html)

### Usage

Display an image

```sh
termimage IMAGE_PATH
```

Print all images in a dir to a file.

```sh
(for f in $(find image_dir -type f); do termimage -s 150x33 $f; done) > out_file
```

For more usage examples see [the documentation](https://cdn.rawgit.com/nabijaczleweli/termimage/doc/termimage/index.html).

### Examples

Windows:
![DS3 image after](assets/DS3-result.jpg)

Linux with truecolor support:
![rust logo image after](assets/rust-logo-truecolor.png)
![playing dice image after](assets/playing-dice-truecolor.png)
