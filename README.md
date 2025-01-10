# belo

Belo is a command-line tool for managing player ratings using the Elo rating system. It is designed to be simple and easy to use, with a focus on the core functionality of managing player ratings. It allows you to containerize projects, add players, record match outcomes, and view player statistics in a neatly formatted table.

## Features


- **Project Management**: Initialize, activate, list, and deactivate multiple projects.
- **Player Management**: Add, remove, and list players within a project.
- **Match Recording**: Record match outcomes between players.

## Quick Start

```bash

# Install Belo
pip install https://github.sec.samsung.net/SR-SE/belo/releases/download/v0.1.0/belo-0.1.0-py3-none-manylinux_2_31_x86_64.whl
```

You can now use the `belo` command to manage player ratings. Here are some example commands to get you started:

```bash
# Initialize a new project
belo init my_project

# Add players to the project
belo add player1
belo add player2
belo add player3
```

You can now record match outcomes between players and view player statistics. Here are some example commands:


```bash
# Record a match outcome
belo vs player1 gt player2
belo vs player2 eq player3
belo vs player3 lt player1

# View player statistics
belo head {optional n}
belo info player1

# Deactivate the project
belo deactivate

# Delete the project
belo delete my_project
```