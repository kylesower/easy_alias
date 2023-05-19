SHELL=/bin/bash

build:
	cargo build --release
	@echo "PATH=\"`pwd`/target/release:\$$PATH\"" >> ~/.bashrc
	@mv ./target/release/easy_alias ./target/release/ea
	@echo "Added easy_alias to path in ~/.bashrc."
	@echo "You may need to 'source ~/.bashrc' or restart your terminal."

