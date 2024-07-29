# Custom Character Systems
I'm still learning a lot about Bevy & most of what I build can probably done smarter/cleaner/more efficiently. This document serves to illuminate the character system design.

At the core of the design is, as always, simplicity: I want to be able to easily create, define, load & edit characters.

## Character Plugin
The plugin configures assets, asset loaders, components & abstracts away other nitty-gritty bits such as managing the `TextureAtlas` component & its `TextureAtlasLayout` asset handle.

## Character Component
This component is responsible for managing common character state such as animations & general behaviors. Things like AI & special state are not its concern & should be encapsulated in a separate dedicated component.

The Character component is defined primarily by its corresponding [`CharacterBehavior`](#character-behavior) and should be constructed with

```rust
commands.spawn(CharacterBundle::new(asset_server, "behavior.bhv"));
```

Where the path to the behavior file is relative to `assets/behaviors`. The `CharacterBundle::new` method will take care of spawning all the necessary components.

### Component Hooks & Asset Events
The Character behavior manages the spritesheet & within contained animations. Unfortunately, this means some entity components can only be spawned once the behavior asset has been loaded. Thus, the Character subsystem uses component hooks & asset events to streamline loading the behavior & extracting its data into the various auxiliary components, such as `SpriteBundle` and `TextureAtlas`.
