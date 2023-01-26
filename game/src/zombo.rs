
use fyrox::{
    core::{uuid::{Uuid, uuid}, visitor::prelude::*, reflect::{FieldInfo, Reflect}},
    event::Event, impl_component_provider,
    scene::{node::TypeUuidProvider},
    script::{ScriptContext, ScriptDeinitContext, ScriptTrait},
};

#[derive(Visit, Reflect, Default, Debug, Clone)]
pub struct Zombo {
    // Add fields here.
}

impl_component_provider!(Zombo);

impl TypeUuidProvider for Zombo {
    fn type_uuid() -> Uuid {
        uuid!("32e406b5-8917-4d02-abae-bd1c12fbd8e7")
    }
}

impl ScriptTrait for Zombo {
    fn on_init(&mut self, _context: &mut ScriptContext) {
        // Put initialization logic here.
    }

    fn on_start(&mut self, _context: &mut ScriptContext) {
        // There should be a logic that depends on other scripts in scene.
        // It is called right after **all** scripts were initialized.
    }

    fn on_deinit(&mut self, _context: &mut ScriptDeinitContext) {
        // Put de-initialization logic here.
    }

    fn on_os_event(&mut self, _event: &Event<()>, _context: &mut ScriptContext) {
        // Respond to OS events here.
    }

    fn on_update(&mut self, _context: &mut ScriptContext) {
        // Put object logic here.
    }

    fn id(&self) -> Uuid {
        Self::type_uuid()
    }
}
