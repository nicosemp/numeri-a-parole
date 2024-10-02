# Numeri a parole (Numbers to words)

## Description

`numeri-a-parole` is a small program that converts numbers into their text form in Italian.

## Installation

To install the project, you can clone the repository and install the dependencies:

```bash
git clone https://github.com/yourusername/numeri-a-parole.git

cd numeri-a-parole
```

## Usage

```bash
cargo r 1
# Output: "uno"

cargo r 123
# Output: "centoventitre"

cargo r 1234567
# Output: "un milione duecentotrentaquattromilacinquecentosessantasette"
```

You can also build the release executable and run that

```bash
cargo b -r

./target/release/numeri-a-parole 42
# Output: "quarantadue"
```

## Contributing

Contributions are welcome! Please fork the repository and submit a pull request.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Contact

For any questions or suggestions, please open an issue.
