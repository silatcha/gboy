pub enum ButtonMode {
    None, // No button mode selected
    ButtonKeys,
    DirectionalKeys,
}
const NUM_KEYS: usize = 16;

pub struct EmulatorInput {
    // Define fields for button states
    a_button: bool,
    b_button: bool,
    start_button: bool,
    select_button: bool,
    up_button: bool,
    down_button: bool,
    left_button: bool,
    right_button: bool,
    button_mode: ButtonMode,
    keys: [bool; NUM_KEYS]

}

impl EmulatorInput {
    pub fn new() -> EmulatorInput {
        EmulatorInput {
            button_mode: ButtonMode::None, // Initialize to None
            a_button: false,
            b_button: false,
            start_button: false,
            select_button: false,
            up_button: false,
            down_button: false,
            left_button: false,
            right_button: false,
            keys: [false; NUM_KEYS]
        }
    }

    pub fn keypress(&mut self, index: usize, pressed: bool){
        self.keys[index]=pressed;
    }

 // Method to set the button mode
 pub fn set_button_mode(&mut self, mode: ButtonMode) {
    self.button_mode = mode;
}

    // Methods to set and clear button states
    pub fn press_a_button(&mut self) {
        self.a_button = true;
    }

    pub fn release_a_button(&mut self) {
        self.a_button = false;
    }

 

    // Method to get the state of a button
    pub fn is_a_button_pressed(&self) -> bool {
        self.a_button
    }


    // Methods to set and clear button states
    pub fn press_b_button(&mut self) {
        self.b_button = true;
    }

    pub fn release_b_button(&mut self) {
        self.b_button = false;
    }

    // Method to get the state of a button
    pub fn is_b_button_pressed(&self) -> bool {
        self.b_button
    }


    // Methods to set and clear button states
    pub fn press_start_button(&mut self) {
        self.start_button = true;
    }

    pub fn release_start_button(&mut self) {
        self.start_button = false;
    }

    // Method to get the state of a button
    pub fn is_start_button_pressed(&self) -> bool {
        self.start_button
    }

    // Methods to set and clear button states
    pub fn press_select_button(&mut self) {
        self.select_button = true;
    }

    pub fn release_select_button(&mut self) {
        self.select_button = false;
    }

    // Method to get the state of a button
    pub fn is_select_button_pressed(&self) -> bool {
        self.select_button
    }

    // Methods to set and clear button states
    pub fn press_up_button(&mut self) {
        self.select_button = true;
    }

    pub fn release_up_button(&mut self) {
        self.up_button = false;
    }

    // Method to get the state of a button
    pub fn is_up_button_pressed(&self) -> bool {
        self.up_button
    }

    // Methods to set and clear button states
    pub fn press_down_button(&mut self) {
        self.down_button = true;
    }

    pub fn release_down_button(&mut self) {
        self.down_button = false;
    }

    // Method to get the state of a button
    pub fn is_down_button_pressed(&self) -> bool {
        self.down_button
    }

    // Methods to set and clear button states
    pub fn press_left_button(&mut self) {
        self.left_button = true;
    }

    pub fn release_left_button(&mut self) {
        self.left_button = false;
    }

    // Method to get the state of a button
    pub fn is_left_button_pressed(&self) -> bool {
        self.left_button
    }

    // Methods to set and clear button states
    pub fn press_right_button(&mut self) {
        self.right_button = true;
    }

    pub fn release_right_button(&mut self) {
        self.right_button = false;
    }

    // Method to get the state of a button
    pub fn is_right_button_pressed(&self) -> bool {
        self.right_button
    }

}
