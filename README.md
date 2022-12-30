# CLI LayerZero Explorer
Just a simple command line interface to view the status of a cross-chain message that's going through LayerZero.  

## Usage
The executable is `cli-lz-explorer`, in the root directory. It was built using `cargo build --release`.  

Here are the instructions, which can also be read with `./cli-lz-explorer --help`:  

```
Usage: cli-lz-explorer [OPTIONS] <TX_HASH> <NETWORK>

Arguments:
  <TX_HASH>  The source chain's transaction hash that sent a cross-chain transaction
  <NETWORK>  The network type on which the transaction was on

Options:
  -a, --all      Whether or not to show all of the information for each cross-chain message
  -v, --verbose  Whether or not to show all of the information without shortening for each cross-chain message
  -h, --help     Print help information
  -V, --version  Print version information
```

## Add Executable to PATH
https://apple.stackexchange.com/a/41586  
https://wpbeaches.com/make-an-alias-in-bash-or-zsh-shell-in-macos-with-terminal/

