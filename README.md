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

    cargo install --git https://gitlab.com/chevdor/srtool-cli

## Usage

**help**

    srtool-cli 0.3.0
    chevdor <chevdor@gmail.com>
    This utility allows invoking the srtool with the right parameters and environment variables. See
    documentations of each command below

    USAGE:
        srtool [FLAGS] <SUBCOMMAND>

    FLAGS:
        -h, --help        Prints help information
        -j, --json        Whether we output json or something for humans
        -n, --no-cache
        -V, --version     Prints version information

    SUBCOMMANDS:
        build      Build opts
        help       Prints this message or the help of the given subcommand(s)
        info       Info opts
        version    Version opts

**build**

    srtool-build 0.3.0
    chevdor <chevdor@gmail.com>
    Build opts

    USAGE:
        srtool build [OPTIONS] --package <package> --runtime-dir <runtime-dir> [path]

    ARGS:
        <path>    By default, srtool will work in the current folder. If your project is located in
                  another location, you can pass it here [default: .]

    FLAGS:
        -h, --help       Prints help information
        -V, --version    Prints version information

    OPTIONS:
            --build-opts <build-opts>
                You may pass options to cargo directly here. WARNING, if you pass this value, the
                automatic build options for Kusama and Polkadot will not be passed and you need to take
                care of them manually. In general, you should never use this option unless you HAVE to
                [env: BUILD_OPTS=]

            --default-features <default-features>
                Passing this is less involved than passing BUILD_OPTS. It allows changing the list of
                default features while keeping the automatic features detection. This value is useless
                if BUILD_OPTS is set [env: DEFAULT_FEATURES=]

        -p, --package <package>
                Provide the runtime such as kusama-runtime, polkadot-runtime, etc... [env:
                PACKAGE=polkadot-runtime]

            --profile <profile>
                The default profile to build runtimes is always `release`. You may override the default
                with this flag [env: PROFILE=] [default: release]

        -r, --runtime-dir <runtime-dir>
                If your runtime is not in the standard location runtime/<chain_name> you can pass this
                args to help srtool find it [env: RUNTIME_DIR=runtime/polkadot]
