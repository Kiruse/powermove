# Powermove Common Library
This crate contains the abstract basic foundation of the Powermove game which can be used in external tooling without a strict dependency on the game engine [bevy](https://bevyengine.org/). For example, we can use it to generate procedural content for the game and export it in a data format that can then be re-imported into the actual game.

# Technical Details

## Behaviors
Behaviors are data-driven descriptions of different types of entities in the game. See `src/behaviors.rs` for the concrete implementation. All behaviors share 3 fields, of which 2 required: `type`, `version`, and `iteration` (optional). `type` defines the specific type of entity that this behavior file describes. `version` is the employed integer version of the `type`'s own specialized format. And `iteration` is a custom-defined integer version of this behavior file itself.
