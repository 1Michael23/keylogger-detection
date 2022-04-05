# Hardware Keylogger Detection

**Warning:** Certain Types of Hardware keyloggers can not be detected by this program, Passive Hardware Keyloggers are impossible to detect with software alone, as all they do is monitor the electrical signals between the keyboard and computer. Advanced Hardware keyloggers that copy usb identifiers (Such as the Hak5 key croc) Can not be currently detected with this program, although there are methods to detect these such as monitoring response times of usb commands. **This is planned but not currently implemented**.
**Note:** I do not own a hardware keylogger, this program is based on research papers and not real world research, (yet).

## How it works
This program logs and saves the key attributes of HID (human interface devices) upon the first time the program is run. all subsequent times the program is run, it checks these logs against the current HID devices and reports any discrepancies via a Discord webhook, along with an identifier unique to the computer.
This is not a bulletproof solution by any means, but its a hell of a lot better than checking the usb ports of a large deployment of computers.

### Webhook Example
![The Default discord webhook setup](https://github.com/1Michael23/keylogger-detection/blob/master/webhook-demo.png?raw=true "Discord webhook example")
 
## Todo
 
Short term
- Add Timing based detection method.
- Add Interrupt response based detection methods.
- Add obfuscation of log file.
 
Long term
- Add Wireless based detection method. (Most modern keyloggers are accessible over wifi/bluetooth)
- Block use of the keyboard if detection is confident enough.

## Dependencies
The crates used in this project.
```
Rusb
hidapi-rusb
serde
serde_json
chrono
mac_address (Placeholder unique ID is the computers MAC)
```

