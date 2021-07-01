use rust_gpiozero::output_devices::PWMOutputDevice;

struct Motor(PWMOutputDevice, PWMOutputDevice);

impl Motor {
    pub fn new(first : u8, second : u8) -> Self {
        Self (PWMOutputDevice::new(first), PWMOutputDevice::new(second))
    }

    pub fn forward(&mut self) {
        self.0.on();
        self.1.off();
    }

    pub fn backward(&mut self) {
        self.0.off();
        self.1.on();
    }
    
    pub fn stop(&mut self) {
        self.0.off();
        self.0.off();
    }
}

fn main() {
    let mut motor1 = Motor::new(14, 15);
    let mut motor2 = Motor::new(23, 24);
    loop {
        let input_char = get_first_byte();
        match input_char {
            'a' => {
                motor1.backward();
                motor2.stop();
            },
            'e' => {
                motor1.stop();
                motor2.backward();
            },
            'd' => {
                motor1.forward();
                motor2.stop();
            },
            'q' => {
                motor2.forward();
                motor1.stop();
            },
            'z' => {
                motor1.forward();
                motor2.forward();
            },
            's' => {
                motor1.backward();
                motor2.backward();
            },
            'b' => {
                motor1.stop();
                motor2.stop();
                break;
            },
            _ => {
                motor1.stop();
                motor2.stop();
            },
        };
    }
}

pub fn get_first_byte() -> char {
    let mut input = String::new();
    let _ = std::io::stdin()
        .read_line(&mut input)
        .ok()
        .expect("Could not get stdin input.");
    input.chars().next().unwrap()
}
