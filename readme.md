# Wifi_qr

a simple rust program to generate qr code for wifi connection

## Usage

- Clone and cd into the project directory.
- Run the following command to build the project.
    ```bash
    cargo run --release
    ```
- Follow the prompt to enter the wifi details.
- The qr code will be printed to the console and generated and saved in the project directory as a txt file.

## How it actually prints the qr code to the console

The utf-8 representation of the qr-code is inspired by an awesome unix CLI tool called QREncode.
After browsing thrue web searching how to print qr code to the console, i found [this thread on superuser](https://superuser.com/questions/1420001/is-it-possible-to-create-a-qr-code-using-text) which gave me the idea to use the utf-8 representation of the qr code to print it to the console.

### the algorithm

- first, we get the qr code as a 2d array (or technically a vector of vectors) of booleans.
- then we iterate by 2 lines of the boolean table at a time,  and inside we iterate by 1 column at a time, getting the upper and lower boolean values for each column.
- Then we check the 4 possible combinations of the upper and lower boolean values and print the corresponding utf-8 character. If both upper and lower are true, we print the full block. If the upper is true and the lower is false, we print the upper half block. If the upper is false and the lower is true, we print the lower half block. If both are false, we print the space character.
- after each 2 matrix lines, we print a new line character to separate the qr code lines.

Technically, this qrcode is scannable from the console, notepad and other text showing applications, and since the qrcode is 3 to 5 times bigger than the actual qrcode, it is more scannable from a distance.

## Dependencies

The only dependence is the wifi_qrcode crate which is a simple wrapper around the qrcode crate to generate qr code for wifi connection.
Before, itertools was used for zipping, but rust compiler quickly said that itertools's zip is deprecated and i should use the std::iter::zip instead.
The executable size is only 207 kb on windows with --release flag.

## Contributing

Feel free to contribute to this project by forking the repository and making a pull request.

#### happy exploring and experimenting with the code.
