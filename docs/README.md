# How To Use EDX

Clone this repo and change directory into it...
```
$ git clone https://github.com/cryptosbyte/EDX
$ cd EDX/
```

Build the Rust file by running the following...
```
$ cargo build
```

Then move the executable to the current directory...
```
$ mv ./target/debug/edx ./
```

Create a JSON file called `edx` (for the program to read it)...
```
$ touch edx.json
```

Make the insides of `edx.json` similar to the one in the root directory (of this repo)...
```json
{
    "scripts": {
        "list_dir": "ls",
        "cls": "clear"
    }
}
```

Now run EDX with the script name...
```
$ ./edx list_dir # For example; change it to the relevant name
```