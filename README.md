# RoundTable

D&D tabletop management software written in Rust.

## TODO

### Must

- [ ] Movable map
- [ ] Map select

## Design contraints

All the data for displaying the map and everything on it should be placed in
a single data structure. This way it *should* be easy to sync this with another
client.

## License

See [LICENSE](./LICENSE.txt)
