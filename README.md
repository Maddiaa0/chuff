# Chuff!

> `Chuff` is a huff-language parser with a focus on error recovery.

> **Warning**
> Chuff is a learning resource for myself and is not suited for production use. Go for it, but it probably won't do what you want it to.

## Wat do?

Chuff is a huff parser built with [chumsky]("https://github.com/zesterer/chumsky"). A "parser library for humans with powerful error recovery". Using chumsky as a base reduces the effort to build a parser that can recover from errors, which is essential to provide as much feedback as possible within an lsp. 

## Why

### lsp

This software is being written to start the effort in creating a functional [lsp]("https://microsoft.github.io/language-server-protocol/") for huff. Huff is a _hard_ language to learn as there are so many footguns. Creating an LSP aims to provide feedback to people learning that will reduce the pain of it all.

## TODO

- [ ] Complete Parsing Mvp
  - [ ] Support Abi tuples
  - [ ] Parser validation on length of static types
- [ ] File based span support
- [ ] Tests
- [ ] Document each function
- [ ] Create example folder with an example of rich error feedback
- [ ] Create sister project for ast verification
