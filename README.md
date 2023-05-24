# Installation
You will first need to [install Rust/Cargo](https://www.rust-lang.org/tools/install). Once that's done and you verify that `cargo --version` works in your terminal, you can run the following:

`git clone https://github.com/kylesower/easy_alias && cd easy_alias && make`

You may need to source your ~/.bashrc or restart your terminal afterwards.

Note: this is untested on Windows.

# Usage

Simply type

`ea <alias> <bash cmd>`

to add an alias for the provided bash command. If the command has spaces, 
you'll need to enclose it in quotes. See `ea --help` for more info.

Then, whenever you want to run the command, use

`ea <alias>`
