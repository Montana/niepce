/*
 * niepce - crates/npc-fwk/src/toolkit/controller.rs
 *
 * Copyright (C) 2022-2024 Hubert Figuière
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

use std::cell::{Ref, RefMut};
use std::rc::Rc;

/// Use this macro inside the impl to implement `imp()` and `imp_mut()`
///
/// ```rust,ignore
/// impl Controller for MyController {
///     npc_fwk::controller_imp_imp!(imp_)
/// }
/// ```
#[macro_export]
macro_rules! controller_imp_imp {
    ( $f:ident ) => {
        fn imp(&self) -> std::cell::Ref<'_, $crate::toolkit::ControllerImpl<Self::InMsg>> {
            self.$f.borrow()
        }

        fn imp_mut(&self) -> std::cell::RefMut<'_, $crate::toolkit::ControllerImpl<Self::InMsg>> {
            self.$f.borrow_mut()
        }
    };
}

pub struct ControllerImpl<T> {
    tx: super::Sender<T>,
    rx: super::Receiver<T>,
}

impl<T> Default for ControllerImpl<T> {
    fn default() -> ControllerImpl<T> {
        let (tx, rx) = super::channel();
        ControllerImpl::new(tx, rx)
    }
}

impl<T> ControllerImpl<T> {
    fn new(tx: super::Sender<T>, rx: super::Receiver<T>) -> Self {
        ControllerImpl { tx, rx }
    }
}

/// Controller allow encapsulating functionnality and receive message
/// to call it.
///
/// `InMsg` is the type of messages being sent (inbound) to it.
///
/// Implement [`Controller::dispatch`] to process the inbound
/// messages, and call [`Controller::start`] to set it up.  If the
/// controller is not supposed to receive inbound messages, `InMsg` to
/// `()`. Don't forget to call `controller_imp_imp!()` in your
/// implementation to generate some boilerplate.
///
/// ```rust,ignore
/// enum MyMsg {
///     Command1,
///     Command2(String),
/// }
///
/// struct MyController {
///     imp_: ControllerImpl<Self::InMsg>,
/// }
///
/// impl Controller for MyController {
///     npc_fwk::controller_imp_imp!(imp_)
///
///     type InMsg = MyMsg;
///
///     fn dispatch(&self, msg: MyMsg) {
///         match msg {
///            MyMsg::Command1 => {}
///            MyMsg::Command2(s) => println!("{}", s),
///         }
///     }
/// }
/// ```
/// To send a message, call `Controller::emit()`.
///
/// To get the sender to pass elsewhere call
/// `Controller::sender()`. The sender can be cloned.
///
///
/// ```rust,ignore
/// let ctrl = MyController {};
///
/// let sender = ctrl.sender();
/// ctrl.start();
///
/// ctrl.send(MyMsg::Command1);
/// sender.send(MyMsg::Command2("test".to_string())).await;
/// ```
pub trait Controller {
    type InMsg;

    /// Start the controller event loop. This should be called by the
    /// parent after creating the controller, if needed (i.e. need to
    /// process inbound messages).
    ///
    /// This default implementation ought to be enough.
    fn start<T: Controller + 'static>(this: &Rc<T>) {
        let rx = this.imp().rx.clone();
        super::channels::receiver_attach(
            rx,
            glib::clone!(@weak this as ctrl => move |e| {
                ctrl.dispatch(e)
            }),
        );
    }

    /// Get the sender for the controller inbound messages.
    fn sender(&self) -> super::Sender<Self::InMsg> {
        self.imp().tx.clone()
    }

    /// Send an inbound message.
    ///
    /// XXX not sure about thread location. This is meant to be on the
    /// local context (main)
    fn send(&self, msg: Self::InMsg)
    where
        Self::InMsg: 'static,
    {
        super::send_async_local!(msg, self.imp().tx);
    }

    /// Notify the controller is ready.
    fn ready(&self) {
        dbg_out!("ready");
        self.on_ready();
    }

    /// What to do when ready.
    fn on_ready(&self) {}

    /// Dispatch input message, called by the event loop. See [`Controller::start`]
    fn dispatch(&self, _message: Self::InMsg) {}

    /// Return the implementation
    /// Implemented via controller_imp_imp!()
    fn imp(&self) -> Ref<'_, ControllerImpl<Self::InMsg>>;
    /// Return the mutable implementation
    /// Implemented via controller_imp_imp!()
    fn imp_mut(&self) -> RefMut<'_, ControllerImpl<Self::InMsg>>;
}
