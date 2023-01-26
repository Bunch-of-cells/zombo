use fyrox::{
    core::{uuid::{Uuid, uuid}, visitor::prelude::*, reflect::{FieldInfo, Reflect}},
    engine::resource_manager::ResourceManager,
    event::Event, impl_component_provider,
    scene::{node::TypeUuidProvider},
    script::{ScriptContext, ScriptDeinitContext, ScriptTrait},
};

#[derive(Visit, Reflect, Default, Debug, Clone)]
pub struct Player {
    // Add fields here.
}

impl_component_provider!(Player);

impl TypeUuidProvider for Player {
    fn type_uuid() -> Uuid {
        uuid!("5fc6ad85-388f-4b42-aa1c-bf66b9500b06")
    }
}

impl ScriptTrait for Player {
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
    