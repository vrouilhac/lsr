# lsr

lsr is a command to list directories content (almost like `ls` command). This project has been made for learning purpose.

### Installation

First, download the zip of release, then head to the folder and run the command below
```
cargo build --release
```

Then inside your `.bashrc` or `.zshrc`
```
alias lsr='<PATH_TO_THE_REPO_FOLDER>/target/release/./lsr'
```

### Usage

The basic usage of the command doesn't print anything, this allows for listing specific file type.

```
-d	: Print directories
-h  : Print hidden files
-f  : Print files
-s  : Print symlink files
```
