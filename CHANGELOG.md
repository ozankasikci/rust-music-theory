## v0.2.0 - 2020-08-19

### Features
- Add support for inversion of intervals (by @henryksloan)
- Add support for descending scales (by @henryksloan)

### Breaking Changes
- `Scale::new` method now expects an additional Direction argument.

## v0.1.7 - 2020-05-03

### Improvements
- Small performance improvements
- Fix Clippy lint errors

## v0.1.6 - 2020-01-26

### Features
- Add support for Note, Interval, Chord, Scale
- Add a cli binary to parse the given chord/scale string using regex
- Raise test coverage to 75 percent
- Add initial documentation for crates.io