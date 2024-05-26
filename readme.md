`cargo build --target aarch64-unknown-linux-gnu`

## FAQ 

### raspberry pi Error: Io(Os { code: 13, kind: PermissionDenied, message: "Permission denied" })

```bash
sudo usermod -a -G dialout <USER>
```