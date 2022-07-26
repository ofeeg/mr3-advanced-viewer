extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(ProcessGaddrs)]
pub fn process_gaddrs_derive(input: TokenStream) -> TokenStream
{
    let ast = syn::parse(input).unwrap();
    impl_process_gaddrs(&ast)
}



//use std::collections::vec_deque::ring_slices::RingSlices;
fn impl_process_gaddrs(ast: &syn::DeriveInput) -> TokenStream
{
    let name = &ast.ident;
    let gen = quote! {
	impl ProcessGaddrs for #name {
	    fn _new(s: usize) -> Self{ return #name{valid_addresses: Vec::with_capacity(s)}; }
	    fn init_data(&mut self, _start: u32, _offsets:  [u32;15])
	    {
		self.valid_addresses.push(_start);
		let i = _offsets.clone();
		for m in i.iter()
		{
		    self.valid_addresses.push((self.valid_addresses[0] + m));
		}
	    }
	}
    };
    gen.into()
}
