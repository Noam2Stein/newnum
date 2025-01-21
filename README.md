Splits the Rust number API and hierarchy into traits, with a cleaner design than ```num```.

The design of this crate follows these rules:

* the ```Num``` trait isn't restricted to primitives,
doesn't require to represent values such as ```0``` or ```1```,
and is only restricted to representing abstract numbers meaning no measurement units.

* the ```Prim``` trait is restricted to primitives,
but allows non ```std``` types to impl it as long as they follow primitive restrictions.

* API traits (```Round```, ```Root```, ```Trig```...) aren't restricted to ```Num```
and are designed for non number support, for example ```<Angle as Trig>::Output = Ratio```.
