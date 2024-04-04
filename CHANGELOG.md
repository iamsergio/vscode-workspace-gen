# Changelog



## [1.4.0] - 2024-04-04

### 🚀 Features

- Add --create-cmake-presets

### 🧪 Testing

- Remove old natvis before running test_download_qtnatvis

### ⚙️ Miscellaneous Tasks

- *(ci)* Pass --features qt to clippy

## [1.3.0] - 2024-04-03

### 🚀 Features

- Replace env vars in the form of $${env_var}

### ⚙️ Miscellaneous Tasks

- Add qt6.natvis to .gitignore
- Add codespell pre-commit hook
- *(ci)* Add a pre-commit GH action
- Add a .codespellrc
- Allow ser:: in codespell
- Allow ser in codespell

## [1.2.0] - 2024-03-31

### 🚀 Features

- Added --download_qtnatvis option

### ⚙️ Miscellaneous Tasks

- *(vscode)* Qualify the json extension a bit more
- *(vscode)* Update workspace file

## [1.1.0] - 2024-03-31

### 🚀 Features

- Support a .vscode-workspace-gen.json file
- Allow to generate per OS

### 📚 Documentation

- Fix README typo
- Make it clear that macos is an available gen.os value
- Updated README regarding json_indent

### ⚙️ Miscellaneous Tasks

- *(vscode)* Regenerate workspace
- Update Cargo.lock

## [0.2.2] - 2024-03-30

### 🚀 Features

- Test release-plz again

## [0.2.1] - 2024-03-30

### 🚀 Features

- Introduce "gen.os"
- @@ now honours gen.os
- Testing feat commit message tag

### 🐛 Bug Fixes

- Reduce indentation to match vscode
- Gen.globals were being inserted if not present in template

### 📚 Documentation

- Mention gen.os

### ⚙️ Miscellaneous Tasks

- Install git-cliff via action so it's cached
- Add .pre-commit support

## [0.1.2] - 2024-03-30

### 🚜 Refactor

- Add token_kind()

### 🧪 Testing

- Add a failing test for inner expansion
- Fix expected output
- Add a failing test for inline object expansion

### ⚙️ Miscellaneous Tasks

- Install git-cliff

### Minor

- Moved tests to a separate file
- Fix typo
- Add/remove comments
- Code simplification
- Rename a test function

