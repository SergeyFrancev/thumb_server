# Rust Thumb Image Server

[![Build Status](https://github.com/SergeyFrancev/thumb_server/actions/workflows/rust.yml/badge.svg)](https://github.com/SergeyFrancev/thumb_server/actions)

![Logo](https://github.com/SergeyFrancev/thumb_server/blob/main/examples/logo.jpg?raw=true)

This is simple image thumbnail HTTP server.

- [Hyper](https://github.com/hyperium/hyper) - HTTP server
- [Image](https://github.com/image-rs/image) - Work with image

He generates image previews or returns existing ones based on the specified parameters in the configuration file.

## Example

- config.toml

```
base_dir = "/your/path/to/base_directory"
sizes = ["300x200", "100x100"]
```

- You have only _image.jpg_ source file

```
base_directory
└── recipies_image
    └── image.jpg       // Source Image [Any size]
```

- After request to `YOUR_DOMAIN/recipies_image/300x200/image.jpg`

```
base_directory
└── recipies_image
    ├── 300x200
    |   └── image.jpg   // Image [300x200]
    └── image.jpg       // Source Image [Any size]
```

_Requests return 404_

- incorect size
  `YOUR_DOMAIN/recipies_image/123x321/image.jpg` > **404**
- not found image
  `YOUR_DOMAIN/recipies_image/300x200/image2.jpg` > **404**

## Build Relise Version

```
cargo build -r
```

## Run

> - **-c:** _path to config file [conf.toml]_
> - **-p:** _port for listen HTTP request [default:4000]_

```
thumb_server -c ./conf.toml -p 1234
```

## Tests

```
cargo tests
```

## Benchmark

```
cargo bench
```
