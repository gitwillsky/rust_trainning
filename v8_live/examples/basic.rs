use v8::Script;

fn main() {
    // initial v8 engine
    init_v8();

    // create isolate
    let isolate = &mut v8::Isolate::new(Default::default());

    // create handle scope
    let handle_scope = &mut v8::HandleScope::new(isolate);

    // create context
    let context = v8::Context::new(handle_scope);

    // create context scope
    let context_scope = &mut v8::ContextScope::new(handle_scope, context);

    let source = r#"
       function hello() { return "123" }
       hello();
    "#;
    let source = v8::String::new(context_scope, source).unwrap();
    let script = Script::compile(context_scope, source, None).unwrap();

    // run
    let result = script.run(context_scope).unwrap();

    println!("{}", result.to_rust_string_lossy(context_scope));
}

fn init_v8() {
    let platform = v8::new_default_platform(0, false).make_shared();
    v8::V8::initialize_platform(platform);
    v8::V8::initialize();
}
