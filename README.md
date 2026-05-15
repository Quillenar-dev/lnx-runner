lnx-runner (literally meaning; 'Linux Nested Executable Runner') is intended to be a packaging format and a super-lightweight runner written in Rust that turns apps into isolated, permission-gated executables.

The goal is to create a packaging format similar to Android's .apk, delivered primarily as a user-friendly CLI tool for Linux.

I am building this to:

    Bridge the gap between portable apps (like AppImage) and secure sandboxing (like Flatpak).
    Master Rust and low-level Linux kernel implementations (Namespaces, Landlock, and Cgroups).

IMPORTANT!: This project is currently in its earliest stages. It is a learning experiment I wanted to get it working, as I believe it'd help me learn more about Linux and Rust ecosystem.
