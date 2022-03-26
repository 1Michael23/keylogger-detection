# Hardware Keylogger Detection

**Warning:** Certain Types of Hardware keyloggers can not be detected by this program, Passive Hardware Keyloggers are impossible to detect with software alone, as all they do is monitor the electrical signals between the keyboard and computer. Advanced Hardware keyloggers that copy usb identifiers (Such as the Hak5 key croc) Can not be currently detected with this program, although there are methods to detect these such as monitoring response times of usb commands. **This is planned but not currently implimented**.

**Note:** I do not own a hardware keylogger, this program is based on research papers and not real world research, (yet).

## How it works

This program logs and saves the key attributes of HID (human interface devices) upon the first time the program is run. all subsequent times the program is run, it checks these logs against the current HID devices and reports any descrepancies via a Discord webhook, along with an identifier unique to the computer.

This is not a bulletproof sollution by any means, but its a hell of a lot better than checking the usb ports of a large deployment of computers.

## Dependancies 

The crates used in this project.

```
Rusb
hidapi-rusb
serde
serde_json
chrono
mac_address (Placeholder unique ID is the computers MAC)
```
