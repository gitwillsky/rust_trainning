use v8_live::JsRuntime;

fn main() {
    JsRuntime::init();

    //isolate
    let mut runtime = JsRuntime::new(Default::default());

    let code = r#"
        function helloWorld() {
            return {a: 1, b: 2};
        }
        console.log("hello world");
        helloWorld();
        try {
            fetch("https://www.baidu.com");
        }catch(e){
            console.log(e);
        }
    "#;

    let result = runtime.execute_script(code).unwrap();

    println!("{:#?}", result);
}
