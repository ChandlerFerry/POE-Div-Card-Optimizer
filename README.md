# POE-Div-Card-Optimizer
Calculates optimal Favored Maps for Divination Scarab of Completion + Curation


## Scarabs
https://www.pathofexile.com/forum/view-thread/3497694

### Completion
Divination cards which drop in Area have a 20% chance to drop as a full stack instead

### Curation
10% more Divination Cards found in Area per different Favoured Map
Divination card drops in Area are replaced by those of your Favoured Maps


## Usage

### Setup
1. Install [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)

### Run in release mode for faster computation
1. Edit `cards.json` with what you value
2. Edit `main.rs` Line #12 `NUM_THREADS` for # of threads you want to compute with
3. Edit `main.rs` Line #492 `initial_areas` for manual picks
4. Edit `main.rs` Line #165 `FORCE_REMOVE_FILTER` and Line #255 `FORCE_SHOW_FILTER`
5. Execute from `src` directory:
```
cargo run --release
```


## TODO
1. Find a better algorithm (tried sampling, but PoE is not true-random) (tried choose 3x3x3x3/4x4x4 but it wasn't intelligent enough)
2. Cleanup `Cargo.toml`, pulled from a different repo
3. Cleanup `ev` and `drop_info`, pulled from another source
