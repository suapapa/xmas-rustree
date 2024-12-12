flash:
```sh
cargo flash --chip STM32F446RETx --connect-under-reset
```

run:
```sh
probe-rs run --chip STM32F446RET --connect-under-reset target/thumbv7em-none-eabi/release/twikle_x2_rs
```