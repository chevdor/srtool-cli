# Description

You can find the changelogs below.

# Downloads

Download the binary for your OS from below:
- **Linux**
    - [Debian package]({{ DEBIAN_URL }})
- **MacOS**
    - [Archive]({{ MACOS_TGZ_URL }})
# Install

## From source

```
cargo install --locked --git https://github.com/chevdor/srtool-cli --tag {{ VERSION }}
```

## Linux
```
wget {{ DEBIAN_URL }}
dpkg -i srtool_linux_amd64_{{ VERSION }}.deb
srtool --help
```

## MacOS

```
brew tap chevdor/srtool-cli https://github.com/chevdor/srtool-cli
brew update
brew install chevdor/srtool-cli/srtool
```

{{ CHANGELOG }}
