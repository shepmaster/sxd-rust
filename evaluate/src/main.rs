extern crate document;
extern crate xpath;

use std::io::File;

use std::collections::hashmap::HashMap;

use document::ToAny;
use document::parser::Parser;

use xpath::{XPathEvaluationContext,XPathFactory};
use xpath::expression::XPathExpression;

fn main() {
    let mut args = std::os::args();

    let filename = match args.remove(1) {
        Some(x) => x,
        None => { println!("File required"); return; },
    };

    let xpath_str = match args.remove(1) {
        Some(x) => x,
        None => { println!("XPath required"); return; },
    };

    let factory = XPathFactory::new();

    let expr = match factory.build(xpath_str.as_slice()) {
        Err(x) => { println!("Unable to compile XPath: {}", x); return; },
        Ok(None) => { println!("Unable to compile XPath"); return; },
        Ok(Some(x)) => x,
    };

    let p = Parser::new();

    let path = Path::new(filename);
    let mut file = File::open(&path);

    let data = match file.read_to_end() {
        Ok(x) => String::from_utf8(x),
        Err(x) => { println!("Can't read: {}", x); return; },
    };

    let data = data.ok().expect("Unable to convert to UTF-8");

    let d = p.parse(data.as_slice());

    let mut functions = HashMap::new();
    xpath::function::register_core_functions(& mut functions);
    let variables = HashMap::new();
    let mut context = XPathEvaluationContext::new(d.root().to_any(),
                                                  &functions,
                                                  &variables);
    context.next(d.root().to_any());

    let res = expr.evaluate(&context);

    println!("{}", res);
}
