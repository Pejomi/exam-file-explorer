# Exam-File-Explorer
**by the group Pejomi (Peter, Jonas & Mie)**

## Description
This is a Rust program that can be used to explore the files and directories on your computer. You can navigate through the directories and see the files and directories in each directory.

![alt text](image.png)

## Features
You can do the following things in the program:

- Navigate through directories by clicking on the directories' folder buttons
- Go back to the previous directory by clicking on the ```⬅``` in the top left corner
- See or copy the path of the current directory by clicking at the top of the window
- Open file description by clicking on a file
- Search for files and directories by name in the search bar
- Change theme-mode by selecting the theme-mode in the bottom right corner
- See the current amount of files and directories for the present directory in the bottom left corner
- Copy or create a new file by right-clicking on a file or directory

## How to run the program
To run the progrom type the following command in the terminal:
```bash
cargo run
```
<br/>

Notice, that by default the program will start and display files from the ```test-directory``` from this project. If you want to start in another directory, you can change the path in the ```app.rs``` file in the ```new``` function:

```rust
start_dir: PathBuf::from("test-directory"),
```

## Egui and StripBuilder
The program uses the GUI library ```egui``` to create and manage the GUI with elements like buttons, text and input fields for the user to interact with.

With use of the ```egui_extras``` crate, we also use ```StripBuilder``` to divide the window into different sections:

![img.png](img.png)


## Use of external crates
We use some different external crates in this program like:

- **egui**: for the GUI
- **egui_extras**: for special GUI features like ```StripBuilder```
- **walkdir**: for walking through directories
- **enum-iterator**: for iterating over enums
