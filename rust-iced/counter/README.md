# Counter in [Rust-Iced](iced.rs)

Here, the struct `Counter` defines the state of the counter application.

1. value - represents the current count
2. count_button - represents whether the button is pressed or not

To implement `Sandbox` trait for `Counter`, we defined `new`, `title`, `view`, and `update` functions.

`new` - returns Counter struct with default values
`title` - returns a string. This string is used to modify title.
`view` and `update` - renders the window and updates the window

## Objectives

1. Create a frame - done using implementation of `Sandbox`
2. Modify the title - done using `title` function in `Sandbox` trait
3. Button - implemented using `Button::new`. This function takes `iced::button::State` and `Text` as argument to form a button. To define the message to be passed after button is pressed, we can use `on_press` with `Message` argument.
4. Modifying the window - this is achieved modifying the `iced::Settings` struct passed to `run` function for `Sandbox` trait.
