# hackxpansion Speaker Module

**Crates:** pkg:cargo/hackxpansion_speaker@0.1.0
<br/>
A speaker module for hackxpansion that allows sound to be emitted from the device

The module takes digital audio from the Hackxpansion controller, converts it to an analog signal, amplifies it, and drives a speaker
## Images
![case](https://cdn.hackclub.com/01a0bbe8-d943-7aad-8db5-c783844745b8/image.png)
![pcb](https://cdn.hackclub.com/01a0a24d-f736-7c97-8032-d4e0bac616ac/image.png)

## Schematic
![schematic](https://cdn.hackclub.com/01a0a243-7778-77a2-8318-0d572c64835c/image.png)
## Overview

The Speaker Module provides the HackXpansion system with a dedicated audio output.

```text
                    Hackxpansion Controller
                              │
                              │
                    ┌─────────┴─────────┐
                    │    Digital Audio  │
                    │                   │
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
* 48 kHz, 16-bit audio
* Two user-input buttons

## Hardware

The **TLV320DAC3100** converts the incoming digital I²S stream into an analog audio signal.

It is configured for:

```text
Sample Rate: 48 kHz
Bit Depth:   16-bit
Interface:   I²S
MCLK:        12.288 MHz
```

The DAC is controlled over I²C and has a dedicated hardware reset line.

The analog output from the DAC is passed to a **PAM8320 Class-D amplifier**.


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

See the [Xpanse API docs](https://docs.rs/xpanse-api/latest/xpanse_api/index.html)

This allows the main HackXpansion firmware to assign functionality to the buttons without the speaker module needing to know what those actions are.

The driver is responsible for:

1. Resetting the audio DAC
2. Configuring the DAC over I²C
3. Starting the master clock
4. Starting the I²S output
5. Registering the audio interface with HackXpansion
6. Registering the module buttons


`lib.rs` contains the HackXpansion-specific module driver, while `dac.rs` contains the reusable TLV320DAC3100 driver.

---

This was all possible thanks to [Hackspansion: A hackclub YSWS](http://hackxpansion.hackclub.com/)

