# Changelog



## [3.1.0] - 2024-06-03

### 🚀 Features

- Display a "type" column when listing projects
- Add "depends" json field

## [3.0.0] - 2024-06-01

### 🚀 Features

- -c is now short-hand for --create project
- Allow to copy projects without their parent folder
- [**breaking**] Remove qt feature

### ⚙️ Miscellaneous Tasks

- Use serde deserialize to read to struct in one go
- Minor rename
- Minor refactoring for readability

## [2.3.0] - 2024-05-28

### 🚀 Features

- Added project.rs
- Added --list-projects
- Projects are printed with comfy_table now
- Added --create-project

## [2.2.3] - 2024-05-21

### 🐛 Bug Fixes

- Fix message about workspace being generated

### 📚 Documentation

- Remove trailing commas from example

## [2.2.2] - 2024-05-21

### 🐛 Bug Fixes

- Fix outdated README

### 📚 Documentation

- Explain people should fix their issues instead of reporting

## [2.2.1] - 2024-05-20

### ⚙️ Miscellaneous Tasks

- Update Cargo.lock dependencies

## [2.2.0] - 2024-05-17

### 🚀 Features

- Add --create-clang-format option

### ⚙️ Miscellaneous Tasks

- Don't specify features when running clippy
- Run tests for feature cpp as well

## [2.1.0] - 2024-05-10

### 🚀 Features

- Qt is now a default feature

### 🐛 Bug Fixes

- Fix location of output filename when not specified

### ⚙️ Miscellaneous Tasks

- Minor renaming
- Pass --no-default-features

## [2.0.0] - 2024-05-10

### 🚀 Features

- [**breaking**] Cleanup args handling

### ⚙️ Miscellaneous Tasks

- Remove unneeded strategy from ci yml file

## [1.7.0] - 2024-05-08

### 🚀 Features

- Copy the cmake preset directly into the template

### ⚙️ Miscellaneous Tasks

- Fix warning about unused imports

## [1.6.0] - 2024-04-26

### 🚀 Features

- Generated CMakePresets.json now has CMAKE_EXPORT_COMPILE_COMMANDS

## [1.5.0] - 2024-04-16

### 🚀 Features

- Warn the user if Qt env vars are missing

### ⚙️ Miscellaneous Tasks

- Minor rename
- Fix build
- Fix non-qt build

## [1.4.0] - 2024-04-16

### 🚀 Features

- Add --create-cmake-presets
- Add --create-default-vscode-workspace convenience for Qt

### 📚 Documentation

- Improve docs

### 🧪 Testing

- Remove old natvis before running test_download_qtnatvis

### ⚙️ Miscellaneous Tasks

- *(ci)* Pass --features qt to clippy
- Added workflow to run cargo update
- Rename cargo workflow name
- Setup git author name for cargo update PR
- Minor comments
- Cargo update
- Added a build.sh

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

