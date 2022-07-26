pub trait ProcessGaddrs
{
    fn _new(s: usize) -> Self;
    fn init_data(&mut self, _start: u32, _offsets:  [u32;15]);
}
