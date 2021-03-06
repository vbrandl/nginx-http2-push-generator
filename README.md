# Nginx HTTP2 Push Generator

[![Travis Build
Status](https://travis-ci.org/vbrandl/nginx-http2-push-generator.svg?branch=master)](https://travis-ci.org/vbrandl/nginx-http2-push-generator)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](https://github.com/vbrandl/nginx-http2-push-generator/blob/master/LICENSE-MIT)
[![License](https://img.shields.io/badge/license-Apache-green.svg)](https://github.com/vbrandl/nginx-http2-push-generator/blob/master/LICENSE-APACHE)

Since version 1.13.9 from 2018-02-20, the [nginx](https://www.nginx.com/) web server supports HTTP2
server push. This can be a pain to configure since for every file the list of files to push has to
be configured.

This tool aims to automate the process of generating said list. It parses a HTML file, iterates over
all nodes and looks for a set of predefined attributes (see below). If the value of the attribute
seems to be a local path (e.g. does not contain `://` and does not start with `//`), the tool checks
if the value is a relative or absolute path. Relative paths get prefixed by a user defined prefix,
absolute paths are taken as is.
The tool takes a list of file extensions as parameter and only resources ending in any of the
extensions will be used in the configuration.

## Checked Attributes

Currently the following attributes are checked:

 * `href`
 * `src`

## Building

I tested building the binary using the current stable (`1.24.0`) and nightly (`1.25.0-nightly (3ec5a99aa 2018-02-14)`)
Rust compiler but older versions should work fine, too.

To build the latest stable version of the config generator, you need to install Rust and cargo (preferably using
rustup), clone this repository, check out the `master` branch and run the following command:

```
$ cargo build --release
```

Precompiled binaries for a variety of targets can be found on the [release
page](https://github.com/vbrandl/nginx-http2-push-generator/releases)

## Usage

```
$ ./nginx_http2_push_generator -h
nginx_http2_push_generator 0.1.0
Valentin B. <mail@vbrandl.net>
Parse a HTML file and generate a HTTP2 push configuration for nginx

USAGE:
    nginx_http2_push_generator [OPTIONS] <EXT>...

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -i, --input <FILE>       Input value. If not set or if set to "-", data is read from stdin
    -p, --prefix <PREFIX>    Set a path prefix. Should be the path where the file is located. Defaults to "/"

ARGS:
    <EXT>...    File extensions to include
```

### Examples

```
$ ./nginx_http2_push_generator .css .js -i index.html -p /prefix
http2_push /prefix/style1.css;
http2_push /style2.css;
http2_push /style3.css;
http2_push /prefix/foo.js;
```

```
$ cat index.html | ./nginx_http2_push_generator .css .js -p /prefix
http2_push /prefix/style1.css;
http2_push /style2.css;
http2_push /style3.css;
http2_push /prefix/foo.js;
```

## License

nginx_http2_push_generator is licensed under either of the following, at your option:

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT License ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
