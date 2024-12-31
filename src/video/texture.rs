pub struct Texture {
    id: u32,
    width: u32,
    height: u32,
    // todo!()
}

impl Default for Texture {
    fn default() -> Self {
        Texture {
            id: 0,
            width: 10,
            height: 10,
        }
    }
}
