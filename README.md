Atari Portfolio address file reader
===================================
Little command line reader application for the Atari Portfolio address file
format. While intended to store addresses, it is a neat little key/value
database and can be used to store all kinds of information. Many Portfolio users
stored other things in it, for example command references, notes or task lists.

Usage
-----
The application mimics the hot keys and modes of the Address application on the
Atari Portfolio. It has two modes: Initially the list of all database entries is
presented. When an entry is selected via the cursor keys, pressing `[Enter]`
switches to the selected page. Hitting `[Escape]` returns you to the list. Press
it again to leave the application.

```text
             press [Enter]
┌─────────┐---------------->┌─────────┐
│line mode│                 │page mode│
└─────────┘<----------------└─────────┘
             press [Escape]
```

Keys in line mode:
- `[Cursor Up]` selects previous entry
- `[Cursor Down]` selects next entry
- `[Enter]` switches to page mode and displays selected entry
- `[Escape]` closes the application

Keys in page mode:
- `[Page Up]` selects previous entry
- `[Page Down]` selects next entry
- `[Escape]` switches to list mode

Build
-----
The application is written in [Rust 🦀](https://www.rust-lang.org/) and you need
to have `cargo` and `rustc` installed to build it. Example:

```shell
$ make help
Usage: make <target(s)>

Specify one or multiple of the following targets and they will be processed in the given order:
build           Build the binary for debug (default).
release         Build the binary for release.
lint            Run fmt & clippy on the code to come up with improvements.
help            Displays these usage instructions.
$ make release
cargo build --release
   Compiling libc v0.2.77
   Compiling memchr v2.3.3
   Compiling byteorder v1.3.4
   Compiling lazy_static v1.4.0
   Compiling cfg-if v0.1.10
   Compiling serde v1.0.116
   Compiling autocfg v1.0.1
   Compiling ryu v1.0.5
   Compiling log v0.4.11
   Compiling smallvec v1.4.2
   Compiling scopeguard v1.1.0
   Compiling itoa v0.4.6
   Compiling arc-swap v0.4.7
   Compiling bitflags v1.2.1
   Compiling cassowary v0.3.0
   Compiling unicode-segmentation v1.6.0
   Compiling unicode-width v0.1.8
   Compiling lock_api v0.3.4
   Compiling num-traits v0.2.12
   Compiling num-integer v0.1.43
   Compiling csv-core v0.1.10
   Compiling signal-hook-registry v1.2.1
   Compiling parking_lot_core v0.7.2
   Compiling mio v0.7.0
   Compiling time v0.1.44
   Compiling regex-automata v0.1.9
   Compiling parking_lot v0.10.2
   Compiling signal-hook v0.1.16
   Compiling crossterm v0.17.7
   Compiling chrono v0.4.15
   Compiling tui v0.10.0
   Compiling bstr v0.2.13
   Compiling csv v1.1.3
   Compiling codepage-437 v0.1.0
   Compiling pofo_adr v0.1.0 (/home/elrido/Projekte/pofo_adr)
    Finished release [optimized] target(s) in 26.47s
strip target/release/pofo_adr
$ target/release/pofo_adr examples/ez-ref.adr
╔ examples/ez-ref.adr ═══════════════════════════════════════════════════ #122 ╗
║ ABACUS BOOKS  (616)698-0330                                                  ║
║ AC Adapter [HPC-401]                                                         ║
║ Address Book                                                                 ║
║ ADR file                                                                     ║
║ application                                                                  ║
║ app (applications)                                                           ║
║ ARTISAN SOFTWARE (209)239-1552                                               ║
║ ascii (American Standard Code)                                               ║
║ ATARI COMPUTER CORPORATION (408)744-0880 [cust serv] (408)745-2191 [BBS] (408║
║ ATARI EXPLORER MAGAZINE  (218)723-9202 [subscribe]                           ║
║ AUTOBYTE  (514)637-6232 [main]  (514)637-1491 [fax]                          ║
║ autoexec.bat                                                                 ║
║ AWARE ELECTRONICS  (302)655-3800 [main/fax]                                  ║
║ backup                                                                       ║
║ bar code scanner                                                             ║
║ batteries                                                                    ║
║ bit                                                                          ║
║ bit-mapped graphics                                                          ║
║ BSE  (714)832-4316 [main] (714)832-5381 [fax]                                ║
║ byte                                                                         ║
║ cable                                                                        ║
║ Calculator                                                                   ║
╚ Sun 20 Sep 20 15:46 ═════════════════════════════════════════════════════════╝
```
