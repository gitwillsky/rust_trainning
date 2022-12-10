use v8::{FunctionCallbackArguments, HandleScope, ReturnValue};

use crate::utils::execute_script;

pub struct Extensions;

const BOOTSTRAP: &str = include_str!("bootstrap.js");

impl Extensions {
    pub fn install(scope: &mut HandleScope) {
        let bindings = v8::Object::new(scope);
        let name = v8::String::new(scope, "print").unwrap();
        let func = v8::Function::new(scope, print).unwrap();
        bindings.set(scope, name.into(), func.into());

        let name = v8::String::new(scope, "fetch").unwrap();
        let func = v8::Function::new(scope, fetch).unwrap();
        bindings.set(scope, name.into(), func.into());

        if let Ok(result) = execute_script(scope, BOOTSTRAP) {
            let script_result = v8::Local::<v8::Function>::try_from(result).unwrap();
            let recv = v8::undefined(scope).into();
            let args = [bindings.into()];
            script_result.call(scope, recv, &args).unwrap();
        }
    }
}

fn print(scope: &mut HandleScope, args: FunctionCallbackArguments, mut rv: ReturnValue) {
    let params: serde_json::Value = serde_v8::from_v8(scope, args.get(0)).unwrap();
    println!("{params:#?}");
    rv.set_undefined();
}

fn fetch(scope: &mut HandleScope, args: FunctionCallbackArguments, mut rv: ReturnValue) {
    let url: String = serde_v8::from_v8(scope, args.get(0)).unwrap();
    match reqwest::blocking::get(url) {
        Ok(result) => rv.set(serde_v8::to_v8(scope, result.text().unwrap()).unwrap()),
        Err(e) => {
            let exception_value = serde_v8::to_v8(scope, e.to_string()).unwrap();
            scope.throw_exception(exception_value);
        }
    };
}
