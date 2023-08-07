pciutils.rs
-----------
This is a very much work in progress clone of [pciutils](https://git.kernel.org/pub/scm/utils/pciutils/pciutils.git). The goal is to eventually match full functionality of pciutils library and the associated `lspci` and `setpci` command line utilities.

[![Build](https://github.com/ekohandel/pciutils.rs/actions/workflows/build.yml/badge.svg)](https://github.com/ekohandel/pciutils.rs/actions/workflows/build.yml)

### Status
Currently only minimal functionality is supported. `lspci` command line utility exists and is able to discover traditional capabilities.

```bash
$ sudo -E bash -c 'cargo run --bin lspci -- -vs04:00.0'
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `./lspci '-vs04:00.0'`
04:00.0 USB controller: Intel Corporation JHL7540 Thunderbolt 3 USB Controller [Titan Ridge DD 2018] (rev 06)
        Subsystem: Vendor 8086 Device 0000
        Memory at 82000000 (32-bit, non-prefetchable)
        Capabilities: [80] Power Management version 3
        Capabilities: [88] Capability 0x5 at 0x88
        Capabilities: [c0] Capability 0x10 at 0xc0
        Kernel driver in use: xhci_hcd
        Kernel modules: xhci_pci
```
