Change Log of Atari Portfolio address file reader
=================================================

Version 0.3.4 / 2026-02-07
--------------------------
- bumped chrono from 0.4.42 to 0.4.43

Version 0.3.3 / 2025-10-03
--------------------------
- bumped chrono from 0.4.41 to 0.4.42

Version 0.3.2 / 2025-05-02
--------------------------
- bumped chrono from 0.4.40 to 0.4.41

Version 0.3.1 / 2025-03-02
--------------------------
- bumped chrono from 0.4.39 to 0.4.40

Version 0.3.0 / 2025-01-03
--------------------------
- more closely mimic Portfolio by:
  - inset border titles by one
  - add a border between title and address in page mode
  - fix display of tabs to width of 8 characters
- display index of selected address when in page mode (deviating from Portfolio)

Version 0.2.9 / 2025-01-02
--------------------------
- bumped chrono from 0.4.38 to 0.4.39
- bumped ratatui from 0.27.0 to 0.29.0
- bumped crossterm from 0.27.0 to 0.28.1
- updated dependencies

Version 0.2.8 / 2024-07-06
--------------------------
- bumped ratatui from 0.26.3 to 0.27.0

Version 0.2.7 / 2024-06-02
--------------------------
- bumped ratatui from 0.26.2 to 0.26.3

Version 0.2.6 / 2024-04-16
--------------------------
- bumped chrono from 0.4.34 to 0.4.38
- bumped ratatui from 0.26.1 to 0.26.2

Version 0.2.5 / 2024-03-02
--------------------------
- bumped rust version to 1.74 for clap 4.5.1, used in rpm packaging

Version 0.2.4 / 2024-03-02
--------------------------
- bumped chrono from 0.4.33 to 0.4.34
- bumped ratatui from 0.26.0 to 0.26.1

Version 0.2.3 / 2024-02-03
--------------------------
- bumped rust version to 1.72 to gain access to stable feature stdsimd for
  aarch64 builds, like on MacOS

Version 0.2.2 / 2024-02-03
--------------------------
- switched from tui 0.19.0 (unmaintained) to ratatui 0.26.0 (maintained fork)
- bumped crossterm from 0.25.0 to 0.27.0

Version 0.2.1 / 2024-02-03
--------------------------
- bumped chrono from 0.4.31 to 0.4.33

Version 0.2.0 / 2023-12-25
--------------------------
- updated dependencies, increased Rust edition & version
- added this Change Log
- added GitHub actions for dependency updates, linting and release builds
- applied lints, corrected typo, internalized list mode state

Version 0.1.0 / 2020-09-20
--------------------------
- initial release, providing read-only functionality
