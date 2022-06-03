# `huffup`

Update or revert to a specific Huff branch with ease.

Forked from [foundry](https://github.com/foundry-rs/foundry/tree/master/foundryup).

### Installing

`curl -L <huff-install-bin-url> | bash`

## Usage

To install the **nightly** version:

```sh
huffup
```

To install a specific **version** (in this case the `nightly` version):

```sh
huffup --version nightly
```

To install a specific **branch** (in this case the `release/0.1.0` branch's latest commit):

```sh
huffup --branch release/0.1.0
```

To install a **fork's main branch** (in this case `abigger87/huff-rs`'s main branch):

```sh
huffup --repo abigger87/huff-rs
```

To install a **specific branch in a fork** (in this case the `patch-10` branch's latest commit in `abigger87/huff-rs`):

```sh
huffup --repo abigger87/huff-rs --branch patch-10
```

To install from a **specific Pull Request**:

```sh
huffup --pr 1071
```

To install from a **specific commit**:
```sh
huffup -C 94bfdb2
```

To install a local directory or repository (e.g. one located at `~/git/huff`, assuming you're in the home directory)
##### Note: --branch, --repo, and --version flags are ignored during local installations.

```sh
huffup --path ./git/huff
```

---

**Tip**: All flags have a single character shorthand equivalent! You can use `-v` instead of `--version`, etc.

---