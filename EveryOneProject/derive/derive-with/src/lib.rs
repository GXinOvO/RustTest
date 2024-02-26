use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::{format_ident, quote};
use syn::parse::Parse;
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{Attribute, Index, Meta, Token};

/// A custom derive implementation for `#[derive(With)]`
///
/// # Get started
///
/// 1.Generate with-constructor for each field
/// ```rust
/// use derive_with::With;
///
/// #[derive(With, Default)]
/// pub struct Foo {
///     pub a: i32,
///     pub b: String,
/// }
///
/// #[derive(With, Default)]
/// pub struct Bar (i32, String);
///
/// #[test]
/// fn test_struct() {
///     let foo = Foo::default().with_a(1).with_b(1.to_string());
///     assert_eq!(foo.a, 1);
///     assert_eq!(foo.b, "1".to_string());
///
///     let bar = Bar::default().with_0(1).with_1(1.to_string());
///     assert_eq!(bar.0, 1);
///     assert_eq!(bar.1, "1".to_string());
/// }
/// ```
///
/// 2.Generate with-constructor for specific fields
/// ```rust
/// use derive_with::With;
///
/// #[derive(With, Default)]
/// #[with(a)]
/// pub struct Foo {
///     pub a: i32,
///     pub b: String,
/// }
///
/// #[derive(With, Default)]
/// #[with(1)]
/// pub struct Bar (i32, String);
///
/// #[test]
/// fn test_struct() {
///     let foo = Foo::default().with_a(1);
///     assert_eq!(foo.a, 1);
///
///     let bar = Bar::default().with_1(1.to_string());
///     assert_eq!(bar.1, "1".to_string());
/// }
/// ```

/*
--TODO: proc_macro_derive 
    属性宏，并定义了一个名为`With`的派生宏。这个宏可以用于标记结构体、枚举或联合体，并为他们自动生成一些相关代码。

    proc_macro_derive用于创建派生宏的属性宏的特性。接受两个参数: 第一个参数为派生宏的名称，第二参数是用于注解的属性列表

        attributes(with)表示你的派生宏可以使用with属性进行注解。这意味着你可以在结构体、枚举或联合体上使用#[with]注解，以
    便在派生宏的实现中使用该属性


    DeriveInput是syn库中的一个结构体，表示派生宏的输入。它包含了派生宏的名称、类型、属性等信息
*/
#[proc_macro_derive(With, attributes(with))]
pub fn derive(input: TokenStream) -> TokenStream {
    // -> 将其转换为syn::DeriveInput类型的AST(抽象语法树).syn是一个常用的用于解析Rust代码的库。
    let ast: syn::DeriveInput = syn::parse(input).expect("Couldn't parse item");
    /*
    --TODO: syn::Data
        · syn::Data::Struct时，执行with_for_struct
        · 枚举和联合体，会抛出相应的异常。
     */
    let result = match ast.data {
        syn::Data::Struct(ref s) => with_for_struct(&ast, &s.fields),
        syn::Data::Enum(_) => panic!("doesn't work with enums yet"),
        syn::Data::Union(_) => panic!("doesn't work with unions yet"),
    };
    // -> 将result转换成TokenStream类型
    result.into()
}


/*
    syn::Fields::Named: 结构体拥有具名字段。
    syn::Fields::Unnamed: 结构体拥有未命名字段
    syn::Fields::Unit: 结构体是一个单元结构体(没有字段)，就抛出错误
*/
fn with_for_struct(ast: &syn::DeriveInput, fields: &syn::Fields) -> proc_macro2::TokenStream {

    match *fields {
        syn::Fields::Named(ref fields) => with_constructor_for_named(ast, &fields.named),
        syn::Fields::Unnamed(ref fields) => with_constructor_for_unnamed(ast, &fields.unnamed),
        syn::Fields::Unit => panic!("Unit structs are not supported"),
    }
}

/*
--TODO:
    用于为具有具名字段的结构体生成构造函数相关的代码

    Punctuated<syn::Field, Token![,]>表示具名字段的集合


--TODO: quote!
        用于生成Rust代码片段。它允许我们在代码中嵌入Rust语法树，并以字符串形
    式进行操作和生成。可以通过quote::quote函数创建一个代码片段。

--TODO: TokenStream
    用于表示一系列Rust代码片段。
*/
fn with_constructor_for_named(
    ast: &syn::DeriveInput,
    fields: &Punctuated<syn::Field, Token![,]>,
) -> proc_macro2::TokenStream {
    // -> name存储结构体的名称，类型为&syn::Ident
    let name = &ast.ident;
    println!("name: {:#?}", name);
    // -> (存储结构体的泛型参数)，用于生成代码时使用。
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

    // -> 用于从结构体的属性中解析处标记了with的字段名称。
    let with_args = parse_with_args::<Ident>(&ast.attrs);

    /*
    --TODO:
        遍历具名字段，并为每个标记了with的字段生成构造函数
     */
    // -> 初始值时一个空的quote!()，用于创建一个空的代码片段。
    let mut constructors = quote!();
    for field in fields {
        // -> .as_ref().unwrap()来解引用Option类型的字段标识符field.ident
        // -> as_ref() 将Option或Result等值转换为引用。
        let field_name = field.ident.as_ref().unwrap();
        println!("field_name: {:?}", field_name);
        // -> 检查with_args中是否包含当前字段的名称。
        if !contains_field(&with_args, field_name) {
            continue;
        }
        // -> 使用&field.ty获取字段的类型
        let field_type = &field.ty;

        // -> 生成一个以with_开头的构造函数名称constructor_name
        let constructor_name = format_ident!("with_{}", field_name);
        println!("constructor_name: {}", constructor_name);
        /*
            使用quote!宏生成构造函数的代码片段constructor_name。
         */
        let constructor = quote! {
            pub fn #constructor_name(mut self, #field_name: impl Into<#field_type>) -> Self {
                self.#field_name = #field_name.into();
                self
            }
        };
        // println!("constructor: {:?}", constructor);
        // 使用quote!宏将生成的构造函数代码片段constructor添加到constructors中。
        constructors = quote! {
            #constructors
            #constructor
        };
        // println!("constructors: {:#?}", constructors);
    }
    // -> 使用quote!宏生成结构体的实现代码片段，这个实现代码包含了之前生成的所有构造函数
    quote! {
        impl #impl_generics #name #ty_generics #where_clause {
            #constructors
        }
    }
}

fn with_constructor_for_unnamed(
    ast: &syn::DeriveInput,
    fields: &Punctuated<syn::Field, Token![,]>,
) -> proc_macro2::TokenStream {
    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();
    let with_args = parse_with_args::<Index>(&ast.attrs);

    let mut constructors = quote!();
    for (index, field) in fields.iter().enumerate() {
        let index = syn::Index::from(index);
        if !contains_field(&with_args, &index) {
            continue;
        }
        let field_type = &field.ty;
        let param_name = format_ident!("field_{}", index);
        let constructor_name = format_ident!("with_{}", index);

        let constructor = quote! {
            pub fn #constructor_name(mut self, #param_name: impl Into<#field_type>) -> Self {
                self.#index = #param_name.into();
                self
            }
        };
        constructors = quote! {
            #constructors
            #constructor
        };
    }
    quote! {
        impl #impl_generics #name #ty_generics #where_clause {
            #constructors
        }
    }
}

/*
--TODO:
    解析属性
*/
fn parse_with_args<T: Parse>(attrs: &Vec<Attribute>) -> Option<Punctuated<T, Comma>> {
    // -> 在属性列表中查找标记为with的属性。iter()方法用于遍历属性列表; find()方法用于查找满足给的那个条件的第一个属性
    if let Some(attr) = attrs.iter().find(|attr| attr.path().is_ident("with")) {
        // -> 获取属性的原属性
        match &attr.meta {
            // -> 如果元信息是一个列表，则解析列表中的参数，并返回解析结构。Punctuated::parse_terminated用于解析带逗号分隔的参数列表。
            Meta::List(list) => Some(
                list.parse_args_with(Punctuated::<T, Comma>::parse_terminated)
                    .expect("Couldn't parse with args"),
            ),
            // -> 如果不是，则抛出一个panic
            _ => panic!("`with` attribute should like `#[with(a, b, c)]`"),
        }
    } else {
        None
    }
}

/*
--TODO:
    检查字段是否存在于给定的参数中

    使用了泛型参数T，并要求T实现了Parse和PartialEq两个trait
*/
fn contains_field<T: Parse + PartialEq>(
    with_args: &Option<Punctuated<T, Comma>>,
    item: &T,
) -> bool {
    /*
        is_none()检查给定的参数是否为None。
        .as_ref().unwrap()用于获取参数的引用并解包成可迭代的形式
        iter()用于获取参数的迭代器
        any(|arg| arg == item)用于检查迭代器是否存在与给定字段相等的参数。
     */
    with_args.is_none() || with_args.as_ref().unwrap().iter().any(|arg| arg == item)
}
