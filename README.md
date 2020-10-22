# <img src="https://i.imgur.com/kVCAlCt.png" height="29"> RogueCompany - SoftCheats 
Triggerbot for game "Rogue Company" that doesn't need an injection and is undetectable by the EAC

## Features
- Triggerbot *(2 variants)*
- Config with key definitions
- Simple text GUI
- Fast detection
- Bugs, much of them
 
## Config
**trigger_click**: Triggerbot will click the LMB <br>
**trigger_hold**: Triggerbot will hold the LMB

All the key codes you can find [here](https://docs.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes). 
```json
{
  "trigger_click": "0x12",
  "trigger_hold": "0xA0"
}
```
 
## Notes
I didn't include checking the active window to make the response much faster. <br>
The triggerbot will activate when the middle of the screen isn't white, and the button is hold. <br>
It won't be activated when the crosshair is red because I've read that there were some issues with it, 
but feel free to edit the code a little and test it. (Lines `148`, `149` and probably `172` with `180`).

## Credits
[**Thlordsw**](https://www.unknowncheats.me/forum/other-fps-games/421397-rogue-company-triggerbot.html) for the Python version.
 