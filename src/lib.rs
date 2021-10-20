
extern crate proc_macro;
use proc_macro::TokenStream;

extern crate syn;
#[macro_use]
extern crate quote;


#[proc_macro_derive(AnswerFn)]
pub fn derive_answer_fn(_item: TokenStream) -> TokenStream {
    "fn answer() -> u32 { 42 }".parse().unwrap()
}


#[proc_macro_derive(HelperAttr, attributes(helper, required, in_array))]
pub fn derive_helper_attr(_item: TokenStream) -> TokenStream {
    
    TokenStream::new()
}



#[proc_macro_derive(Falidate, attributes(helper, required, in_array))]
pub fn falidate(_item: TokenStream) -> TokenStream {
    let source = _item.to_string();
    println!("Normalized source code: {}", _item.to_string());    
    let ast = syn::parse_derive_input(&source).unwrap();


    //println!("Syn's AST: {:?}", ast); // {:#?} - pretty print
    let struct_name = &ast.ident;
    
    if let syn::Body::Struct(s) = ast.body {
        
        // let field_names : Vec<_> = s.fields().iter().map(|ref x|
        //     // println!("{}", x.vis);
        //     x.ident.clone().unwrap()).collect();
        
        // println!("Syn's AST: {:?}", field_names);

        let mut _i = 0;

        while _i < s.fields().len() {
            let _field_name  = &s.fields()[_i].ident.clone().unwrap().to_string();
            let _val = &s.fields()[_i].attrs[0].value;
            
            println!("{:?}", _val);
            

            _i += 1;
        }




        
    }


    

    
    // if let syn::Body::Struct(s) = ast.body {
    //     let field_names : Vec<_> = s.fields().iter().map(|ref x|
    //             x.ident.clone().unwrap()).collect();
        
    //     let field_getter_names = field_names.iter().map(|ref x|
    //             syn::Ident::new(format!("get_{}", x).as_str()));
    //     let field_setter_names = field_names.iter().map(|ref x|
    //             syn::Ident::new(format!("set_{}", x).as_str()));
    //     let field_types : Vec<_> = s.fields().iter().map(|ref x|
    //             x.ty.clone()).collect();



    //     // return quoted_code.parse().unwrap();
    // }






    let quoted_code = quote!{
        fn falidate() {
            println!("-----, {}!", stringify!(#struct_name));
        }
    };


    quoted_code.parse().unwrap()
}




