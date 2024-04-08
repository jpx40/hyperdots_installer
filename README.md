you need rust and Cargo installed to excute the code

Install Rust and Cargo

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```


add the following to your .zshrc / .bashrc

```
export PATH=~/.cargo/bin/$PATH
```

clone the repo 

to build the Code u have to use cargo build or cargo build --release

the binary is in target/dubug/installer or with relaese flag target/relaese/installer

you have to pass arguments to the binary 

--list "app_list.toml"
--out out.lst
-d dep_list.toml
--menu editor=neovim



