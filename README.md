# progredient

[![Crate][crate-image]][crate-link]
[![Build Status][build-image]][build-link]
[![MIT Licensed][license-image]][license-link]

Just a program wrapping the nice and simple [dominicparga/progressing](https://github.com/dominicparga/progressing) for direct use in scripts etc.

## Installing

```
$ cargo install progredient
```

## Usage

```
Usage:
progredient --at X%
progredient --at Y --from X --to Z

Optional arguments:
--length L        stretch the bar to be this number of chars long
--style ABCDE     render the bar as "ABBBBBBCDDDE" with C positioned at --at
--label LABEL     show this text after the bar
--left            show the label before the bar instead
--no-newline      do not print a newline after the bar
```

## Examples

Glance at your disk usage:
```shell
$ df | grep disk | while read label blocks used available capacity rest; do progredient --at $capacity --label $label --style '[|| ]' ; done
[||||              ] /dev/disk1s5s1
[|                 ] /dev/disk1s4
[|                 ] /dev/disk1s2
[|                 ] /dev/disk1s6
[||||||||||||||    ] /dev/disk1s1
```

Track your workday:
```
$ to_seconds() { gdate -d "1970-01-01 UTC $1" +%s }
$ progredient --at $(to_seconds $(gdate +%T)) --from $(to_seconds 08:00) --to $(to_seconds 17:00) --style "--O--"
------------O-------
```

Check your battery:
```shell
$ progredient --at $(pmset -g batt | grep -o '[0-9]*%') --style '()) }'
()))))))))))))))   }
```

[//]: # (badges)

[build-image]: https://github.com/fabjan/progredient/workflows/Rust/badge.svg
[build-link]: https://github.com/fabjan/progredient/actions?query=workflow%3ARust
[license-image]: https://img.shields.io/badge/license-MIT-blue.svg
[license-link]: https://github.com/interchainio/tendermint-rs/blob/master/LICENSE
[crate-image]: https://img.shields.io/crates/v/progredient.svg
[crate-link]: https://crates.io/crates/progredient
