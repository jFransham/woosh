# WOOSH: WOnky Optimising SHell

A POSIX-compatible shell that exists to ideally run as ridiculously fast as
possible, and not designed for interactive use. Supposed to be a drop-in
replacement for Dash, so it should have feature- and misfeature-parity with
that, within reason.

Yeah, I know, who would use a shell not designed for interactive use. I wish the
answer was no-one.

Ideas to make it run fast:

Compile to bytecode (almost certainly going to happen):

- Zero-copy parsing using `tendril` (currently unimplemented, everything is
  copied to the heap)
- Pipes implemented as (in LLVM IR-ish pseudocode):
  ```sh
  ls -clt | grep some_file
  ```
  ```llvm
  %process1 = spawnproc "ls" ["-clt"]
  %process2 = spawnproc "grep" ["some_file"]
  connectfd [%process1 1] to [%process2 0]
  ```
- No defined order for spawning processes in pipes (does that matter?).
- Mark some things as "pure" so results can be cached.
- Aggressive const-folding and symbolic execution (including const-folding of
  `eval`)
- Cache build results in Super Secret Hidden Folder (assumes that most scripts
  don't use `eval`, since we can't cache that)
- Store numbers as numbers so that we don't have to marshal between different
  datatypes
- Use coreutils crate to supply in-tree implementations of many tools. Likely
  not important on its own, but we can mark some as pure and include them in
  with const folding. Plus it could probably make piping significantly more
  efficient
