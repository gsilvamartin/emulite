//! Input system for handling controllers and keyboard input

use crate::{EmuliteResult, EmuliteError, config::InputConfig as ConfigInputConfig};
use gilrs::{Gilrs, Button, Axis, Gamepad};
use winit::event::{KeyboardInput, ElementState, VirtualKeyCode};
use std::collections::HashMap;

/// Input configuration
#[derive(Debug, Clone)]
pub struct InputConfig {
    pub keyboard_mapping: HashMap<VirtualKeyCode, InputButton>,
    pub gamepad_mapping: HashMap<Button, InputButton>,
    pub axis_mapping: HashMap<Axis, InputAxis>,
    pub deadzone: f32,
    pub sensitivity: f32,
    pub auto_fire: bool,
    pub turbo_speed: u32,
}

impl Default for InputConfig {
    fn default() -> Self {
        let mut keyboard_mapping = HashMap::new();
        keyboard_mapping.insert(VirtualKeyCode::Up, InputButton::Up);
        keyboard_mapping.insert(VirtualKeyCode::Down, InputButton::Down);
        keyboard_mapping.insert(VirtualKeyCode::Left, InputButton::Left);
        keyboard_mapping.insert(VirtualKeyCode::Right, InputButton::Right);
        keyboard_mapping.insert(VirtualKeyCode::Z, InputButton::A);
        keyboard_mapping.insert(VirtualKeyCode::X, InputButton::B);
        keyboard_mapping.insert(VirtualKeyCode::Return, InputButton::Start);
        keyboard_mapping.insert(VirtualKeyCode::Space, InputButton::Select);
        
        let mut gamepad_mapping = HashMap::new();
        gamepad_mapping.insert(Button::DPadUp, InputButton::Up);
        gamepad_mapping.insert(Button::DPadDown, InputButton::Down);
        gamepad_mapping.insert(Button::DPadLeft, InputButton::Left);
        gamepad_mapping.insert(Button::DPadRight, InputButton::Right);
        gamepad_mapping.insert(Button::South, InputButton::A);
        gamepad_mapping.insert(Button::East, InputButton::B);
        gamepad_mapping.insert(Button::Start, InputButton::Start);
        gamepad_mapping.insert(Button::Select, InputButton::Select);
        
        let mut axis_mapping = HashMap::new();
        axis_mapping.insert(Axis::LeftStickX, InputAxis::LeftX);
        axis_mapping.insert(Axis::LeftStickY, InputAxis::LeftY);
        axis_mapping.insert(Axis::RightStickX, InputAxis::RightX);
        axis_mapping.insert(Axis::RightStickY, InputAxis::RightY);
        
        Self {
            keyboard_mapping,
            gamepad_mapping,
            axis_mapping,
            deadzone: 0.1,
            sensitivity: 1.0,
            auto_fire: false,
            turbo_speed: 10,
        }
    }
}

/// Input button enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InputButton {
    Up,
    Down,
    Left,
    Right,
    A,
    B,
    X,
    Y,
    L,
    R,
    Start,
    Select,
    L1,
    R1,
    L2,
    R2,
    L3,
    R3,
    Home,
    Back,
    Menu,
}

/// Input axis enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InputAxis {
    LeftX,
    LeftY,
    RightX,
    RightY,
    L2,
    R2,
}

/// Input state for a single controller
#[derive(Debug, Clone)]
pub struct InputState {
    pub buttons: HashMap<InputButton, bool>,
    pub axes: HashMap<InputAxis, f32>,
    pub connected: bool,
}

impl Default for InputState {
    fn default() -> Self {
        let mut buttons = HashMap::new();
        for button in [
            InputButton::Up, InputButton::Down, InputButton::Left, InputButton::Right,
            InputButton::A, InputButton::B, InputButton::X, InputButton::Y,
            InputButton::L, InputButton::R, InputButton::Start, InputButton::Select,
            InputButton::L1, InputButton::R1, InputButton::L2, InputButton::R2,
            InputButton::L3, InputButton::R3, InputButton::Home, InputButton::Back,
            InputButton::Menu,
        ] {
            buttons.insert(button, false);
        }
        
        let mut axes = HashMap::new();
        for axis in [InputAxis::LeftX, InputAxis::LeftY, InputAxis::RightX, InputAxis::RightY, InputAxis::L2, InputAxis::R2] {
            axes.insert(axis, 0.0);
        }
        
        Self {
            buttons,
            axes,
            connected: false,
        }
    }
}

/// Input system for handling all input devices
pub struct InputSystem {
    config: InputConfig,
    gilrs: Option<Gilrs>,
    keyboard_state: HashMap<VirtualKeyCode, bool>,
    gamepad_states: HashMap<gilrs::GamepadId, InputState>,
    should_exit: bool,
}

impl InputSystem {
    pub fn new(config: &ConfigInputConfig) -> EmuliteResult<Self> {
        let mut keyboard_mapping = HashMap::new();
        for (key_str, button_str) in &config.keyboard_mapping {
            if let Ok(key) = Self::string_to_keycode(key_str) {
                if let Ok(button) = Self::string_to_button(button_str) {
                    keyboard_mapping.insert(key, button);
                }
            }
        }
        
        let mut gamepad_mapping = HashMap::new();
        for (button_str, input_str) in &config.gamepad_mapping {
            if let Ok(button) = Self::string_to_gilrs_button(button_str) {
                if let Ok(input_button) = Self::string_to_button(input_str) {
                    gamepad_mapping.insert(button, input_button);
                }
            }
        }
        
        let input_config = InputConfig {
            keyboard_mapping,
            gamepad_mapping,
            axis_mapping: HashMap::new(),
            deadzone: config.deadzone,
            sensitivity: config.sensitivity,
            auto_fire: config.auto_fire,
            turbo_speed: config.turbo_speed,
        };
        
        Self::new_with_input_config(&input_config)
    }
    
    pub fn new_with_input_config(config: &InputConfig) -> EmuliteResult<Self> {
        let gilrs = Gilrs::new().ok();
        let mut gamepad_states = HashMap::new();
        
        if let Some(ref g) = gilrs {
            for (id, _) in g.gamepads() {
                gamepad_states.insert(id, InputState::default());
            }
        }
        
        Ok(Self {
            config: config.clone(),
            gilrs,
            keyboard_state: HashMap::new(),
            gamepad_states,
            should_exit: false,
        })
    }
    
    /// Update input system (called each frame)
    pub fn update(&mut self) -> EmuliteResult<()> {
        // Update gamepad states
        if let Some(gilrs) = &mut self.gilrs {
            while let Some(gilrs::Event { id, event, time: _ }) = gilrs.next_event() {
                match event {
                    gilrs::EventType::ButtonPressed(button, _) => {
                        if let Some(input_button) = self.config.gamepad_mapping.get(&button) {
                            if let Some(state) = self.gamepad_states.get_mut(&id) {
                                state.buttons.insert(*input_button, true);
                            }
                        }
                    },
                    gilrs::EventType::ButtonReleased(button, _) => {
                        if let Some(input_button) = self.config.gamepad_mapping.get(&button) {
                            if let Some(state) = self.gamepad_states.get_mut(&id) {
                                state.buttons.insert(*input_button, false);
                            }
                        }
                    },
                    gilrs::EventType::AxisChanged(axis, value, _) => {
                        if let Some(input_axis) = self.config.axis_mapping.get(&axis) {
                            if let Some(state) = self.gamepad_states.get_mut(&id) {
                                let deadzone = self.config.deadzone;
                                let processed_value = if value.abs() < deadzone {
                                    0.0
                                } else {
                                    (value - deadzone * value.signum()) / (1.0 - deadzone)
                                };
                                state.axes.insert(*input_axis, processed_value * self.config.sensitivity);
                            }
                        }
                    },
                    gilrs::EventType::Connected => {
                        if let Some(state) = self.gamepad_states.get_mut(&id) {
                            state.connected = true;
                        }
                    },
                    gilrs::EventType::Disconnected => {
                        if let Some(state) = self.gamepad_states.get_mut(&id) {
                            state.connected = false;
                            *state = InputState::default();
                        }
                    },
                    _ => {}
                }
            }
        }
        
        Ok(())
    }
    
    /// Handle keyboard input
    pub fn handle_keyboard_input(&mut self, input: KeyboardInput) -> EmuliteResult<()> {
        if let Some(keycode) = input.virtual_keycode {
            let pressed = input.state == ElementState::Pressed;
            self.keyboard_state.insert(keycode, pressed);
            
            // Check for exit key
            if keycode == VirtualKeyCode::Escape && pressed {
                self.should_exit = true;
            }
        }
        
        Ok(())
    }
    
    /// Check if a button is pressed
    pub fn is_button_pressed(&self, controller: usize, button: InputButton) -> bool {
        // Check keyboard first
        for (keycode, mapped_button) in &self.config.keyboard_mapping {
            if *mapped_button == button {
                if let Some(&pressed) = self.keyboard_state.get(keycode) {
                    if pressed {
                        return true;
                    }
                }
            }
        }
        
        // Check gamepad
        for (_, state) in &self.gamepad_states {
            if state.connected {
                if state.buttons.get(&button).copied().unwrap_or(false) {
                    return true;
                }
            }
        }
        
        false
    }
    
    /// Get axis value
    pub fn get_axis_value(&self, controller: usize, axis: InputAxis) -> f32 {
        for (_, state) in &self.gamepad_states {
            if state.connected {
                return state.axes.get(&axis).copied().unwrap_or(0.0);
            }
        }
        0.0
    }
    
    /// Check if controller is connected
    pub fn is_controller_connected(&self, controller: usize) -> bool {
        for (_, state) in &self.gamepad_states {
            if state.connected {
                return true;
            }
        }
        false
    }
    
    /// Get number of connected controllers
    pub fn connected_controller_count(&self) -> usize {
        self.gamepad_states.iter()
            .filter(|(_, state)| state.connected)
            .count()
    }
    
    /// Check if should exit
    pub fn should_exit(&self) -> bool {
        self.should_exit
    }
    
    /// Set button mapping
    pub fn set_keyboard_mapping(&mut self, keycode: VirtualKeyCode, button: InputButton) {
        self.config.keyboard_mapping.insert(keycode, button);
    }
    
    /// Set gamepad button mapping
    pub fn set_gamepad_mapping(&mut self, gamepad_button: Button, button: InputButton) {
        self.config.gamepad_mapping.insert(gamepad_button, button);
    }
    
    /// Set axis mapping
    pub fn set_axis_mapping(&mut self, axis: Axis, input_axis: InputAxis) {
        self.config.axis_mapping.insert(axis, input_axis);
    }
    
    /// Set deadzone
    pub fn set_deadzone(&mut self, deadzone: f32) {
        self.config.deadzone = deadzone.clamp(0.0, 1.0);
    }
    
    /// Set sensitivity
    pub fn set_sensitivity(&mut self, sensitivity: f32) {
        self.config.sensitivity = sensitivity.clamp(0.1, 5.0);
    }
    
    /// Get current configuration
    pub fn config(&self) -> &InputConfig {
        &self.config
    }
    
    /// Get controller name
    pub fn get_controller_name(&self, controller: usize) -> Option<String> {
        if let Some(gilrs) = &self.gilrs {
            // Get the first available gamepad ID
            if let Some((id, _)) = gilrs.gamepads().next() {
                let gamepad = gilrs.gamepad(id);
                return Some(gamepad.name().to_string());
            }
        }
        None
    }
    
    /// Get controller info
    pub fn get_controller_info(&self, controller: usize) -> Option<ControllerInfo> {
        if let Some(gilrs) = &self.gilrs {
            // Get the first available gamepad ID
            if let Some((id, _)) = gilrs.gamepads().next() {
                let gamepad = gilrs.gamepad(id);
                return Some(ControllerInfo {
                    name: gamepad.name().to_string(),
                    connected: self.is_controller_connected(controller),
                    button_count: 16, // Default button count
                    axis_count: 6,    // Default axis count
                });
            }
        }
        None
    }
    
    fn string_to_keycode(s: &str) -> Result<VirtualKeyCode, ()> {
        match s {
            "Up" => Ok(VirtualKeyCode::Up),
            "Down" => Ok(VirtualKeyCode::Down),
            "Left" => Ok(VirtualKeyCode::Left),
            "Right" => Ok(VirtualKeyCode::Right),
            "Space" => Ok(VirtualKeyCode::Space),
            "Return" => Ok(VirtualKeyCode::Return),
            "Escape" => Ok(VirtualKeyCode::Escape),
            _ => Err(()),
        }
    }
    
    fn string_to_button(s: &str) -> Result<InputButton, ()> {
        match s {
            "Up" => Ok(InputButton::Up),
            "Down" => Ok(InputButton::Down),
            "Left" => Ok(InputButton::Left),
            "Right" => Ok(InputButton::Right),
            "A" => Ok(InputButton::A),
            "B" => Ok(InputButton::B),
            "Start" => Ok(InputButton::Start),
            "Select" => Ok(InputButton::Select),
            _ => Err(()),
        }
    }
    
    fn string_to_gilrs_button(s: &str) -> Result<Button, ()> {
        match s {
            "South" => Ok(Button::South),
            "East" => Ok(Button::East),
            "North" => Ok(Button::North),
            "West" => Ok(Button::West),
            "LeftTrigger" => Ok(Button::LeftTrigger),
            "LeftTrigger2" => Ok(Button::LeftTrigger2),
            "RightTrigger" => Ok(Button::RightTrigger),
            "RightTrigger2" => Ok(Button::RightTrigger2),
            "Select" => Ok(Button::Select),
            "Start" => Ok(Button::Start),
            "LeftThumb" => Ok(Button::LeftThumb),
            "RightThumb" => Ok(Button::RightThumb),
            "DPadUp" => Ok(Button::DPadUp),
            "DPadDown" => Ok(Button::DPadDown),
            "DPadLeft" => Ok(Button::DPadLeft),
            "DPadRight" => Ok(Button::DPadRight),
            _ => Err(()),
        }
    }
}

/// Controller information
#[derive(Debug, Clone)]
pub struct ControllerInfo {
    pub name: String,
    pub connected: bool,
    pub button_count: usize,
    pub axis_count: usize,
}

/// Input mapping presets for different platforms
pub struct InputPresets;

impl InputPresets {
    /// Get NES controller mapping
    pub fn nes_mapping() -> InputConfig {
        let mut config = InputConfig::default();
        
        // NES only has A, B, Start, Select, and D-pad
        config.keyboard_mapping.clear();
        config.keyboard_mapping.insert(VirtualKeyCode::Up, InputButton::Up);
        config.keyboard_mapping.insert(VirtualKeyCode::Down, InputButton::Down);
        config.keyboard_mapping.insert(VirtualKeyCode::Left, InputButton::Left);
        config.keyboard_mapping.insert(VirtualKeyCode::Right, InputButton::Right);
        config.keyboard_mapping.insert(VirtualKeyCode::Z, InputButton::A);
        config.keyboard_mapping.insert(VirtualKeyCode::X, InputButton::B);
        config.keyboard_mapping.insert(VirtualKeyCode::Return, InputButton::Start);
        config.keyboard_mapping.insert(VirtualKeyCode::Space, InputButton::Select);
        
        config
    }
    
    /// Get SNES controller mapping
    pub fn snes_mapping() -> InputConfig {
        let mut config = InputConfig::default();
        
        // SNES has A, B, X, Y, L, R, Start, Select, and D-pad
        config.keyboard_mapping.clear();
        config.keyboard_mapping.insert(VirtualKeyCode::Up, InputButton::Up);
        config.keyboard_mapping.insert(VirtualKeyCode::Down, InputButton::Down);
        config.keyboard_mapping.insert(VirtualKeyCode::Left, InputButton::Left);
        config.keyboard_mapping.insert(VirtualKeyCode::Right, InputButton::Right);
        config.keyboard_mapping.insert(VirtualKeyCode::Z, InputButton::A);
        config.keyboard_mapping.insert(VirtualKeyCode::X, InputButton::B);
        config.keyboard_mapping.insert(VirtualKeyCode::A, InputButton::X);
        config.keyboard_mapping.insert(VirtualKeyCode::S, InputButton::Y);
        config.keyboard_mapping.insert(VirtualKeyCode::Q, InputButton::L);
        config.keyboard_mapping.insert(VirtualKeyCode::W, InputButton::R);
        config.keyboard_mapping.insert(VirtualKeyCode::Return, InputButton::Start);
        config.keyboard_mapping.insert(VirtualKeyCode::Space, InputButton::Select);
        
        config
    }
    
    /// Get PlayStation controller mapping
    pub fn playstation_mapping() -> InputConfig {
        let mut config = InputConfig::default();
        
        // PlayStation has all buttons plus analog sticks
        config.keyboard_mapping.clear();
        config.keyboard_mapping.insert(VirtualKeyCode::Up, InputButton::Up);
        config.keyboard_mapping.insert(VirtualKeyCode::Down, InputButton::Down);
        config.keyboard_mapping.insert(VirtualKeyCode::Left, InputButton::Left);
        config.keyboard_mapping.insert(VirtualKeyCode::Right, InputButton::Right);
        config.keyboard_mapping.insert(VirtualKeyCode::Z, InputButton::A);
        config.keyboard_mapping.insert(VirtualKeyCode::X, InputButton::B);
        config.keyboard_mapping.insert(VirtualKeyCode::A, InputButton::X);
        config.keyboard_mapping.insert(VirtualKeyCode::S, InputButton::Y);
        config.keyboard_mapping.insert(VirtualKeyCode::Q, InputButton::L1);
        config.keyboard_mapping.insert(VirtualKeyCode::W, InputButton::R1);
        config.keyboard_mapping.insert(VirtualKeyCode::E, InputButton::L2);
        config.keyboard_mapping.insert(VirtualKeyCode::R, InputButton::R2);
        config.keyboard_mapping.insert(VirtualKeyCode::Return, InputButton::Start);
        config.keyboard_mapping.insert(VirtualKeyCode::Space, InputButton::Select);
        
        config
    }
}
