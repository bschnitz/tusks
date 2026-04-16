# Structure

- `tusks-macro` defines the `tusks`-macro.
- `tusks-lib` is supposed to define all data structures which shall be used in
  `tusks-macro` itself and the code the `tusks`-macro generates (So `tusks`
  collects the reflected data and inserts it into the actual code to be usable
  by the cli builder)
- `tusks` reexports both of the other libs, such that it is easy to use them
  together in the actual program.
- `tusks-test` is just a test implementation, which uses the `tusks`-lib.
