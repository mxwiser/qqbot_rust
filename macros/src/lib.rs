use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn my_main(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(item as ItemFn);

    // 确保函数名为 main 并且是 async
    if input_fn.sig.ident != "main" {
        panic!("这个宏只能用于 main 函数");
    }
    if input_fn.sig.asyncness.is_none() {
        panic!("main 函数必须是 async 的");
    }

    // 获取原函数的块
    let block = &input_fn.block;

    // 生成新的 main 函数
    let expanded = quote! {
        fn main() {
            tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
                .unwrap()
                .block_on(async #block)
        }
    };

    TokenStream::from(expanded)
}