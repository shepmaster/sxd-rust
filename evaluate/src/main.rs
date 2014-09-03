extern crate document;
extern crate xpath;

use std::collections::hashmap::HashMap;

use document::{Document,ToAny};

use xpath::{XPathEvaluationContext,XPathFactory};
use xpath::expression::XPathExpression;

fn main() {
    let mut args = std::os::args();

    let arg = match args.remove(1) {
        Some(x) => x,
        None => { println!("XPath required"); return; },
    };

    let factory = XPathFactory::new();

    let expr = match factory.build(arg.as_slice()) {
        Err(x) => { println!("Unable to compile XPath: {}", x); return; },
        Ok(None) => { println!("Unable to compile XPath"); return; },
        Ok(Some(x)) => x,
    };

    let d = Document::new();
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
