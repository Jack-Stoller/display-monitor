# Display Monitor

Switches monitors source based on when a USB device connects and disconnects. Only needs to be ran on one device.

For Windows only. This exe will run without a terminal window. Use task manager to stop it if needed.

## Setup

### 1) Download

Download and extract the latest build exe from [releases](https://github.com/Jack-Stoller/display-monitor/releases)

### 2) Move to AppData

Move the file to `%appdata%/display-monitor/display-monitor.exe`

### 3) Create `config.yaml`

Create a `config.yaml` file next to the exe.

Paste in the following config

```
usb_id: <Vendor ID>:<Product ID>

on_connect: [<Display>]
on_disconnect: [<Display>]
```

`Vendor ID` and `Product ID` can be gotten from the Device Manager. Go into the Details tab for the USB device you want to listen to and Select `Hardware Ids`. 
Red is the Vendor ID and Blue is the Product ID. The `usb_id` for this keyboard would be `413c:2112`.

![image](https://user-images.githubusercontent.com/47873835/182264677-342191e0-fb34-452a-9505-7449251ca629.png)


`Display` is the value to set the Display input to. These normally start at 15 for the the first input and progress from there. This value is an array so multiple monitors can be switched at once.

### 4) Configure Task

I found the easiest way to run this is just as a task using Task Scheduler. Create a new task that runs only when the user is logged in, triggers at system startup, and starts the exe at `%appdata%/display-monitor/display-monitor.exe`.
Make sure the task is started after its created.
