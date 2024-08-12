use std::iter::zip;
use wifi_qr_code::{AuthenticationType, Visibility, WifiCredentials};
use std::io::{Write, Read, stdin, stdout};

fn input_string(prompt: &str, default: &str) -> String {
    let mut input = String::new();
    let stdin = stdin();
    let mut stdout = stdout();
    // while is used to loop until a non-empty string is entered. But if default is not empty, it will be used.
    while input.trim().is_empty() {
        print!("{}", prompt);
        stdout.flush().expect("Something horrible with your terminal happened.");
        stdin.read_line(&mut input).expect("Something horrible with your terminal happened. or eof was reached.");
        input = input.trim().to_string();
        if input.is_empty() && !default.is_empty() {
            input = default.to_string();
        } else if input.is_empty() {  // but default is also empty
            println!("\nThis field cannot be empty.");  // \n because the prompt does not end with a newline
        }
    }
    input

}

fn yes_or_no(prompt: &str, default: bool) -> bool {
    let mut stdin = stdin();
    let mut stdout=stdout();
    let mut result = default;

    // Read a single character without waiting for Enter
    print!("{}", prompt);
    stdout.flush().unwrap();
    let mut buf = [0; 1];
    match stdin.read(&mut buf) {
        Ok(n) if n > 0 => {
            let ch = buf[0] as char;
            result = match ch {
                'y' | 'Y' => true,
                'n' | 'N' => false,
                _ => result, // Ignore other characters
            };
        }
        _ => {} // Ignore timeout or read errors
    }

    // Print the result
    println!("{}", if result { "yes" } else { "no" });

    result
}

fn choose_authentication_type() -> AuthenticationType {
    let authentication_type = input_string("Enter the authentication type. (WPA by default): ", "WPA");
    match authentication_type.as_str() {
        "open" => AuthenticationType::NoPassword,
        "WEP" => AuthenticationType::WEP(input_string("Enter the WEP password: ", "")),
        "WPA" => AuthenticationType::WPA(input_string("Enter the WPA password: ", "")),
        _ => AuthenticationType::WPA(input_string("Enter the WPA password: ", "")),
    }
}

fn boolmatrix2string(matrix: Vec<Vec<bool>>) -> String {
    // this thing uses half block characters: U+2580 (▀) upper half block, U+2584 (▄) lower half block, U+2588 (█) full block
    // source: https://superuser.com/questions/1420001/is-it-possible-to-create-a-qr-code-using-text
    // very hope that the matrix has the same number of columns in each row. But if rows are even, lets add a row of zeros
    let mut result = String::new();
    let mut matrix = matrix.clone();
    if matrix.len() % 2 == 1 {
        matrix.push(vec![false; matrix[0].len()]);  // we believe you, the matrix decoder, that you produce a beautifully alligned matrix
    }
    // one text row is two matrix rows, so we iterate over the matrix in chunks of 2
    for chunk in matrix.chunks(2) {
        let mut iter = chunk.iter();
        if let (Some(first_row), Some(second_row)) = (iter.next(), iter.next()) {
            for (k, l) in zip(first_row.iter(), second_row.iter()) {
                // k and l are the nth elements of the first and second row of the chunk.
                match (k, l) {
                    (true, true) => result.push('█'), // full block
                    (true, false) => result.push('▀'), // upper half block
                    (false, true) => result.push('▄'), // lower half block
                    (false, false) => result.push(' '), // space
                }
            }  // end of for loop
            result.push('\n');
        }
    }

    result
}

fn main(){
    let ssid = input_string("Enter the SSID: ", "");
    let authentication_type = choose_authentication_type();
    let visibility = yes_or_no("Is the network visible? (yes by default)", true);
    let visibility = if visibility { Visibility::Visible } else { Visibility::Hidden };
    // Create a new WifiCredentials struct with owned data.
    let wifi_credentials = WifiCredentials {
        ssid: ssid.clone(), // Clone to avoid moving
        authentication_type,
        visibility,
    };

    let qr_code = wifi_qr_code::encode_as_matrix(&wifi_credentials, wifi_qr_code::QrCodeEcc::High).unwrap();
    let qr_code = boolmatrix2string(qr_code);
    println!("\n\n"); // it worked without this, but it looks better with it
    println!("{}", qr_code);

    // Construct the filename after getting the QR code
    let filename = format!("wifi_qr_code_{}.txt", ssid);  // Use ssid here

    // Create a file and write the QR code
    let mut file = std::fs::File::create(filename).unwrap();
    write!(file, "{}", qr_code).unwrap();
}