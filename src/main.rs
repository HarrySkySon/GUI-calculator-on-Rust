use gtk::{Application, ApplicationWindow, Box, Button, Grid, Label, Orientation};
use std::process;
gtk::init().unwrap();

fn main() {
    let app = Application::new(Some("com.example.calculator"), Default::default())
        .expect("Failed to initialize GTK application");

    app.connect_activate(|app| {
        // Create the main window
        let window = ApplicationWindow::new(app);
        window.set_title("Calculator");
        window.set_default_size(300, 300);

        // Create the grid to hold the buttons and display
        let grid = Grid::new();
        grid.set_column_homogeneous(true);
        grid.set_row_homogeneous(true);
        window.add(&grid);

        // Create the label to display the input and output
        let label = Label::new(None);
        grid.attach(&label, 0, 0, 4, 1);

        // Create the buttons for the numbers and operations
        let buttons = [
            "7", "8", "9", "/",
            "4", "5", "6", "*",
            "1", "2", "3", "-",
            "0", ".", "=", "+",
        ];
        for (i, button_label) in buttons.iter().enumerate() {
            let button = Button::new_with_label(button_label);
            let x = i % 4;
            let y = i / 4 + 1;
            grid.attach(&button, x, y, 1, 1);

            // Handle button clicks
            button.connect_clicked(clone!(@weak label => move |_| {
                let text = label.get_text().unwrap().as_str();
                let new_text = match button_label {
                    "=" => {
                        // Evaluate the expression
                        match eval_expression(text) {
                            Ok(result) => result.to_string(),
                            Err(msg) => msg,
                        }
                    },
                    _ => text.to_owned() + button_label,
                };
                label.set_text(&new_text);
            }));
        }

        // Show the window and start the GTK main loop
        window.show_all();
    });

    // Run the GTK main loop
    app.run(&[]);
}

// Evaluate the expression and return the result or an error message
fn eval_expression(expr: &str) -> Result<f64, String> {
    // Split the expression into numbers and operators
    let tokens: Vec<&str> = expr.split(|c| "+-*/".contains(c)).collect();
    let ops: Vec<&str> = expr.split(|c| "0123456789.".contains(c)).filter(|s| !s.is_empty()).collect();

    // Check that there is at least one operator and one number
    if ops.is_empty() || ops.len() >= tokens.len() {
        return Err("Invalid expression".to_owned());
    }

    // Evaluate the expression
    let mut result = tokens[0].parse::<f64>().map_err(|_| "Invalid number".to_owned())?;
    for i in 0..ops.len() {
        let num = tokens[i+1].parse::<f64>().map_err(|_| "Invalid number".to_owned())?;
        match ops[i] {
            "+" => result += num,
            "-" => result -= num,
            "*" => result *= num,
            "/" => result /= num,
            _ => return Err("Invalid operator".to_owned()),
        }
    }
    Ok(result)
}
