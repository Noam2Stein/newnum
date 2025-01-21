Splits the Rust number API and hierarchy into traits, with a cleaner design than the ```num``` crate.

The design of this crate follows these rules:

* the ```Num``` trait isn't restricted to primitives,
doesn't require representing values such as ```0``` or ```1```,
and is only restricted to representing abstract numbers meaning no measurement units.

* the ```Prim```, ```SignedPrim```, ```UnsignedPrim```, ```Int```, ```UInt```, ```SInt```, and ```Float``` traits are restricted to primitives,
but allow non ```std``` types to impl them as long as they follow primitive restrictions.

* API traits (```Round```, ```Root```, ```Trig```...) aren't restricted to ```Num```
and are designed for non number support, for example ```<Angle as Trig>::Output = Ratio```.

### Development State

the current state of this crate is using it for different projects and modifying the design until it hardens,
so until then every version will have breaking changes.
