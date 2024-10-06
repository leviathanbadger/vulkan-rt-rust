# Abstractions

## Components

### Engine

Provides interface abstractions for:

  - Configuration
  - Scenes
  - Game objects
  - Game update loop (update tick, fixed tick, physics & collision, etc.)
  - Platform-independent input (keyboard, mouse, controller/joystick, etc.)
  - Cameras
  - Dynamic resource loading (including LODs)

The engine requires a singular platform adapter and graphics adapter,

### Platform adapters

Interfaces with engine. Provides abstractions for:

  - Platform-dependent handle creation and destruction (window handles, but also potentially sockets or other OS resources)
  - Platform-dependent input (keyboard events, mouse events, controller/joystick events, etc.)
  - Platform-dependent output other than graphics (haptic feedback for example)

Example planned platform adapters, although likely most of these will not ever be developed:

  - platform-adapter-winit (winit-based platform adapter to create window, handle windows events, etc.)
  - platform-adapter-web (HTML5- + WASM-based platform adapter to create render target, handle resize + keyboard events, etc.)
  - platform-adapter-windows (hypothetical Windows-specific platform adapter to replace winit, with support for
    Windows-specific APIs such as DirectStorage and explicit handling of unabstracted Windows events.)
  - ...you get the idea.

### Graphics adapters

Interfaces with engine. Provides implementations or abstractions for:

  - Render loop, including fixed frame rate/matching frame rate to refresh rate.
  - Creating and compiling shaders.
  - Communicating with the GPU, creating GPU resources, etc.
  - All aspects of the render pipeline, including dynamic resolution scaling, postprocessing effects, etc.
  - Updating or recreating GPU resources when the render target is resized or recreated.

The graphics adapter is not necessarily independent of the platform adapter. Some graphics adapters
may require more direct integration with specific platform-adapter abstractions, making the two abstractions
more tightly-coupled than I would like. However, the coupling is still in only one direction - graphics
adapters can depend on the platform adapter, but the platform adapter can't depend on a specific graphics
adapter.

Example planned graphics adapters, although likely most of these will not ever be developed:

  - graphics-adapter-vulkan
  - graphics-adapter-directx
  - graphics-adapter-opengl
  - graphics-adapter-webgpu
  - graphics-adapter-webgl
  - graphics-adapter-metal

### Plugins (maybe)

Possibly in the future, some functionality that shouldn't be in every project because it would unreasonably bloat
the libraries or executables can be added in the form of some kind of plugin. This is very future stuff, I'm not
sure what this would look like in practice. A lot of other stuff needs to get done first.

### Unknown/requires research:

I'm not currently sure where the best place would be to put:

  - Audio processing (incl. 3d audio, reverb/other processing effects)
  - Texture/mesh/etc. ingest and loading
