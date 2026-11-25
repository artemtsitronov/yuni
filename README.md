# yuni

Yuni is a steganography tool that stores data in polynomial expression.
It encodes text as LaTeX algebra and decodes it back.

## Example

```sh
$ yuni encode "Hello world!"

\left[\left\{\left[\left(9x^{2} + 59x + 54\right) + \left(-66x^{2} - 23x - 91\right)\right] + \left[\left(-11x^{2} - 38x + 59\right) - \left(-31x^{2} + 8x - 15\right)\right]\right\} + \left(-19x^{2} - 106x - 165\right)\right] = -56x^{2} - 116x - 128

$ yuni decode '\left[\left\{ ... = -56x^{2} - 116x - 128'

Hello world!
```

Rendered, it looks like my homework:

`[ { [ (9x² + 59x + 54) + (−66x² − 23x − 91) ] + [ (−11x² − 38x + 59) − (−31x² + 8x − 15) ] } + (−19x² − 106x − 165) ] = −56x² − 116x − 128`

You can encode the same message, and get different results: because the bits left over after the message are random filler, each encoding differs.

## HOW!?

Different expressions can have the same value, so the shape of an expressio can carry data that the final result doesn't show.
This tool uses it in two places:
- The result carries the first 24 bits of the message as a polynomial
- The structure of the expression carries the rest: each `+`/`-` picks a hidden polynomial `A`, and rewrites its value as `A + (value - A)`/`(value + A) - A`. The choices of `A` and choice of `+`/`-` are the message bits. The two halves split the same way again, until the message runs out.
The depth is chosen automatically from length of the message. Brackets alternate `[ ]` and `{ }` by nesting level. Only polynomials leaves use `( )`

## Usage

```sh
yuni encode <MESSAGE>
yuni decode '<EXPRESSION>'
```

## Buildin'

```sh
cargo build --release
```
