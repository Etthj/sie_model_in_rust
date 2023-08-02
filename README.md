# Gravitational Lensing Simulation in Rust

[![Rust](https://img.shields.io/badge/Rust-1.55.0-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

This project implements a gravitational lensing simulation using a Singular Isothermal Ellipsoid (SIE) model in Rust. The SIE model is a simplified representation of gravitational lensing caused by a massive object. Given the SIE parameters and the position of a point source, the program calculates the positions of the lensed images.

## Prerequisites

- Rust (1.55.0 or higher): You can install Rust by following the instructions at [https://www.rust-lang.org/](https://www.rust-lang.org/).

## Installation and Usage

1. Clone the repository:

```bash
git clone https://github.com/Etthj/sie_model_in_rust.git
cd sie_model_in_rust
```

2. Run the simulation:

```bash
cargo run
```

The program will prompt you to enter the parameters of the SIE model and the position of the point source. It will then compute the positions of the lensed images.

## SIE Model

The SIE model is defined by three parameters:

- `einstein_radius`: The Einstein radius of the gravitational lens.
- `axis_ratio`: The ratio of the semi-minor axis to the semi-major axis of the lens.
- `orientation`: The orientation angle of the lens in radians.

The deflection angle at any position in the lens plane is calculated using these parameters.

## Contributing

Contributions to this project are welcome! If you find any issues or want to improve the code, feel free to open an issue or submit a pull request.

## License

This project is licensed under the MIT License.