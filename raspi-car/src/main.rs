use rust_gpiozero::output_devices::Motor;

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
                stop_motor(&mut motor1, &mut motor2);
                break;
            },
            _ => {
                stop_motor(&mut motor1, &mut motor2);
            },
        };
    }
}

pub fn stop_motor(m1 : &mut Motor, m2 : &mut Motor) {
    m1.stop();
    m2.stop();
}

pub fn get_first_byte() -> char {
    let mut input = String::new();
    let _ = std::io::stdin()
        .read_line(&mut input)
        .expect("Could not get stdin input.");
    input.chars().next().unwrap()
}
