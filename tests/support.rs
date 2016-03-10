use bindgen;
use bindgen::BindgenOptions;

use std::default::Default;

use syntax::ast;
use syntax::codemap;
use syntax::codemap::DUMMY_SP;
use syntax::parse;
use syntax::parse::token;
use syntax::print::pprust;
use syntax::ptr::P;

pub fn generate_bindings(filename: &str) -> Result<Vec<P<ast::Item>>, ()> {
    let mut options:BindgenOptions = Default::default();
    if filename.ends_with("hpp") {
        options.clang_args.push("-std=c++11".to_string());
        options.clang_args.push("-Wno-narrowing".to_string());
    }
    options.clang_args.push(filename.to_string());

    Ok(try!(bindgen::Bindings::generate(&options, None)).into_ast())
}

pub fn assert_bind_eq(filename: &str, reference_items_str: &str)
{
    let ext_cx = mk_dummy_ext_ctxt();
    let generated_items = generate_bindings(&format!("tests/{}", filename)[..]).unwrap();

    let mut parser = parse::new_parser_from_source_str(ext_cx.parse_sess(), ext_cx.cfg(), "".to_string(), reference_items_str.to_string());
    let mut reference_items = Vec::new();
    while let Some(item) = parser.parse_item().unwrap() {
        reference_items.push(item);
    }

    // The ast::Items themselves have insignificant (for our purposes)
    // differences that make them difficult to compare directly.  So, compare
    // rendered versions, which is not beautiful, but should work.
    let reference_rendered = render_items(&reference_items);
    let generated_rendered = render_items(&generated_items);

    if reference_rendered != generated_rendered {
        println!("Generated bindings for {} do not match the reference bindings.", filename);
        println!("");
        println!("Generated:");
        println!("");
        println!("{}", generated_rendered);
        println!("");
        println!("Reference:");
        println!("");
        println!("{}", reference_rendered);
        panic!();
    }
}

fn render_items(items: &Vec<P<ast::Item>>) -> String {
    pprust::to_string(|s| {
        let module = ast::Mod {
            inner: DUMMY_SP,
            items: items.clone(),
        };
        s.print_mod(&module, &[])
    })
}

pub struct DummyExtCtxt {
    sess: parse::ParseSess,
}

impl DummyExtCtxt {
    pub fn cfg(&self) -> ast::CrateConfig {
        vec!()
    }
    pub fn parse_sess(&self) -> &parse::ParseSess {
        &self.sess
    }
    pub fn call_site(&self) -> codemap::Span {
        codemap::Span {
            lo: codemap::BytePos(0),
            hi: codemap::BytePos(0),
            expn_id: codemap::NO_EXPANSION
        }
    }
    pub fn ident_of(&self, s: &str) -> ast::Ident {
        token::str_to_ident(s)
    }
    pub fn name_of(&self, s: &str) -> ast::Name {
        token::intern(s)
    }
}

fn mk_dummy_ext_ctxt<'a>() -> DummyExtCtxt {
    DummyExtCtxt { sess: parse::ParseSess::new() }
}
