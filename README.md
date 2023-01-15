# umbara - clone detection

Detect code clones, copies, and instances of plagiarism. Methods are based on the paper ["Winnowing: Local Algorithms for Document Fingerprinting"](https://theory.stanford.edu/~aiken/publications/papers/sigmod03.pdf)

Usage: after compiling, run  
```sh
$ umbara [language] [directory] [file 1] [file 2]
```

You will see the files output to the terminal, with sections detected as copies highlighted by ">>>" and "<<<".

Some Features not yet implemented:  
- [ ] support for more than two files at once
- [ ] support for boilerplate
- [ ] support for reference files to check against 
- [ ] configuration options/files
- [ ] html output
- [ ] robust winnowing as described in the paper. (currently it is local winnowing)
- [ ] fancy terminal output

some credits:
* skmendez for the crate [tree-sitter-traversal](https://github.com/skmendez/tree-sitter-traversal). Though I did not use it in my library, my own traversal code is heavily based on it
* blingenf for the project [copydetect](https://github.com/blingenf/copydetect). Reading through how someone else implemented a plagiarism detector was very help in 
understanding where I should start 
