# epic_killer

A small command line program for windows that kills the epic launcher after all epic store games have stoppped.

## Use case

When you run an epic store game as a non-steam game in steam it always starts the epic launcher in the background.
But when you run such a game while the epic launcher is running, it doesn't work (at least with steam link).
This small progam stops the epic launcher after all epic store games have stopped.

## Build
`cargo build --release`

## Run
`target/release/epic-killer.exe EPIC_GAMES_DIRECTORY`
