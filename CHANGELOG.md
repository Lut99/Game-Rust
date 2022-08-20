# Changelog
This file will maintain a list of changes per release of the Game-Rust.


## [0.2.0] - 2022-08-20
### Added
- `game-evt` as a general event system (i.e., just a separate crate with the game loop at the end of `main`).
- `game-pip` as a collection of render system pipelines.
- `game-tgt` as a collection of render system targets.
- A `SquarePipeline` that can now render stuff indexed.
- CLI & config file options for controlling the window position.

### Changed
- `game-ecs` to be an external crate instead ([`rust-ecs`](https://github.com/Lut99/rust-game/rust-ecs)).
- `game-vk` to be an external crate instead ([`rust-vk`](https://github.com/Lut99/rust-game/rust-vk) and [`rust-win`](https://github.com/Lut99/rust-game/rust-win) for the Window-part).


## [0.1.0] - 2022-05-15
### Added
- `game-bin` as the main binary crate of the project.
- `game-lst` as a supplamentary binary that lists available GPUs.
- `game-ins` as the setup executable.
- `game-cfg` as a crate collecting config / settings / CLI parsing for the main binary crate.
- `game-gfx` as the crate implementing the RenderSystem.
- `game-vk` as the crate connecting the RenderSystem to the Vulkan ([ash](https://github.com/ash-rs/ash)) backend.
- `game-ecs` as the crate implementing an Entity Component System (ECS).
- `game-utl` as an auxillary crate containing basic tools and utilities shared among other crates.
