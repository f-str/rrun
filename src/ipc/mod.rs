use std::io;

mod hyprland;
mod i3wm;

pub trait IPCHandler {
    fn execute(&self, _: &String) -> io::Result<()> {
        Ok(())
    }
}

pub fn ipc_execute(command: &String) -> io::Result<()> {
    let ipc_handler = get_ipc_handler();
    ipc_handler.execute(command)
}

fn get_ipc_handler() -> Box<dyn IPCHandler> {
    if using_i3() {
        return Box::new(i3wm::I3WMIPCHandler::new());
    }

    if using_hyprland() {
        return Box::new(hyprland::HyprlandIPCHandler::new());
    }

    panic!("No supported window manager detected");
}

fn using_i3() -> bool {
    match std::env::var_os("I3SOCK") {
        // Sway sets I3SOCK for backwards compatibility
        Some(_val) => true,
        None => false,
    }
}

fn using_hyprland() -> bool {
    match std::env::var_os("HYPRLAND_INSTANCE_SIGNATURE") {
        Some(_val) => true,
        None => false,
    }
}
