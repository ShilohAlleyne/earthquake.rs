# Earthquake.rs

`earthquake.rs` is a simple tui dashboard of the provided client data. Given that this was framed as a technical assessment, and not a insurance test, the analytics in this program are rudamenatry, instead the focus is on presenting the client data so that they can gain a clear understanding of their portfolio. It is written entirely in Rust.

![dashboard](https://github.com/user-attachments/assets/609c175b-2f3d-46e2-95d7-a6e7835997ad)

## Features

- Fully rendered Terminal Dashboard via the *Ratatui* crate.
  - Graphs showing the states with the most earthquakes in the past week, as well as states with the highest average earthquake   magnitude the past week.
  - Client data rendered as a table and colour coded by an assigned risk level. 
- Serialization of csv data into native Rust datatypes via the *csv* and *Serde* crates.
- Async Rust via the *Tokio* crate.
- USGS API integration via the *Reqwest* crate.

## Shortcomings

Given the small time frame for this assessment (approx 2hrs), this program has a few limitations, namely that there is limited flexibility when resized, so some content may get truncated and not shown (Its advised to use this program in full screen to see all the data). Allowed more time, I would have solved these issues through the use scrollbars and a flexible rendering framework.

Another limitation is that in the case either the client or earthquake data cannot be retrieved, the application halts and alerts the user. Given more time, those errors would be relayed to the user in the TUI, without necessarily halting the application. 

# Running the Program

This program is written in Rust can be run using *Cargo*, Rust's package manager and build system. To run it, you may need to install rust and cargo.

## Installing Rust

Rust can be installed here:

https://www.rust-lang.org/tools/install

This will install *rustup*, which is a tool that manages both *Rust* and *Cargo*.
- If you are running linux, you will need to make sure *openssl* is on the path, as one of the dependencies require it.

## Operating the Program

First clone this repo into the desired location, cd into the newly cloned directory, then the program can be started with the command: *cargo run*
- press 'q' to close the dashboard

Tests can evaluated using the command: *cargo test*

I look forward to hearing back from you,

Cheers,

Shiloh
