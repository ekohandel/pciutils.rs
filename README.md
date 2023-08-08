pciutils.rs
-----------
This is a very much work in progress clone of [pciutils](https://git.kernel.org/pub/scm/utils/pciutils/pciutils.git). The goal is to eventually match full functionality of pciutils library and the associated `lspci` and `setpci` command line utilities.

[![Build](https://github.com/ekohandel/pciutils.rs/actions/workflows/build.yml/badge.svg)](https://github.com/ekohandel/pciutils.rs/actions/workflows/build.yml)

### Status
Currently only minimal functionality is supported. `lspci` command line utility exists and is able to discover capabilities.

```bash
$ sudo -E bash -c 'cargo run --bin lspci -- -vs2e:00.0'
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/lspci '-vs2e:00.0'`
2e:00.0 Non-Volatile memory controller: Solid State Storage Technology Corporation Device 3500 (rev 01)
	Subsystem: Solid State Storage Technology Corporation Device 1092
	Memory at 80600000 (64-bit, non-prefetchable)
	Capabilities: [40] Power Management version 3
	Capabilities: [50] Capability 0x5 at 0x50
	Capabilities: [70] Capability 0x10 at 0x70
	Capabilities: [b0] Capability 0x11 at 0xb0
	Capabilities: [100] Capability 0x1 at 0x100
	Capabilities: [148] Capability 0x3 at 0x148
	Capabilities: [158] Capability 0x4 at 0x158
	Capabilities: [168] Capability 0xe at 0x168
	Capabilities: [178] Capability 0x19 at 0x178
	Capabilities: [2b8] Capability 0x18 at 0x2b8
	Capabilities: [2c0] Capability 0x1e at 0x2c0
	Kernel driver in use: nvme
	Kernel modules: nvme
```
