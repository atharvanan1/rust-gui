# Temperature in [Rust-Iced](iced.rs)

Here, the struct `Temperature` defines the state of the Temperature application.

1. c_value - represents the current value for C 
2. f_value - represents the current value for F
3. c_text_state - represents the current status for C text input block - to be used by iced
4. f_text_state - represents the current status for F text input block - to be used by iced

Since, the `Sandbox` trait is already implemented from the Counter project. We'll be using that as is.

## Objectives

1. Get input from text input - `Message::InputChange(String)` will be used to update and `String` will contain the value. We save this value to the state of application. At the same time, we update the value for the other text_input. 
2. For `TextInput`, it takes a state variable, a placeholder, and the value to display. Then we point to `Message::InputChange` which is an enum with `(String)`, from which we'll extract the value.
 