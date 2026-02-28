A pure-rust implementation of protocols & messages used for communication in Ceph clusters.

This crate aims to support versions NAUTILUS and up. Support for older versions and protocols (i.e. legacy adresses, msgr1) is not planned.

## Crates

This project contains the following crates:

* `ceph-client`: a (currently bare-bones) implementation of a ceph client.
* `ceph-foundation`: decoding, encoding, and encryption primitives used by all crates, as well as primitive data structures used by other crates.
* `cephx`: decoding of `cephx` messages.
* `ceph-messages`: support for decoding of higher-level messages (i.e. those contained within [`msgr2` Message frames][1].)
* `msgr2`: implementation of the [`msgr2`] protocol used by Ceph clients.

[`msgr2`]: https://docs.ceph.com/en/quincy/dev/msgr2/
[1]: https://docs.ceph.com/en/quincy/dev/msgr2/#message-exchange
