# HackXpansion Speaker Module

**Cargo:** pkg:cargo/hackxpansion_speaker@0.1.0
A modular audio expansion for **HackXpansion**, designed to give the system high-quality digital audio playback through an external speaker.

The module takes digital audio from the HackXpansion controller, converts it to an analog signal, amplifies it, and drives a speaker — all through a compact plug-in expansion board.
## Images
![case](https://cdn.hackclub.com/01a0bbe8-d943-7aad-8db5-c783844745b8/image.png)
![pcb](https://cdn.hackclub.com/01a0a24d-f736-7c97-8032-d4e0bac616ac/image.png)

## Schematic
![schematic](https://cdn.hackclub.com/01a0a243-7778-77a2-8318-0d572c64835c/image.png)
## Overview

The Speaker Module provides the HackXpansion system with a dedicated audio output.

```text
                    HackXpansion Controller
                              │
                              │
                    ┌─────────┴─────────┐
                    │    Digital Audio  │
                    │       I²S         │
                    └─────────┬─────────┘
                              │
                              ▼
                    ┌───────────────────┐
                    │  TLV320DAC3100    │
                    │    Audio DAC      │
                    └─────────┬─────────┘
                              │
                              │ Analog Audio
                              ▼
                    ┌───────────────────┐
                    │      PAM8320      │
                    │  Class-D Amplifier│
                    └─────────┬─────────┘
                              │
                              ▼
                         ┌─────────┐
                         │ Speaker │
                         └─────────┘
```

The controller sends digital audio to the module over I²S. The TLV320DAC3100 converts the digital signal into analog audio, which is then amplified by the PAM8320 before being sent to the speaker.

## Features

* Digital audio playback through I²S
* 48 kHz, 16-bit audio
* Dedicated stereo audio DAC
* Class-D speaker amplification
* Two user-input buttons
* Hardware-controlled DAC reset
* I²C configuration of the audio DAC
* Designed for the HackXpansion modular ecosystem
* Asynchronous embedded Rust architecture

## Hardware

The module is built around three main stages:

### 1. Digital Audio

The HackXpansion controller provides the digital audio stream using I²S.

The module uses:

| Signal | Function              |
| ------ | --------------------- |
| MCLK   | Master clock          |
| BCLK   | I²S bit clock         |
| DIN    | Digital audio data    |
| LRCLK  | Left/right word clock |

The controller also communicates with the DAC over I²C to configure the audio system.

### 2. Digital-to-Analog Conversion

The **TLV320DAC3100** converts the incoming digital I²S stream into an analog audio signal.

It is configured for:

```text
Sample Rate: 48 kHz
Bit Depth:   16-bit
Interface:   I²S
MCLK:        12.288 MHz
```

The DAC is controlled over I²C and has a dedicated hardware reset line.

### 3. Speaker Amplification

The analog output from the DAC is passed to a **PAM8320 Class-D amplifier**.

The amplifier provides the additional power required to drive the external speaker.

This separates the digital audio processing from the power amplification stage:

```text
Digital Audio
     │
     ▼
    DAC
     │
     │ Low-power analog signal
     ▼
 Amplifier
     │
     │ High-power analog signal
     ▼
  Speaker
```

## Module Interface

The module uses the HackXpansion GPIO bank for both audio and user input.

| GPIO  | Function  |
| ----- | --------- |
| GPIO0 | I²C SCL   |
| GPIO1 | I²C SDA   |
| GPIO2 | I²S BCLK  |
| GPIO3 | I²S DIN   |
| GPIO4 | I²S LRCLK |
| GPIO5 | MCLK      |
| GPIO6 | DAC RESET |
| GPIO7 | Button 1  |
| GPIO8 | Button 2  |
| GPIO9 | Unused    |

The two buttons are exposed through the HackXpansion button interface:

```text
GPIO7 → Button A
GPIO8 → Button B
```

This allows the main HackXpansion firmware to assign functionality to the buttons without the speaker module needing to know what those actions are.

## Firmware

The module firmware is written in Rust using the HackXpansion driver architecture.

The driver is responsible for:

1. Resetting the audio DAC
2. Configuring the DAC over I²C
3. Starting the master clock
4. Starting the I²S output
5. Registering the audio interface with HackXpansion
6. Registering the module buttons


## HackXpansion Integration

The Speaker Module is designed to be one of the interchangeable expansion modules in the HackXpansion ecosystem.

Rather than requiring the main controller to contain dedicated speaker hardware, audio functionality can be provided by attaching this module.

The module is identified through HackXpansion's module detection system and initializes its own required peripherals when detected.

This keeps the audio hardware modular and allows the same controller architecture to support different combinations of expansion modules.

## Project Structure

```text
speaker-module/
├── Cargo.toml
├── README.md
└── src/
    ├── lib.rs
    └── dac.rs
```

`lib.rs` contains the HackXpansion-specific module driver, while `dac.rs` contains the reusable TLV320DAC3100 driver.

## License

MIT License

## Author

**Arghya Vyas**

Built as part of the **HackXpansion** project.
