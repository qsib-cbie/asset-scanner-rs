# Asset Scanner

This is a CLI utiltity for scanning and reporting connections to devices over BLE. The intended use case for this application is to scan for asset tags, connecting if necessary, and report the contact events to an asset tracking api.


### Output

With the asset tracking firmware running on a peripheral named QSIB0, the device listens and reads the 2 characteristics that the firmware persists through power cycles. It then disconnects and sets a pending connection after a period of time.

```
jwtrueb@dhcp-10-101-176-87 asset-scanner % RUST_LOG=asset_scanner=debug,debug cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.25s
     Running `target/debug/asset-scanner`
[2020-11-11T23:06:59Z INFO  asset_scanner] Using API: None
[2020-11-11T23:07:00Z INFO  asset_scanner::ble] DeviceConnected: 38:61:63:37:64:65
[2020-11-11T23:07:00Z INFO  asset_scanner::ble] Pending disconnection in 10s
[2020-11-11T23:07:00Z INFO  asset_scanner::ble] Pending connection in 20s
[2020-11-11T23:07:00Z INFO  asset_scanner::ble] Processing World for 97:87:A5:54:76:CC:4D:02:99:BB:AA:7D:5A:4F:4A:99
[2020-11-11T23:07:00Z INFO  asset_scanner::ble] Processing Hello for 85:6D:27:BF:B8:E1:41:09:BF:61:57:33:F4:E1:B2:99
[2020-11-11T23:07:00Z INFO  asset_scanner::ble] Processing World for 97:87:A5:54:76:CC:4D:02:99:BB:AA:7D:5A:4F:4A:99
[2020-11-11T23:07:00Z INFO  asset_scanner::ble] Processing Hello for 85:6D:27:BF:B8:E1:41:09:BF:61:57:33:F4:E1:B2:99
[2020-11-11T23:07:00Z INFO  asset_scanner::ble] Processing Hello for 85:6D:27:BF:B8:E1:41:09:BF:61:57:33:F4:E1:B2:99
[2020-11-11T23:07:00Z INFO  asset_scanner::ble] Processing World for 97:87:A5:54:76:CC:4D:02:99:BB:AA:7D:5A:4F:4A:99
[2020-11-11T23:07:02Z INFO  asset_scanner::ble] Mark event: {"Hello", "World"}
[2020-11-11T23:07:10Z INFO  asset_scanner::ble] Disconnecting Some("QSIB0")
[2020-11-11T23:07:10Z INFO  asset_scanner::ble] DeviceDisconnected: 38:61:63:37:64:65
[2020-11-11T23:07:20Z INFO  asset_scanner::ble] Attempting connecting Some("QSIB0")
[2020-11-11T23:07:20Z INFO  asset_scanner::ble] DeviceConnected: 38:61:63:37:64:65
[2020-11-11T23:07:20Z INFO  asset_scanner::ble] Pending disconnection in 10s
[2020-11-11T23:07:20Z INFO  asset_scanner::ble] Pending connection in 20s
[2020-11-11T23:07:21Z INFO  asset_scanner::ble] Processing Hello for 85:6D:27:BF:B8:E1:41:09:BF:61:57:33:F4:E1:B2:99
[2020-11-11T23:07:21Z INFO  asset_scanner::ble] Processing World for 97:87:A5:54:76:CC:4D:02:99:BB:AA:7D:5A:4F:4A:99
[2020-11-11T23:07:21Z INFO  asset_scanner::ble] Processing Hello for 85:6D:27:BF:B8:E1:41:09:BF:61:57:33:F4:E1:B2:99
[2020-11-11T23:07:21Z INFO  asset_scanner::ble] Processing World for 97:87:A5:54:76:CC:4D:02:99:BB:AA:7D:5A:4F:4A:99
[2020-11-11T23:07:21Z INFO  asset_scanner::ble] Processing Hello for 85:6D:27:BF:B8:E1:41:09:BF:61:57:33:F4:E1:B2:99
[2020-11-11T23:07:21Z INFO  asset_scanner::ble] Processing World for 97:87:A5:54:76:CC:4D:02:99:BB:AA:7D:5A:4F:4A:99
[2020-11-11T23:07:23Z INFO  asset_scanner::ble] Mark event: {"World", "Hello"}
[2020-11-11T23:07:30Z INFO  asset_scanner::ble] Disconnecting Some("QSIB0")
[2020-11-11T23:07:30Z INFO  asset_scanner::ble] DeviceDisconnected: 38:61:63:37:64:65
[2020-11-11T23:07:40Z INFO  asset_scanner::ble] Attempting connecting Some("QSIB0")
[2020-11-11T23:07:41Z INFO  asset_scanner::ble] DeviceConnected: 38:61:63:37:64:65
[2020-11-11T23:07:41Z INFO  asset_scanner::ble] Pending disconnection in 10s
[2020-11-11T23:07:41Z INFO  asset_scanner::ble] Pending connection in 20s
[2020-11-11T23:07:41Z INFO  asset_scanner::ble] Processing World for 97:87:A5:54:76:CC:4D:02:99:BB:AA:7D:5A:4F:4A:99
[2020-11-11T23:07:41Z INFO  asset_scanner::ble] Processing Hello for 85:6D:27:BF:B8:E1:41:09:BF:61:57:33:F4:E1:B2:99
[2020-11-11T23:07:41Z INFO  asset_scanner::ble] Processing Hello for 85:6D:27:BF:B8:E1:41:09:BF:61:57:33:F4:E1:B2:99
```