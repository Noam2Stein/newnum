The `newnum` Rust crate is an alternative to the `num` crate that splits the Rust number API and hierarchy into traits, with a cleaner design than `num`.

The design of this crate follows these rules:

* the `Num` trait isn't restricted to primitives, or to types that can represent specific common values (0, 1...).

* the `Prim`, `SignedPrim`, `UnsignedPrim`, `Int`, `UInt`, `SInt`, and `Float` traits are restricted to primitives,
  but allow non `std` types to implement them as long as they follow primitive restrictions.

* API traits (`Round`, `Root`, `Trig`...) aren't restricted to `Num` and are designed for non number support, for example `<Angle as Trig>::Output = Ratio`.

### Development State

The crate is already very usable but makes breaking changes every version to improve the design.