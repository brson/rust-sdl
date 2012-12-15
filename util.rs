pub type Rect = {
    x: i16,
    y: i16,
    w: u16,
    h: u16
};

pub fn init_flags_to_bitfield(flags: &[sdl::InitFlag]) -> u32 {
    vec::foldl(0u32, flags, |flags, flag| {
        flags | *flag as ll::sdl::sdl_init_flag 
    })
}
