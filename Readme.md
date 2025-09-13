# Rust embedded examples
These are simple examples of Rust embedded code.

## Requirements Nucleo
Install target environment  
`rustup target add thumbv7em-none-eabihf`  

Install probe-rs  
`curl --proto '=https' --tlsv1.2 -LsSf https://github.com/probe-rs/probe-rs/releases/latest/download/probe-rs-tools-installer.sh | sh`  

## Nucleo stm32f446re examples

### Blinking ld2
Build project  
`cargo build`  

Flash Nucleo  
`cargo flash --chip stm32F446RE`  

### Usart2 send back using usb
Install screen  
`sudo apt install screen`  

Build project  
`cargo build`  

Flash Nucleo  
`cargo flash --chip stm32F446RE`  

Run screen  
`screen /dev/ttyACM0 9600`  


