# 📚 OTP RS Dioxus Example

## 🛠️ Pre-requisites:

### 🐧 **Linux Users**

1. **Install [`rustup`](https://www.rust-lang.org/tools/install)**:

   ```sh
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

1. Install [`Dioxus CLI`](https://dioxuslabs.com/learn/0.7/getting_started):

   ```sh
   cargo install dioxus-cli
   ```

### 🪟 **Windows Users**

1. **Download and install `rustup`**: Follow the installation instructions [here](https://www.rust-lang.org/tools/install).

1. **Install Linux packages in WSL**:

   ```sh
   sudo apt update
   sudo apt install build-essential pkg-config libudev-dev
   ```

1. Install [`Dioxus CLI`](https://dioxuslabs.com/learn/0.7/getting_started):

   ```sh
   cargo install dioxus-cli
   ```

## 🚀 Building and Running

1. Fork/Clone the GitHub repository.

   ```sh
   git clone https://github.com/opensass/otprs
   ```

1. Navigate to the application directory.

   ```sh
   cd otprs/examples/dioxus
   ```

1. Run the client:

   ```sh
   dx serve --port 3000
   ```

Navigate to http://localhost:3000 to explore the landing page.
