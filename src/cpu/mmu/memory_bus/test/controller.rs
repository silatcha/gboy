use std::cell::Cell;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::{joystick, controller};
use sdl2::GameControllerSubsystem;
use sdl2::controller::{GameController, Button, Axis};
use sdl2::Sdl;

use crate::cpu::mmu::ButtonState;
use crate::cpu::mmu::Buttons;
use crate::cpu::mmu;

use crate::cpu::mmu::memory_bus;

pub struct Controller {
    buttons:      Cell<Buttons>,
    #[allow(dead_code)]
    x_axis_state: Cell<AxisState>,
    y_axis_state: Cell<AxisState>,
}

impl Controller {
    pub fn new() -> Controller {
        // Attempt to add a game controller

       
       

        Controller {
            buttons:      Cell::new(Buttons::new(ButtonState::Up)),
            x_axis_state: Cell::new(AxisState::Neutral),
            y_axis_state: Cell::new(AxisState::Neutral),
        }
    }

    pub fn update(&self, sdl2: &Sdl) -> mmu::Event {
        let mut event = mmu::Event::None;

        let mut event_pump = sdl2.event_pump();

        for e in event_pump.expect("error in sending sld in update").poll_iter() {
            match e {
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } =>
                    event = mmu::Event::PowerOff,
                Event::KeyDown { keycode: Some(key), .. } =>
                    self.update_key(key, ButtonState::Down),
                Event::KeyUp { keycode: Some(key), .. } =>
                    self.update_key(key, ButtonState::Up),
                Event::ControllerButtonDown{ button, .. } =>
                    self.update_button(button, ButtonState::Down),
                Event::ControllerButtonUp{ button, .. } =>
                    self.update_button(button, ButtonState::Up),
                Event::ControllerAxisMotion{ axis, value: val, .. } =>
                    self.update_axis(axis, val),
                Event::Quit { .. } =>
                    event = mmu::Event::PowerOff,
                _ => ()
            }
        }

        event
    }

    pub fn buttons(&self) -> &Cell<Buttons> {
        &self.buttons
    }

    /// Update key state. For now keybindings are hardcoded.
    fn update_key(&self, key: Keycode, state: ButtonState) {
        let mut b = self.buttons.get();

        match key {
            Keycode::Up        => b.up     = state,
            Keycode::Down      => b.down   = state,
            Keycode::Left      => b.left   = state,
            Keycode::Right     => b.right  = state,
            Keycode::LAlt      => b.a      = state,
            Keycode::LCtrl     => b.b      = state,
            Keycode::Return    => b.start  = state,
            Keycode::RShift    => b.select = state,
            _                  => (),
        }

        self.buttons.set(b);
    }

    /// Same as update_key but for controller buttons
    fn update_button(&self, button: Button, state: ButtonState) {
        let mut b = self.buttons.get();

        match button {
            Button::A         => b.a      = state,
            Button::B         => b.b      = state,
            Button::DPadLeft  => b.left   = state,
            Button::DPadRight => b.right  = state,
            Button::DPadUp    => b.up     = state,
            Button::DPadDown  => b.down   = state,
            Button::Start     => b.start  = state,
            Button::Back      => b.select = state,
            _                 => (),
        }

        self.buttons.set(b);
    }

    /// Map left stick X/Y to directional buttons
    fn update_axis(&self, axis: Axis, val: i16) {
        let mut b = self.buttons.get();

        let state = AxisState::from_value(val);

        match axis {
            Axis::LeftX => {
                if state != self.x_axis_state.get() {
                    self.x_axis_state.set(state);

                    b.left  = state.down_if_negative();
                    b.right = state.down_if_positive();
                }
            }
            Axis::LeftY => {
                if state != self.y_axis_state.get() {
                    self.y_axis_state.set(state);

                    b.up   = state.down_if_negative();
                    b.down = state.down_if_positive();
                }
            }
            _ => (),
        }

        self.buttons.set(b);
    }
}

#[derive(Clone,Copy,PartialEq,Eq)]
enum AxisState {
    Neutral,
    Negative,
    Positive,
}

impl AxisState {
    fn from_value(val: i16) -> AxisState {
        if val > AXIS_DEAD_ZONE {
            AxisState::Positive
        } else if val < -AXIS_DEAD_ZONE {
            AxisState::Negative
        } else {
            AxisState::Neutral
        }
    }

    fn down_if_negative(self) -> ButtonState {
        if self == AxisState::Negative {
            ButtonState::Down
        } else {
            ButtonState::Up
        }
    }

    fn down_if_positive(self) -> ButtonState {
        if self == AxisState::Positive {
            ButtonState::Down
        } else {
            ButtonState::Up
        }
    }
}

/// The controller axis moves in a range from -32768 to +32767. To
/// avoid spurious events this constant says how far from 0 the axis
/// has to move for us to register the event.
const AXIS_DEAD_ZONE: i16 = 10_000;