# Termup - A Terminal Setup Tool

Automate your terminal and development environment setup with simple TOML configuration files.

> [!IMPORTANT]
> This tool is still work-in-progress, but feel free to play around with it! 😊

## Features

- 🚀 **Automated setup** - Run pre-defined commands in sequence
- 📝 **Simple TOML config** - Easy to read and write
- 🔄 **Reproducible environments** - Same setup across machines
- 🎯 **Workflow support** - Semi-manual steps with prompts and notes
- ✅ **Dependency checking** - Verify required commands, files, and directories exist

## Installation

**WIP**

Right now the easiest way is to install the tool via `cargo`:

```shell
cargo install --git https://github.com/brlywk/termup_rs
```

## Quickstart

1. Create a `config.toml`:
```toml
[info]
name = "My Setup"
version = "0.4.2"

[setup]
main = ["hello_world"]

[[action]]
id = "hello_world"
name = "Hello World"
cmd = "echo"
args = ["Hello World!"]
```

2. Run it:
```shell
termup run
```

## Usage

Termup comes with three command modes:

`run`, `workflow` and `config`.

All three modes use a global flag `-c` or `--config` that specifies the TOML file containing the setup config.
The default value for this flag is `config.toml`.

### `run`

You can use `run` to run the automated setup process:

```shell
# Run setup and use "config.toml" config file in the same folder:
termup run

# Run setup and use custom config TOML file:
termup run -c example.toml
```

Note that `run` will not run any `workflow` steps (see below).

### `workflow`

Workflows are for semi-automated tasks that need manual preparation or confirmation.

**Example use case:** Setting up SSH config requires you to first generate keys manually,
but then you want to automate copying your standard host configurations.

```toml
[[workflow]]
id = "setup_ssh"
name = "Setup SSH Configuration"
description = "Configure SSH with standard hosts"
notes = [
    "Make sure you've generated SSH keys first:",
    "ssh-keygen -t ed25519 -C 'your_email@example.com'",
]
prompt = "Have you generated your SSH keys?"
actions = ["copy_ssh_config", "set_permissions"]
```

To run workflows in general:

```shell
# Run all workflows:
termup workflow

# Run only the example workflow above:
termup workflow -w setup_ssh

# Run workflow with ID "something-nice" from "example.toml":
termup workflow -c example.toml -w something-nice
```

### `config`

The config command "pretty prints" the specified config TOML to the terminal if you don't want to bother loading
up your favorite editor 😉:

```shell
# Print config:
termup config

# Print config in "example.toml":
termup config -c example.toml
```

## Configuration File

Termup uses a toml file to specify all setup steps that should be taken.

A config file consists of up to four sections:
-  `info` object containing general information about the config
-  `setup` object containing the pre-, main- and post-setup actions to run
-  `action` array containing the atomic steps used to define the setup and workflows
-  `workflow` array containing custom, semi-manual workflows to run

Following is an overview of how each section is structured:

### Example file

Check out the [example file](https://github.com/brlywk/termup_rs/tree/main/config/example.toml) to
hopefully get a better picture.

### Schema

The full schema for the config TOML can be found here:

[Config TOML schema](https://raw.githubusercontent.com/brlywk/termup_rs/main/schema.json)

### Details
#### `info`

```toml
[info]
name = "Arbitrary name of your terminal config"
version = "0.0.1" # So you can distinguish between configs if you'd like
```

#### `setup`

`main` is required. `pre` and `post` are optional.

```toml
[setup]
pre = ["actions_to_run", "as_prerequisites"]
main = ["your_main", "setup_actions"]
post = ["must_run", "after_some", "main_step"]
```

**Quick note:**

IDs are just arbitrary strings, there is no pattern ID naming has to follow.

#### `action`

```toml
[[action]]
name = "Human readable name"
id = "random-string_id"
cmd = "command_to_run"
args = ["some", "number", "of", "args"]
items = ["optional", "values", "to", "append", "per", "run"]
working_dir = "/dir/to/run/cmd/in"
requires = ["cmd_that_should_already_exist"]
requires_files = ["files", "that", "should", "exist"]
requires_dir = "dir/that/should/exist"
content = '''
Some content to provide as stdin for
the command to run...
'''
```

`args` are the base arguments passed to the command.

If `items` is omitted or empty, the command is run once with `args`.

If `items` is provided and non-empty, the command is run once per item, appending that item to `args`.

For example:

```toml
[[action]]
name = "Install Homebrew packages"
id = "homebrew_packages"
cmd = "brew"
args = ["install"]
items = ["bat", "fd", "ripgrep"]
```

This runs:
```
brew install bat
brew install fd
brew install ripgrep
```


#### `workflow`

```toml
[[workflow]]
id = "id_of_workflow"
name = "Human readable name of workflow"
description = "Some description to help you remember what the workflow does"
requires = ["required", "commands"]
notes = [
  "These notes will be printed when the workflow is executed.",
  "The idea is to give yourself some hints on what to do before running the workflow,",
  "e.g.: make sure to add some directory to your $PATH variable",
]
prompt = "This is yes/no prompt being shown as confirmation to run the workflow."
actions = ["actions", "to", "run"]
```

## Motivation

I'm one of those people that like to regularly try out new tools and setups, to the point that almost inevitably I feel
like I need to do a fresh installation of my system 😁

And although there are absolutely fantastic tools like [chezmoi](https://www.chezmoi.io/) that handle the full dotfile
and setup process (that you should definitely check out!), these tools almost had too many features for my liking, and
I very regularly stopped syncing my config changes with these tools.

So I arrived at the point where I found that it's just easiest for me to keep my configs and my terminal setup
separate, and with that, this little tool was born. Coincidentally, I also wanted to finally dive into Rust and found
this to be a nicely scoped project for that 🦀

## Planned Features

- General config setting to set base commands to use (currently, `sh` is
  hardcoded)
- Dry-run feature
- Option to attempt installation/creation of `require*` fields contents
- Flag for `run` that also executes all workflow steps
- Code cleanup...

