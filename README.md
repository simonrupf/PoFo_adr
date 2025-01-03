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

```text
$ make help
Usage: make <target(s)>

Specify one or multiple of the following targets and they will be processed in the given order:
build           Build the binary for debug (default).
release         Build the binary for release.
lint            Run fmt & clippy on the code to come up with improvements.
help            Displays these usage instructions.
$ make release
cargo build --release
   Compiling proc-macro2 v1.0.92
   Compiling autocfg v1.4.0
   Compiling libc v0.2.169
   Compiling unicode-ident v1.0.14
   Compiling rustversion v1.0.19
   Compiling serde v1.0.217
   Compiling signal-hook v0.3.17
   Compiling fnv v1.0.7
   Compiling ident_case v1.0.1
   Compiling memchr v2.7.4
   Compiling cfg-if v1.0.0
   Compiling strsim v0.11.1
   Compiling parking_lot_core v0.9.10
   Compiling scopeguard v1.2.0
   Compiling rustix v0.38.42
   Compiling lock_api v0.4.12
   Compiling smallvec v1.13.2
   Compiling csv-core v0.1.11
   Compiling ryu v1.0.18
   Compiling log v0.4.22
   Compiling quote v1.0.38
   Compiling itoa v1.0.14
   Compiling num-traits v0.2.19
   Compiling diff v0.1.13
   Compiling either v1.13.0
   Compiling syn v2.0.94
   Compiling signal-hook-registry v1.4.2
   Compiling mio v1.0.3
   Compiling allocator-api2 v0.2.21
   Compiling linux-raw-sys v0.4.14
   Compiling foldhash v0.1.4
   Compiling heck v0.5.0
   Compiling equivalent v1.0.1
   Compiling paste v1.0.15
   Compiling yansi v1.0.1
   Compiling bitflags v2.6.0
   Compiling hashbrown v0.15.2
   Compiling signal-hook-mio v0.2.4
   Compiling parking_lot v0.12.3
   Compiling itertools v0.13.0
   Compiling pretty_assertions v1.4.1
   Compiling castaway v0.2.3
   Compiling static_assertions v1.1.0
   Compiling unicode-segmentation v1.12.0
   Compiling indoc v2.0.5
   Compiling unicode-width v0.1.14
   Compiling compact_str v0.8.1
   Compiling crossterm v0.28.1
   Compiling lru v0.12.5
   Compiling cassowary v0.3.0
   Compiling unicode-width v0.2.0
   Compiling csv v1.3.1
   Compiling iana-time-zone v0.1.61
   Compiling unicode-truncate v1.1.0
   Compiling chrono v0.4.39
   Compiling codepage-437 v0.1.0
   Compiling darling_core v0.20.10
   Compiling strum_macros v0.26.4
   Compiling darling_macro v0.20.10
   Compiling darling v0.20.10
   Compiling instability v0.3.5
   Compiling strum v0.26.3
   Compiling ratatui v0.29.0
   Compiling pofo_adr v0.3.0 (Projects/PoFo_adr)
    Finished `release` profile [optimized] target(s) in 22.56s
$ target/release/pofo_adr examples/ez-ref.adr
╔═ examples/ez-ref.adr ═════════════════════════════════════════════════ #122 ═╗
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
╚═ Fri 03 Jan 25 08:09 ════════════════════════════════════════════════════════╝
# selecting "Address Book" page with the down cursor key and hitting Enter:
╔═ examples/ez-ref.adr ════════════════════════════════════════════ 3 of #122 ═╗
║ Address Book                                                                 ║
║──────────────────────────────────────────────────────────────────────────────║
║ A database application provided in the                                       ║
║ ROM of the Portfolio. Access is                                              ║
║ obtained by holding the Atari key and                                        ║
║ pressing the A key.                                                          ║
║ The autodialer will provide two                                              ║
║ options of each phone number if the                                          ║
║ entry is made on the top line as                                             ║
║ follows:                                                                     ║
║ Artisan Software  (209)239-1552 [main]                                       ║
║                                                                              ║
║                                                                              ║
║                                                                              ║
║                                                                              ║
║                                                                              ║
║                                                                              ║
║                                                                              ║
║                                                                              ║
║                                                                              ║
║                                                                              ║
║                                                                              ║
╚═ Fri 03 Jan 25 08:11 ════════════════════════════════════════════════════════╝
```
