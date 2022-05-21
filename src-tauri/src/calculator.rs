use tauri::command;

pub fn sumar(a: i32, b: i32) -> i32 {
    return a + b;
}

pub fn sustraction(a: i32, b: i32) -> i32 {
    return a - b;
}

pub fn division(a: i32, b: i32) -> i32 {
    return a / b;
}

pub fn multiplication(a: i32, b: i32) -> i32 {
    return a * b;
}

pub fn porcentaje(a: i32, b: i32) -> i32 {
    return (a * b) / 100;
}

pub fn change_sign(a: i32) -> i32 {
    return -a;
}

pub fn start_operations(value_input: String, value: String) -> String {
    let symbols = ["+", "-", "/", "*", "%"];
    if value_input.len() <= 0 {
        return "".to_string();
    }
    for i in 0..symbols.len() {
        if value_input.contains(&symbols[i]) {
            let symbol = symbols[i];
            let mut numbers = value_input.split(&symbol);
            //  get vlue one
            let mut bool = false;

            let value_one = format!("{}", numbers.next().unwrap());

            if value_one.len() <= 0 {
                return format!("{}{}", value_input, value);
            }
            let value_two = format!("{}", numbers.next().unwrap());

            for j in 0..symbols.len() {
                if value.contains(&symbols[j]) {
                    bool = true;
                }
            }

            if bool {
                return value_input.replace(symbols[i], &value);
            }

            let a = value_one.parse::<i32>().unwrap();
            // get value two
            let b = value_two.parse::<i32>().unwrap();
            let result = match symbol.as_ref() {
                "+" => sumar(a, b).to_string(),
                "-" => sustraction(a, b).to_string(),
                "/" => division(a, b).to_string(),
                "*" => multiplication(a, b).to_string(),
                "%" => porcentaje(a, b).to_string(),
                _ => "".to_string(),
            }
            .to_string();

            if value != "Enter" {
                return format!("{}{}", result, value);
            }

            if value == "Shift" {
                return format!("{}", value_input);
            }

            if value == "Escape" {
                return "".to_string();
            }

            if value == "Enter" {
                return format!("{}", result);
            }

            return result;
        }
    }

    if value == "Enter" {
        return format!("{}", value_input);
    }

    if value == "Shift" {
        return format!("{}", value_input);
    }

    if value == "Backspace" {
        if value_input.len() > 0 {
            let slice = &value_input[..&value_input.len() - 1];
            return format!("{}", slice);
        }
        return "".to_string();
    }

    if value == "Delete" {
        return "".to_string();
    }
    if value == "Escape" {
        return "".to_string();
    }

    for i in 0..symbols.len() {
        if value.contains(&symbols[i].to_string()) {
            if value == "Shift" {
                return format!("{}", value_input);
            }

            if value == "+/-" {
                let mut bool = false;
                for i in 0..symbols.len() {
                    if value_input.contains(&symbols[i]) {
                        bool = true;
                    }
                }
                if !bool {
                    return format!("{}", change_sign(value_input.parse::<i32>().unwrap()));
                }
            }
            return format!("{}{}", value_input, value);
        }
    }

    return value_input;

    return format!("{}{}", value_input, value);
}

#[command]
pub fn select_operation(value: String, value_input: String) -> String {
    let numeros = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

    for i in 0..numeros.len() {
        if value.contains(&numeros[i].to_string()) {
            return format!("{}{}", value_input, value);
        }
    }

    return start_operations(value_input, value).to_string();
}
