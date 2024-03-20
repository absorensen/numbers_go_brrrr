# egui-winit-wgpu-template

The reason for making this template is two-fold. First of all, it can be used for a basic analysis exercise. So
far you have seen either very small projects or projects too big for you to get much out of. This is an attempt
at a medium sized project which you should be able to analyze. Except for the graphics stuff. Don't worry about
the minutia of all the WGPU stuff. It draws a triangle that rotates. But look at what it does with configuration,
serialization, command line arguments and GUI. Try to look at the big picture. Secondly, you can use the template as a starting point for doing your own project. Just replace all the graphics stuff with whatever you want.

The dependencies in the project template are versioned specifically to function with specific versions of
```nokwha```, a camera library, and ```burn```, a deep learning library. Having these versions align means that
only one version of WGPU needs to be downloaded and built as a dependency. This saves a lot of space and compilation
time, but hopefully it should also make transitioning data from each library to the other should happen without
any copying and other kerfuffles.

## Packages to install - Ubuntu 22.04
One a clean install of Pop! OS, which was running Ubuntu 22.04 under the hood, I found I needed to install at least:

* libssl-dev
* clang
* libatk1.0-dev
* libpango-1.0-0
* libgtk-3-dev
* maybe cmake?

to get things up and running.

## How to run

```cmd
cargo run --release -- --nogui --config "some/path/config.toml"
cargo run --release
```

There are currently two command line arguments you can supply.

The ```--config``` flag requires a string value. It is the path to a config file (config.toml) which can set the
values for the program. If you use the GUI to set the values that you would like, and remember to supply a valid
path, the application in GUI mode will save a config file which you can load using this flag. This is especially
useful if you want to use the GUI for configuration and then run without GUI for maximum performance. If no valid
config path is given, the application will open a file dialog to allow you to manually pick a config file. If you
don't have one, you can just click cancel and it will fill out default values.

If you supply ```--nogui```, the application will not use a GUI. It will take its values from the config file
or default values, as well as keyboard and mouse inputs.

## How to log
For logging, the ```env_logger``` crate is used. In order to see the output, you need to set an environment
variable. I would really recommend checking out the [online documentation][0]. if you are going to use it. To
get started quickly you can just set a single flag in your environment.

If you are on Windows:

```cmd
set RUST_LOG=info
```

If you are on Linux/Max try whats written below as a prefix to the cargo run .... line:

```cmd
RUST_LOG=info cargo run --release -- --nogui --config "losdaw"
```

[0] : https://docs.rs/env_logger/latest/env_logger/
