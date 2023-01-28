## pwdf

join current path with given path

## Install

```
cargo install pwdf
```

## Example

```shell
> pwd
/Users/ashigirl96/.ghq/src/github.com/ashigirl96/pwdf
> pwdf
/Users/ashigirl96/.ghq/src/github.com/ashigirl96/pwdf
```

```shell
> pwdf ../pwdf/Cargo.lock
/Users/ashigirl96/.ghq/src/github.com/ashigirl96/pwdf/Cargo.lock
```

## More...

```shell
alias pwd=pwdf
```