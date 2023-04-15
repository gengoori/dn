This repository is for toys to play with natural deduction proof systems and hopefully learn about them.

In `dnlib` you will find a library for reading and checking proofs written as in the INF402 course of Université Grenoble Alpes[^1].
In `dnreader` there is a <abbr title="Command Line Interface">CLI</abbr> program to read and check a proof from a file, using  the `dn-lib`.

# Goals
- [ ] Start a library for propositionnal logic that can
    - [x] Parse a proof as described in [^1] (Section 3.1) and in [the `dnlib`'s README](./dnlib/README.MD)
    - [x] Check the parsed proof
    - [ ] Report many meaningful errors, like compiler could do
- [x] Write a short CLI for reading proofs from a file, with nice error reports.
- [ ] Write a REPL for writing proofs record by record
- [ ] Further improvement
    - [ ] Lighten proof syntax, removing the boilerplate justifications.
    - [ ] Allow comments in proofs.
    - [ ] Introduce first order logic.
    - [ ] Allow (semi?)-automatic proofs.
        - [ ] In propositional logic
        - [ ] In first-order logic

[^1]: [A link to the INF402 course of Université Grenoble Alpes](https://wackb.gricad-pages.univ-grenoble-alpes.fr/inf402/Poly-inf402.pdf)
