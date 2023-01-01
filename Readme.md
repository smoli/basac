BAsAC
=====

The Browser as a Computer - Project, a.k.a "another silly idea".

The idea is, to provide an environment that allow a user to use the
browser as a simple computer, i.e. load programs and execute them.


## bbbasic

Subset of BBC-Basic.

## bbcli

Commandline interpreter for bbbasic.


## Hot to build

Clone the repository and build it using `cargo`.


```shell
$ cargo build --release
```

This will build the `bbbasic`-Library and the `bbcli`-Commandline Interpreter.

## Using bbcli

You just pass it a filename containing the code. No options for now.

```shell
$ bbcli demos\hello.bbb
```

will produce

```shell
Hello, World!
```

## Ideas/Todos

* Expand featureset of bbbasic
* Transpile bbbasic to wasm

## License

MIT - See LICENSE-File for more.

