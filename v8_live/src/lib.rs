use state::JsRuntimeState;
use utils::execute_script;
use v8::{CreateParams, HandleScope, Isolate, OwnedIsolate};

mod extensions;
mod state;
mod utils;

type V8LocalValue<'s> = v8::Local<'s, v8::Value>;

pub struct JsRuntime {
    isolate: OwnedIsolate,
}

#[derive(Default)]
pub struct JsRuntimeParams(CreateParams);

impl JsRuntimeParams {
    pub fn new(_snapshot: Option<Vec<u8>>) -> Self {
        JsRuntimeParams(Default::default())
    }

    pub fn into_inner(self) -> CreateParams {
        self.0
    }
}

impl JsRuntime {
    pub fn init() {
        let platform = v8::new_default_platform(0, false).make_shared();
        v8::V8::initialize_platform(platform);
        v8::V8::initialize();
    }

    pub fn new(options: JsRuntimeParams) -> Self {
        let isolate = Isolate::new(options.into_inner());
        JsRuntime::init_isolate(isolate)
    }

    pub fn execute_script(
        &mut self,
        code: impl AsRef<str>,
    ) -> Result<serde_json::Value, serde_json::Value> {
        let context = JsRuntimeState::get_context(&mut self.isolate);
        let handle_scope = &mut HandleScope::with_context(&mut self.isolate, context);

        match execute_script(handle_scope, code) {
            Ok(v) => Ok(serde_v8::from_v8(handle_scope, v).unwrap()),
            Err(e) => Err(serde_v8::from_v8(handle_scope, e).unwrap()),
        }
    }

    pub fn create_snapshot(&self) -> Vec<u8> {
        todo!()
    }

    fn init_isolate(mut isolate: OwnedIsolate) -> Self {
        let state = JsRuntimeState::new(&mut isolate);
        isolate.set_slot(state);
        {
            let context = JsRuntimeState::get_context(&mut isolate);
            let scope = &mut HandleScope::with_context(&mut isolate, context);
            extensions::Extensions::install(scope);
        }
        JsRuntime { isolate }
    }
}
