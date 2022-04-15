use rust_gpiozero::output_devices::Motor;

fn main() {
    let mut motor1 = Motor::new(14, 15);
    let mut motor2 = Motor::new(23, 24);
    loop {
        let input_char = get_first_byte();
        match input_char {
            '6' => {
                motor1.forward();
                motor2.stop();
            },
            '4' => {
                motor2.forward();
                motor1.stop();
            },
            '8' => {
                motor1.forward();
                motor2.forward();
            },
            '5' => {
                motor1.backward();
                motor2.backward();
            },
            'b' => {
                stop_motor(&mut motor1, &mut motor2);
                break;
            },
            'h' => {
                stop_motor(&mut motor1, &mut motor2);
                print_help();   
            },
            _ => stop_motor(&mut motor1, &mut motor2)
        };
    }
}

fn stop_motor(m1 : &mut Motor, m2 : &mut Motor) {
    m1.stop();
    m2.stop();
}

fn print_help() {
    println!("+=============== RASPIAN MINI-CAR ======================+");
    println!("| motor1 => gpio 14 & 15                                |");
    println!("| motor2 => gpio 23 & 24                                |");
    println!("| corresponding pins for motor1 should be switched,     |");
    println!("| because one of the motors is in a reverse position.   |");
    println!("+=============== HELP ==================================+");
    println!("| Input keys                                            |");
    println!("| 8 => go forward                                       |");
    println!("| 5 => go backward                                      |");
    println!("| 6 => go to the right                                  |");
    println!("| 4 => go to the left                                   |");
    println!("| b => stops the car and quit the program               |");
    println!("| h => prints helps instructions                        |");
    println!("| everything else than the above keys will stop the car |");
    println!("+=======================================================+");
}

fn get_first_byte() -> char {
    let mut input = String::new();
    let _ = std::io::stdin()
        .read_line(&mut input)
        .expect("Could not get stdin input.");
    input.chars().next().unwrap()
}
