# nene

Remove ANSI escape codes in file.

## Usage

output to stdout:

```shell
$ nene xxx.log
...
```

output to file:

```shell
$ nene xxx.log --out new_xxx.log
output: new_xxx.log
```

## Install

### Homebrew

```shell
$ brew tap YusukeHosonuma/nene
$ brew install nene
$ nene bitrise.log > plain_bitrise.log
```

### from Source

```shell
$ git clone https://github.com/YusukeHosonuma/nene.git
$ cd nene
$ cargo install
```
