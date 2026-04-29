my-cli/
├── Cargo.toml
├── Cargo.lock
├── README.md
├── .github/
│ └── workflows/
│ └── ci.yml
│
src/
├── main.rs # Entrypoint: parse args, call run(), handle top-level errors
├── lib.rs # Re-exports; allows integration testing of internals
│
├── cli/ # Everything clap-related (pure CLI surface)
│ ├── mod.rs # Defines root Cli struct and top-level Args
│ ├── args.rs # Shared/global flags (--verbose, --config, --output-format)
│ └── commands/
│ ├── mod.rs # Commands enum that delegates to submodules
│ ├── foo.rs # `FooArgs` struct + `FooCommand` enum (subcommands of foo)
│ └── bar.rs
│
├── commands/ # Business logic handlers, one module per command
│ ├── mod.rs
│ ├── foo.rs # `pub fn run(args: &FooArgs, ctx: &AppContext) -> Result<()>`
│ └── bar.rs
│
├── config/ # Config file loading & merging (file + env + flags)
│ ├── mod.rs
│ └── schema.rs # The strongly-typed Config struct (serde)
│
├── domain/ # Your core data model, completely CLI-agnostic
│ ├── mod.rs
│ ├── project.rs
│ └── user.rs
│
├── services/ # Business logic / use-cases, depend only on domain + infra traits
│ ├── mod.rs
│ └── project_service.rs
│
├── infra/ # I/O implementations (HTTP, DB, filesystem)
│ ├── mod.rs
│ ├── http.rs
│ └── fs.rs
│
├── output/ # Rendering layer: table, JSON, plain text, etc.
│ ├── mod.rs
│ └── table.rs
│
├── errors.rs # App-wide error type (thiserror) + Result alias
└── context.rs # AppContext: config + shared clients passed to every command