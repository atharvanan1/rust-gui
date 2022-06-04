# [Rust-Iced](iced.rs) examples

Iced follows Elm Architecture for GUI design. It has following components:

1. State - defining variables that determine the state of the system.
2. Message - event messaging needed for communicating actions/events taken by user
3. View Logic - Logic to show the window rendering
4. Update Logic - Logic to update the window rendering

To start a typical application, use `Sandbox` trait for a struct which represents state.

Methods required for `Sandbox` trait will help you manipulate the following:

1. System State
2. Event Management
3. Update and View Logic
4. Adds a default run method to render the application

For modifying the window, you'll need to use `iced::Settings` and `iced::window::Settings`.
