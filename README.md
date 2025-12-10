# `librados`

Idiomatic bindings (both sync and runtime-agnostic `async`) for Ceph's `librados`.


## Linking

This library automatically dynamically links to an in-tree copy of the `librados.so.2.0.0`
dynamic library, as included in Ceph reef (ceph version 18.2.7 (6b0e988052ec84cf2d4a54ff9bbbc5e720b621ad) reef (stable)).

This means that the computer you run your final binary program on must have a compatible version of `librados` installed,
and findable in the link path. The default installation of Ceph does this for you automatically.

The `librados` library is licensed under LGPLv2.1 (see [LICENSE-LGPL2.1](./LICENSE-LGPL2.1)), which places requirements
on software compiled using this library.

# Usage

For usage examples, start at the [Rados](https://docs.rs/librados/latest/librados/struct.Rados.html) struct, or [the examples](./examples/).

# License

This project is licensed under the MIT license. See [LICENSE](./LICENSE) for the full license text.

# Notice

The copy of `librados.so.2.0.0` included in this source tree is licensed under the LGPLv2.1 license (see [LICENSE-LGPL2.1](./LICENSE-LGPL2.1)).

This project dynamically links against [`librados`](https://github.com/ceph/ceph/blob/main/src/include/rados/librados.h), licensed
under the LGPLv2.1 license (see [LICENSE-LGPL2.1](./LICENSE-LGPL2.1)), which places requirements on software compiled using this
library.