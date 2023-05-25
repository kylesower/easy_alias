# Installation
You will first need to [install Rust/Cargo](https://www.rust-lang.org/tools/install). Once that's done and you verify that `cargo --version` works in your terminal, you can run the following:
## Linux or Mac
`git clone https://github.com/kylesower/easy_alias && cd easy_alias && make`

You may need to source your ~/.bashrc or restart your terminal afterwards.

## Windows
This will only work in a bash shell:
```
git clone https://github.com/kylesower/easy_alias && cd easy_alias
cargo build --release
cd target/release
mv easy_alias.exe ea.exe
echo "export PATH=$(pwd):\$PATH" >> ~/.bashrc
source ~/.bashrc
mkdir ~/.config
```

# Usage

Simply type

`ea <alias> <bash cmd>`

to add an alias for the provided bash command. If the command has spaces, 
you'll need to enclose it in quotes. See `ea --help` for more info.

Then, whenever you want to run the command, use

`ea <alias>`
