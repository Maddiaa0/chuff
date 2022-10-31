# Mard!

> `Mard` is a huff-language parser with a focus on error recovery at the parseing stages. It's purpose is to produce a partial AST that can be used to grant rich error feedback.

> **Warning**
> Mard is currently under active development and is not suited for production use. Go for it, but it probably won't do what you want it to.

## Wat do?

Mard is a huff parser build with [chumsky]("https://github.com/zesterer/chumsky"). A "parser library for humans with powerful error recovery". Using chumsky as a base reduces the effort to build a parser that can recover from errors and reproduce a functional AST, which is essential if you want to show errors across the whole file.

## Why

### lsp

This software is being written to start the effort in creating a functional [lsp]("https://microsoft.github.io/language-server-protocol/") for huff. Huff is a _hard_ language to learn as there are so many footguns. Creating an LSP aims to provide feedback to people learning that will reduce the pain of these footguns.
