# SPDX-License-Identifier: MIT

# Runs cargo update and creates a PR

cargo update

if [ -n "$(git status --porcelain)" ]; then
    git add Cargo.lock
    git commit -m "chore: Cargo update"
    git checkout -B temp/cargo-update-`git log -1 --pretty=format:"%H"`
    git push --set-upstream origin `git branch --show-current`
    gh pr create --base main --title "chore: cargo update" --body "Automatically created via GH action."
fi
