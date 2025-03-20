use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn bot_event(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(item as ItemFn);
    // 获取原函数的块
    let block = &input_fn.block;
    let fn_args = &input_fn.sig.inputs;
    let fn_ident = &input_fn.sig.ident;
    
    //    // 处理返回类型
    //     let return_type = match &input_fn.sig.output {
    //     ReturnType::Default => quote! { () },
    //     ReturnType::Type(_, ty) => quote! { #ty }
    // };
    // 生成新的函数
    let expanded = quote! {
        fn #fn_ident(#fn_args){
             tokio::spawn(async move {
                #block
            });
  
        }
    };

    TokenStream::from(expanded)
}