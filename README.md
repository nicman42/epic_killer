# epic_killer

A small command line program for windows that kills the epic launcher after all epic store games have stoppped.

## Use case

When you run an epic store game as a non-steam game in steam it always starts the epic launcher in the background.
But when you start such a game while the epic launcher is running, it doesn't work (at steam link).
This small progam stops the epic launcher after all epic store games have stopped so it's possible to start again an epic store game at steam link.

## Build
`cargo build --release`

## Run
`target/release/epic-killer.exe EPIC_GAMES_DIRECTORY`
