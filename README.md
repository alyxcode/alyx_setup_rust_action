# Alyx `setup-rust` GitHub Action

[![CI](https://img.shields.io/github/workflow/status/alyxcode/alyx_setup_rust_action/default/default?color=5e81ac&label=CI&logo=github&style=for-the-badge)](https://github.com/alyxcode/alyx_setup_rust_action/actions)

This action sets up [Rust](https://www.rust-lang.org/) environment for use in [actions](https://github.com/features/actions).

## Usage

```yaml
on:
  push:
    branches: [ default ]
jobs:
  rust:
    strategy:
      matrix:
        os: [ubuntu-latest, macos-latest, windows-latest]
        toolchain: [stable, beta, nightly]
    runs-on: ${{ matrix.os }}
    steps:
      - uses: nisquy/setup-rust@default
        with: 
          toolchain: ${{ matrix.toolchain }}
          targets: 'x86_64-unknown-linux-gnu, x86_64-pc-windows-msvc, x86_64-apple-darwin'
          components: 'rustfmt, clippy'
          profile: 'minimal'
          default: true
          downgrade: true
```

## Inputs

| Name         | Description                                                                                                                                    | Type   | Default |
| ------------ | -----------------------------------------------------------------------------------------------------------------------------------------------| ------ | --------|
| `components` | [Components](https://rust-lang.github.io/rustup/concepts/components.html)                                                                      | string |         |
| `default`    | Set toolchain as [default](https://rust-lang.github.io/rustup/overrides.html#default-toolchain)                                                | bool   | true    |
| `downgrade`  | Allow [nightly toolchain downgrade](https://rust-lang.github.io/rustup/installation/index.html#installing-nightly)                             | bool   | true    |
| `profile`    | [Profile](https://rust-lang.github.io/rustup/concepts/profiles.html)                                                                           | string | minimal |
| `toolchain`  | [Toolchain](https://rust-lang.github.io/rustup/concepts/channels.html) or [Channel](https://rust-lang.github.io/rustup/concepts/channels.html) | string | stable  |
| `target`     | [Targets](https://doc.rust-lang.org/nightly/rustc/platform-support.html)                                                                       | string |         |

## License

[![License](https://img.shields.io/badge/license-mit-81a1c1?style=for-the-badge)](LICENSE)