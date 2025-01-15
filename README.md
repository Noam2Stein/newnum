Splits the Rust number API into common traits, with a cleaner design than ```num```.

The base num trait represents data-types that represent abstract numbers, with the usual +-*/% operators.
So no measurement units (```Meters```) or operator blocking (```Position```).

A ```Num``` can't nessesarely represent any specific value (0, 1, 2...),
because a number type might, for example, represent only numbers in the range of 100...356.

Number APIs are each in a different trait and can be also implemented by non numbers.
For example: a ```Vec2``` can't implement ```Num``` but it can implement ```Round```.
