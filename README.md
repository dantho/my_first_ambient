## Ambient -- The Multiplayer Game Engine
This repo is a tiny app I created from Ambient's Hello World example.  
[Ambient](https://www.ambient.run/) is a WebAssembly runtime for building high-performance online games and 3D applications... using [Rust](https://www.rust-lang.org/)!  
There's an [Ambient Book](https://ambientrun.github.io/Ambient/).  

If you're just learning about Entity Component System programming, I recommend this [blog post series by Adam Martin](https://t-machine.org/index.php/2007/11/11/entity-systems-are-the-future-of-mmog-development-part-2/).  It helped me undestand Ambient.  

I bragged about this little learning app [on Discord](https://discordapp.com/channels/894505972289134632/1081971986491785366).

Use Git "exploding" tag if HEAD doesn't reproduce this fun:     
[![Exploding Boxes!](https://user-images.githubusercontent.com/3302181/222970827-be83850b-64f8-4096-a403-9fe52f4d0c2e.png "Exploding Boxes!")](https://user-images.githubusercontent.com/3302181/222970343-3bed1a2e-45e1-4499-ad4b-be5d3216c172.mp4)


### Other Libraries

hecs owes a great deal to the free exchange of ideas in Rust's ECS library
ecosystem. Particularly influential examples include:

- [bevy], which continually pushes the envelope for performance and ergonomics
  in the context of a batteries-included framework
- [specs], which was key in popularizing ECS in Rust
- [legion], which demonstrated archetypal memory layout and trait-less
  components

If hecs doesn't suit you, one of those might do the trick!

[bevy]: https://github.com/bevyengine/bevy
[specs]: https://github.com/amethyst/specs
[legion]: https://github.com/TomGillen/legion