I had the same issue. This is how I resolved it.

My worktree looks like this:

```
.vscode
└── settings.json
src
└── challenges
    └── some modules...
    lib.rs
    main.rs
Cargo.lock
Cargo.toml
```

The `settings.json` contain this data:

```
{
  "rust-analyzer.linkedProjects": [
    "./challenges/Cargo.toml"
  ]
}
```

AS you can see, I have no `./challenges/Cargo.toml`, that's because I created that folder as a library using `cargo new --lib` and then I deleted it. That's how I think it got that path. 

I just had to change it for `rust-analyzer` start working again

```
{
  "rust-analyzer.linkedProjects": [
    "./Cargo.toml"
  ]
}
```