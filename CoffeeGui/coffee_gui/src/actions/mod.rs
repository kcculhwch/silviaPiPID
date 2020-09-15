use std::sync::mpsc::{Sender, Receiver, channel};
use super::gui_tk::GuiAction;

use std::thread;
use std::collections::HashMap;
use std::time::{Duration};

pub fn run_controller(mut root_controller: RootController) {
    let controller_thread = thread::spawn(move || {
        loop {                
            let gui_action: GuiAction = match root_controller.action_receiver.try_recv() {
                Ok(action) => action,
                Err(_) => GuiAction::new("", None)
            };
            root_controller.handle_action(&gui_action);
            thread::sleep(Duration::from_millis(5));
        };
    });
    match controller_thread.join() {
        Ok(_) => (),
        Err(_) => ()
    }
}


#[allow(dead_code)]
pub struct RootController {
    controllers: HashMap<&'static str, Controller>,
    pub action_receiver: Receiver<GuiAction>,
    action_sender: Sender<GuiAction>
}

impl RootController {
    pub fn new() -> RootController {
        let (sender, receiver) = channel();
        let controllers: HashMap<&'static str, Controller> = HashMap::new();
        RootController {
            controllers,
            action_sender: sender,
            action_receiver: receiver
        }
    }

    pub fn handle_action(&mut self, action: &GuiAction) {
        match action.name {
            _ => ()
        }
    }
    pub fn get_action_sender(&self) -> Sender<GuiAction> {
        self.action_sender.clone()
    }

}

#[allow(dead_code)]
pub struct Controller {
    actions: HashMap<&'static str, Action>
}

#[allow(dead_code)]
impl Controller {
    pub fn new() -> Controller {
        let actions: HashMap<&'static str, Action> = HashMap::new();
        Controller {
            actions
        }
    }
}

pub type Action = fn();


// router  action loop

// recieve action events from gui

// send mutators to change state data

// 

