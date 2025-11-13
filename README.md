# Todo List App

A simple Todo List application built with Dioxus 0.7. This app allows you to add and remove todo items with a clean, dark-themed interface.

## Features

- Add new todos by typing in the input field and pressing Enter or clicking "Add Todo".
- Remove todos by clicking the "Remove" button next to each item.
- Responsive design with a dark theme.

## Project Structure

```
project/
├─ assets/
│  ├─ main.css # Styles for the app, including dark theme
├─ src/
│  ├─ main.rs # Main application code with the Todo List component
├─ Cargo.toml # Dependencies and features
├─ Dioxus.toml # Dioxus configuration
```

## Running the App

To run the app in desktop mode (default):

```bash
cargo run
```

To run in web mode:

```bash
dx serve --platform web
```

Or serve with the default platform:

```bash
dx serve
```

## Technologies Used

- **Dioxus 0.7**: For building the UI with RSX.
- **Rust**: Programming language.
- **CSS**: For styling.
