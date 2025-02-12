# Features
## Video
- [x] Sprite system
    - [x] Sprite / Quad
        - [x] Customizations
            - [x] Position
            - [x] Size
            - [x] Layer
            - [x] Texture
            - [x] Rotation
            - [x] Flip

    - [x] SpriteSheet
        - [x] Initialize from any image file
        - [x] Supports consitently sized sprite sheets of any size

- [ ] Shader system
    - [ ] Default shaders
        - [ ] Vertex Shader
            - Places the vertex positions based on position, size, rotation, flip, etc.
        - [x] Fragment Shader
            - Applies given texture to sprite

    - [x] Shader refactor

- [ ] Main rendering loop
    - [x] Draw in layers (higher layer level shows above lower levels)
    - [ ] Optimizations
        - [ ] Batch by shader
        - [ ] Batch by sprite sheet
        - [ ] Avoid drawing sprites that are offscreen

## Audio
- [ ] Play audio

## Input
- [x] Polling of all keyboard and mouse keys/buttons
- [ ] Polling of mouse position
