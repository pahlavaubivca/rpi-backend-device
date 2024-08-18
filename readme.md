## Build binary for Raspberry Pi

` cargo build --target aarch64-unknown-linux-gnu --release`

### Setup UART

1) remove `console=serial0,115200` from `/boot/firmware/cmdline.txt`
2) add `dtparam=uart=on` to `/boot/firmware/config.txt`
3) remove `enable_uart` from `/boot/firmware/config.txt`
3) add `dtoverlay=disable-bt` to `/boot/firmware/config.txt` (this step need verification)
4) stop bluetooth service `sudo systemctl disable hciuart` (need verification)

## FAQ

### raspberry pi Error: Io(Os { code: 13, kind: PermissionDenied, message: "Permission denied" })

```bash
sudo usermod -a -G dialout <USER>
```

