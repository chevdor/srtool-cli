# srtool cli

![srtool cli 512px](resources/srtool-cli_512px.png)

This project is NOT the `srtool` docker image that is actually used to build Substrate Wasm Runtime. This utility requires docker to be installed and running and will invoke the `srtool` image to help you build a Substrate runtime.

This project is a cli interface to docker to simplify using the `srtool` docker image. With this executable, you no longer need
to set and maintain a long and complex alias, as currently described in the `srtool` documentation.

## Install

If you previously defined `srtool` as an alias, you will first need to remove it first.

### Clean up

If you used `srtool` in the past, you previously used an `srtool` alias. You can check with:

    type srtool

If you see some output mentioning "srtool is an alias for docker run…​", you have an alias set and we need to remove it:

    unalias srtool

This alias is likely set in your `.bash_profile` or `.zshrc`, make sure to remove this alias there as well.

### Install

    cargo install --git https://github.com/chevdor/srtool-cli

## Usage

**help**

    srtool-cli 0.7.1
    chevdor <chevdor@gmail.com>
    srtool is cli allowing to control the srtool docker image

    USAGE:
        srtool [OPTIONS] <SUBCOMMAND>

    OPTIONS:
        -h, --help             Print help information
        -i, --image <IMAGE>    Choose an alternative image. Beware to choose an image that is compatible
                               with the original srtool image. Using a random image, you take the risk
                               to NOT produce exactly the same deterministic result as srtool [default:
                               paritytech/srtool]
        -j, --json             This option is DEPRECATED and has no effect
        -n, --no-cache         Do not use the local cached tag value
        -V, --version          Print version information

    SUBCOMMANDS:
        build      Start a new srtool container to build your runtime
        help       Print this message or the help of the given subcommand(s)
        info       Provide information about the srtool container and your repo
        pull       Simply pull the srtool image and do not run anything else
        version    Show the versions of the srtool container. Use --version if you want the version
                   of this executable

**version**

    srtool-version 0.7.1
    chevdor <chevdor@gmail.com>
    Show the versions of the srtool container. Use --version if you want the version of this executable

    USAGE:
        srtool version

    OPTIONS:
        -h, --help       Print help information
        -V, --version    Print version information

**info**

    srtool-info 0.7.1
    chevdor <chevdor@gmail.com>
    Provide information about the srtool container and your repo

    USAGE:
        srtool info [OPTIONS] --package <PACKAGE> [PATH]

    ARGS:
        <PATH>    By default, srtool will work in the current folder. If your project is located in
                  another location, you can pass it here [default: .]

    OPTIONS:
        -h, --help                         Print help information
        -p, --package <PACKAGE>            Provide the runtime such as kusama-runtime, polkadot-runtime,
                                           etc... [env: PACKAGE=]
        -r, --runtime-dir <RUNTIME_DIR>    If your runtime is not in the standard location
                                           runtime/<chain_name> you can pass this args to help srtool
                                           find it [env: RUNTIME_DIR=]
        -V, --version                      Print version information

**pull**

    srtool-pull 0.7.1
    chevdor <chevdor@gmail.com>
    Simply pull the srtool image and do not run anything else

    USAGE:
        srtool pull

    OPTIONS:
        -h, --help       Print help information
        -V, --version    Print version information

**build**

    srtool-build 0.7.1
    chevdor <chevdor@gmail.com>
    Start a new srtool container to build your runtime

    USAGE:
        srtool build [OPTIONS] --package <PACKAGE> [PATH]

    ARGS:
        <PATH>    By default, srtool will work in the current folder. If your project is located in
                  another location, you can pass it here [default: .]

    OPTIONS:
        -a, --app
                Enable the "app" mode which is a mix of json output and outputting progress during the
                build. This flag is recommended for CI. the json output will be provided as a single
                line at the end in compact mode

            --build-opts <BUILD_OPTS>
                You may pass options to cargo directly here. WARNING, if you pass this value, the
                automatic build options for Kusama and Polkadot will not be passed and you need to take
                care of them manually. In general, you should never use this option unless you HAVE to
                [env: BUILD_OPTS=]

            --default-features <DEFAULT_FEATURES>
                Passing this is less involved than passing BUILD_OPTS. It allows changing the list of
                default features while keeping the automatic features detection. This value is useless
                if BUILD_OPTS is set [env: DEFAULT_FEATURES=]

        -h, --help
                Print help information

        -j, --json
                Enable json output, same than the global --json option

            --no-cache
                Passing this flag allows completely disabling caching. As a result, no cargo-home will
                be mounted to the srtool image. There is no known issue with having the cache ON, this
                is why it is the default

        -p, --package <PACKAGE>
                Provide the runtime such as kusama-runtime, polkadot-runtime, etc... [env: PACKAGE=]

            --profile <PROFILE>
                The default profile to build runtimes is always `release`. You may override the default
                with this flag [env: PROFILE=] [default: release]

        -r, --runtime-dir <RUNTIME_DIR>
                If your runtime is not in the standard location runtime/<chain_name> you can pass this
                args to help srtool find it [env: RUNTIME_DIR=]

        -V, --version
                Print version information

## Contributing

If you landed here, you likely want to contribute the project. Let me thank you already.
There are several ways you can help. Please start with the few notes below.

### Features and issues

Whether you are reporting an issue you ran into or requesting a new feature, please [open an issue here](https://github.com/chevdor/srtool-cli/issues/new).

You know the drill: please try to provide some context information, the version you used, your OS, how to reproduce. That will greatly help solving your issue quicker.

### Documentation

The documentation of this project is mainly done using [AsciiDoc](https://asciidoc.org/). Unfortunately, it takes [litterally ages](https://github.com/github/markup/issues/1095) for Github to support THE feature that makes AsciiDoc shine.
As a result, for now, this project is generating the markdwown from AsciiDoc. In short that means that you should NOT modify any `.md` file but change the `.adoc` ones and run `just md` to generate all the markdown.

### Tooling

This project is mainly using Rust so you will need to install the Rust compiler. Make sure everything works with the latest **stable** version of Rust.

You will find a `justfile` in the root of the repo. This is to be used with [just](https://github.com/casey/just) so you may want to install that as well. Just type `just` 😁 to discover how it can help you.

Before submitting your code, do a `cargo clippy` stop to make sure everything works fine. Don’t forget to `cargo fmt --all` as well if you want to be friend with the CI. No surprise, the test can be ran using `cargo test`.

You may activate the **logs** for the project using `RUST_LOG=debug` for instance.

### Pull Requests

PRs are welcome. Feel free to open them early before putting too much effort (you may start with a draft). This way you can ping me ([@chevdor](https://github.com/chevdor)) if you want my opinion on what and how you are putting your change together.
