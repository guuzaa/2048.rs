# 🎮 2048 Puzzle Game

A terminal-based implementation of the popular 2048 puzzle game written in Rust. 🦀

## 📝 Description

This project is a command-line version of the 2048 game. The game is played on a 4x4 grid where the player combines numbered tiles by sliding them in four directions (up, down, left, right). The goal is to create a tile with the number 2048. 🏆

## ✨ Features

- 🖥️ Terminal-based user interface
- 🎯 Smooth gameplay with arrow key controls
- 🎲 Random tile generation
- 🚫 Game over detection

## 🛠️ Requirements

- 🦀 Rust (edition 2021)
- 📦 Cargo (Rust's package manager)

## 🚀 Installation

1. Clone this repository:
   ```
   git clone https://github.com/guuzaa/2048.rs.git
   cd 2048.rs
   ```

2. Build the project:
   ```
   cargo build --release
   ```

## 🕹️ How to Play

1. Run the game:
   ```
   cargo run --release
   ```

2. Use the arrow keys to move the tiles:
   - ⬆️ (Up Arrow): Move tiles up
   - ⬇️ (Down Arrow): Move tiles down
   - ⬅️ (Left Arrow): Move tiles left
   - ➡️ (Right Arrow): Move tiles right

3. Press 'q' to quit the game at any time. 🚪

4. The game ends when no more moves are possible or when you create a 2048 tile. 🎉

## 📜 Game Rules

- 🎬 The game starts with two random tiles, each with a value of either 2 or 4.
- 🔄 Each move slides all tiles in the chosen direction as far as possible.
- 🔢 If two tiles with the same number collide, they merge into one tile with the sum of their values.
- 🆕 After each move, a new tile (2 or 4) appears in an empty spot on the board.
- 🏅 The game is won when a tile with the value 2048 appears on the board.
- 🛑 The game is over when there are no empty cells and no possible merges.

## 🤝 Contributing

Contributions, issues, and feature requests are welcome! 🙌 Feel free to check the issues page if you want to contribute.

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details. 📜