extern crate proc_macro;

use std::str::FromStr;

use cron::Schedule;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn async_cron_task(attr: TokenStream, func: TokenStream) -> TokenStream {
    let func = parse_macro_input!(func as ItemFn);
    let func_vis = &func.vis; // like pub
    let func_block = &func.block; // { some statement or expression here }

    let func_decl = &func.sig;
    let func_name = &func.sig.ident; // function name
    let func_generics = &func_decl.generics;
    let func_inputs = &func_decl.inputs;
    let func_output = &func_decl.output;
    let func_async = &func_decl.asyncness;
    let is_async_function = func_async.is_some().clone();

    if func_output != &syn::ReturnType::Default {
        panic!("async_cron_task function must return nothing");
    }
    if func_inputs != &syn::punctuated::Punctuated::new() {
        panic!("async_cron_task function must have no arguments");
    }

    let func_block = if is_async_function {
        quote! {
           #func_async {#func_block}.await;
        }
    } else {
        quote! {#func_block}
    };

    let expression = parse_macro_input!(attr as syn::LitStr);
    let cron = expression.value();
    let cron = cron.as_str().try_into().unwrap();

    let _ = Schedule::from_str(cron).expect("parse schedule expression error");

    let caller = quote! {
        #func_vis async fn #func_name #func_generics(#func_inputs) #func_output {
            use std::time;
            use cron::Schedule;
            use std::str::FromStr;

            let schedule = Schedule::from_str(#cron).unwrap();
            loop{
                 #func_block
                 let now = chrono::offset::Local::now();
                 let next = schedule.upcoming(chrono::offset::Local).next().unwrap();
                 let duration = (next - now).to_std().unwrap();
                 let start = time::Instant::now();
                 tokio::time::sleep(duration).await; 
            }

        }
    };

    caller.into()
}


