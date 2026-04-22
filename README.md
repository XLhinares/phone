# Phone

(read `## Important info`)

## Description

This is a utility to connect Android devices through `adb` and access screen or camera with `scrcpy`.

## Use cases

- Connecting / disconnecting devices wirelessly with adb.
    - (I personally use it to run flutter projects without needing to plug my phone each time).
- Streaming your android device screen to your computer monitor.
- Using your android device cameras as your computer camera. 

## Important info

### Device

You need to ***enable wireless debugging*** in your device's developer settings!

### Camera

The `v4l2loopback` should be loaded before using the phone camera commands.
To do so, you can use: 
```
sudo modprobe v4l2loopback exclusive_caps=1 card_label="Phone Camera"
```

This is not part of this utility to avoid superuser commands.


#### Using /etc/modules

The v4l2loopback module can be loaded on boot via `/etc/modules` (or `systemd`) to avoid having to run the sudo command each time you want to use the camera command. This can be achieved with:

> ```bash
> echo 'v4l2loopback' >> '/etc/modules'
> echo 'options v4l2loopback exclusive_caps=1 card_label="Phone Camera"' > '/etc/modprobe.d/v4l2loopback.conf'
> ```

### IP limitations

If you use phone data and hotspot, then your devices might change IP frequently which causes this utility to lose track of them.  
In this case, you should simply reconnect them with 
```bash
phone new
```

## Building the project

The project can be built with:
```bash
cargo build --release
```
Additionally, there is an hidden argument to generate bash autocompletion.
```bash
phone bash-completions
```
