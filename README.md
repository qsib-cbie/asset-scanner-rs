# Asset Scanner

This is a CLI utiltity for scanning and reporting connections to devices over BLE. The intended use case for this application is to scan for asset tags, connecting if necessary, and report the contact events to an asset tracking api.


### Output


```
jwtrueb@dhcp-10-101-176-87 asset-scanner % RUST_LOG=asset_scanner=debug,debug cargo run --  https://localhost:6001
    Finished dev [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/asset-scanner 'https://localhost:6001'`
[2020-11-10T00:21:13Z INFO  asset_scanner] Using API: https://localhost:6001
[2020-11-10T00:21:13Z INFO  btleplug::corebluetooth::adapter] Waiting on adapter connect
[2020-11-10T00:21:15Z INFO  btleplug::corebluetooth::adapter] Waiting on adapter connected
[2020-11-10T00:21:15Z INFO  btleplug::corebluetooth::adapter] Starting CoreBluetooth Scan
[2020-11-10T00:21:15Z INFO  btleplug::corebluetooth::internal] Adapter message!
[2020-11-10T00:21:17Z DEBUG asset_scanner::ble] Found QSIB0 with PeripheralProperties { address: 38:61:63:37:64:65, address_type: Random, local_name: Some("QSIB0"), tx_power_level: None, manufacturer_data: None, discovery_count: 1, has_scan_response: true }
^C
jwtrueb@dhcp-10-101-176-87 asset-scanner % RUST_LOG=asset_scanner=debug,debug cargo run --  https://localhost:6001
   Compiling asset-scanner v0.1.0 (/Users/jwtrueb/Desktop/workspace/asset-tracking/asset-scanner)
    Finished dev [unoptimized + debuginfo] target(s) in 3.51s
     Running `target/debug/asset-scanner 'https://localhost:6001'`
[2020-11-10T00:27:09Z INFO  asset_scanner] Using API: https://localhost:6001
[2020-11-10T00:27:09Z INFO  btleplug::corebluetooth::adapter] Waiting on adapter connect
[2020-11-10T00:27:09Z INFO  btleplug::corebluetooth::adapter] Waiting on adapter connected
[2020-11-10T00:27:09Z INFO  btleplug::corebluetooth::adapter] Starting CoreBluetooth Scan
[2020-11-10T00:27:09Z INFO  btleplug::corebluetooth::internal] Adapter message!
[2020-11-10T00:27:10Z DEBUG asset_scanner::ble] Found QSIB0 with PeripheralProperties { address: 38:61:63:37:64:65, address_type: Random, local_name: Some("QSIB0"), tx_power_level: None, manufacturer_data: None, discovery_count: 1, has_scan_response: true }
[2020-11-10T00:27:10Z INFO  btleplug::corebluetooth::peripheral] Trying device connect!
[2020-11-10T00:27:10Z DEBUG btleplug::corebluetooth::future] Waker set.
[2020-11-10T00:27:10Z INFO  btleplug::corebluetooth::internal] Adapter message!
[2020-11-10T00:27:10Z INFO  btleplug::corebluetooth::internal] got connectdevice msg!
[2020-11-10T00:27:10Z INFO  btleplug::corebluetooth::internal] Trying to connect peripheral!
[2020-11-10T00:27:10Z INFO  btleplug::corebluetooth::internal] Connecting peripheral!
[2020-11-10T00:27:10Z INFO  btleplug::corebluetooth::internal] Found services!
[2020-11-10T00:27:10Z INFO  btleplug::corebluetooth::internal] 0000180f-0000-1000-8000-00805f9b34fb
[2020-11-10T00:27:10Z INFO  btleplug::corebluetooth::internal] 8d53dc1d-1db7-4cd3-868b-8a527460aa84
[2020-11-10T00:27:10Z INFO  btleplug::corebluetooth::internal] 96f062c4-b99e-4141-9439-c4f9db977899
[2020-11-10T00:27:10Z INFO  btleplug::corebluetooth::internal] Found chars!
[2020-11-10T00:27:10Z INFO  btleplug::corebluetooth::internal] 08aefe30-27c5-4d34-9936-d4cc6188ee99
[2020-11-10T00:27:10Z INFO  btleplug::corebluetooth::internal] 4cc69818-27c5-4d34-9936-d4cc6188ee99
[2020-11-10T00:27:10Z INFO  btleplug::corebluetooth::internal] 9787a554-76cc-4d02-99bb-aa7d5a4f4a99
[2020-11-10T00:27:10Z INFO  btleplug::corebluetooth::internal] 856d27bf-b8e1-4109-bf61-5733f4e1b299
[2020-11-10T00:27:10Z INFO  btleplug::corebluetooth::internal] Flags: READ
[2020-11-10T00:27:10Z INFO  btleplug::corebluetooth::internal] Flags: READ
[2020-11-10T00:27:10Z INFO  btleplug::corebluetooth::internal] Flags: READ | WRITE
[2020-11-10T00:27:10Z INFO  btleplug::corebluetooth::internal] Flags: READ | WRITE
[2020-11-10T00:27:10Z INFO  btleplug::corebluetooth::internal] Found chars!
[2020-11-10T00:27:10Z INFO  btleplug::corebluetooth::internal] 00002a19-0000-1000-8000-00805f9b34fb
[2020-11-10T00:27:10Z INFO  btleplug::corebluetooth::internal] Flags: READ | NOTIFY
[2020-11-10T00:27:11Z INFO  btleplug::corebluetooth::internal] Found chars!
[2020-11-10T00:27:11Z INFO  btleplug::corebluetooth::internal] da2e7828-fbce-4e01-ae9e-261174997c48
[2020-11-10T00:27:11Z INFO  btleplug::corebluetooth::internal] Flags: WRITE_WITHOUT_RESPONSE | NOTIFY
[2020-11-10T00:27:11Z INFO  btleplug::corebluetooth::internal] 00:00:2A:19:00:00:10:00:80:00:00:80:5F:9B:34:FB
[2020-11-10T00:27:11Z INFO  btleplug::corebluetooth::internal] DA:2E:78:28:FB:CE:4E:01:AE:9E:26:11:74:99:7C:48
[2020-11-10T00:27:11Z INFO  btleplug::corebluetooth::internal] 08:AE:FE:30:27:C5:4D:34:99:36:D4:CC:61:88:EE:99
[2020-11-10T00:27:11Z INFO  btleplug::corebluetooth::internal] 97:87:A5:54:76:CC:4D:02:99:BB:AA:7D:5A:4F:4A:99
[2020-11-10T00:27:11Z INFO  btleplug::corebluetooth::internal] 4C:C6:98:18:27:C5:4D:34:99:36:D4:CC:61:88:EE:99
[2020-11-10T00:27:11Z INFO  btleplug::corebluetooth::internal] 85:6D:27:BF:B8:E1:41:09:BF:61:57:33:F4:E1:B2:99
[2020-11-10T00:27:11Z DEBUG btleplug::corebluetooth::peripheral] emitted DeviceConnected(38:61:63:37:64:65)
[2020-11-10T00:27:11Z INFO  btleplug::corebluetooth::peripheral] Device connected!
[2020-11-10T00:27:11Z INFO  asset_scanner::ble] DeviceConnected: 38:61:63:37:64:65
[2020-11-10T00:27:11Z DEBUG asset_scanner::ble] Connected Some("QSIB0") with PeripheralProperties { address: 38:61:63:37:64:65, address_type: Random, local_name: Some("QSIB0"), tx_power_level: None, manufacturer_data: None, discovery_count: 1, has_scan_response: true }
[2020-11-10T00:27:11Z DEBUG asset_scanner::ble] Discovered Characteristic: Characteristic {
        start_handle: 0,
        end_handle: 0,
        value_handle: 0,
        uuid: DA:2E:78:28:FB:CE:4E:01:AE:9E:26:11:74:99:7C:48,
        properties: WRITE_WITHOUT_RESPONSE | NOTIFY,
    }
[2020-11-10T00:27:11Z DEBUG asset_scanner::ble] Discovered Characteristic: Characteristic {
        start_handle: 0,
        end_handle: 0,
        value_handle: 0,
        uuid: 97:87:A5:54:76:CC:4D:02:99:BB:AA:7D:5A:4F:4A:99,
        properties: READ | WRITE,
    }
[2020-11-10T00:27:11Z DEBUG asset_scanner::ble] Discovered Characteristic: Characteristic {
        start_handle: 0,
        end_handle: 0,
        value_handle: 0,
        uuid: 85:6D:27:BF:B8:E1:41:09:BF:61:57:33:F4:E1:B2:99,
        properties: READ | WRITE,
    }
[2020-11-10T00:27:11Z DEBUG asset_scanner::ble] Discovered Characteristic: Characteristic {
        start_handle: 0,
        end_handle: 0,
        value_handle: 0,
        uuid: 4C:C6:98:18:27:C5:4D:34:99:36:D4:CC:61:88:EE:99,
        properties: READ,
    }
[2020-11-10T00:27:11Z DEBUG asset_scanner::ble] Discovered Characteristic: Characteristic {
        start_handle: 0,
        end_handle: 0,
        value_handle: 0,
        uuid: 08:AE:FE:30:27:C5:4D:34:99:36:D4:CC:61:88:EE:99,
        properties: READ,
    }
[2020-11-10T00:27:11Z DEBUG asset_scanner::ble] Discovered Characteristic: Characteristic {
        start_handle: 0,
        end_handle: 0,
        value_handle: 0,
        uuid: 00:00:2A:19:00:00:10:00:80:00:00:80:5F:9B:34:FB,
        properties: READ | NOTIFY,
    }
```