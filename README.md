# Shared Types

This repository contains Rust types shared between multiple projects. It is intended to be used as a **git submodule** inside other repositories (consumer projects).

## Table of Contents

- [Adding this repo as a submodule](#adding-this-repo-as-a-submodule)
- [Making changes to shared types](#making-changes-to-shared-types)
- [Updating a consumer project with the latest shared types](#updating-a-consumer-project-with-the-latest-shared-types)
- [Cloning a consumer project with submodules](#cloning-a-consumer-project-with-submodules)
- [Common commands](#common-commands)
- [Troubleshooting](#troubleshooting)

---

## Adding this repo as a submodule

In each consumer project’s root directory, run:

```bash
git submodule add https://github.com/Rainymy/volumize-shared-type.git shared-types
```

Then add the path dependency to the consumer’s `Cargo.toml`:

```toml
[dependencies]
shared-types = { path = "./shared-types" }
```

Commit the submodule and the `Cargo.toml` change:

```bash
git add .gitmodules shared-types Cargo.toml
git commit -m "Add shared-types submodule"
```

---

## Making changes to shared types

1. **Edit the source files** inside the `shared-types` directory (from within the consumer project) or directly in this repository if you have it cloned separately.

2. **Test locally in the consumer project**  
   Because the consumer uses a path dependency, running `cargo check` or `cargo test` in the consumer will immediately compile and test your changes.  
   (Optional) You can also test inside this repository with `cargo test`.

3. **Commit and push your changes to this repository**  
   From inside the `shared-types` directory:

   ```bash
   git add .
   git commit -m "Describe your change"
   git push origin main   # or your branch
   ```

4. **Tell consumer projects to use the new commit** (see next section).

---

## Updating a consumer project with the latest shared types

After you have pushed new commits to this repository, each consumer project must be updated manually (unless you use a CI job). This is a two‑step process: update the submodule’s working tree to the latest commit, then commit the new pointer in the consumer.

### From the consumer project root:

1. **Fetch the latest commit and update the submodule**

   ```bash
   git submodule update --remote shared-types
   ```

   _Note:_ This command assumes the submodule tracks a branch (default is `main`). If not, you may need to `cd shared-types && git pull origin main` first.

2. **Verify the new version compiles**

   ```bash
   cargo check
   ```

3. **Commit the updated submodule pointer**
   ```bash
   git add shared-types
   git commit -m "Update shared-types to latest"
   git push
   ```

That’s it! The consumer project now references the new commit.

### If you want to update to a **specific** commit (not latest):

```bash
cd shared-types
git fetch
git checkout <commit-hash>
cd ..
git add shared-types
git commit -m "Pin shared-types to <commit-hash>"
```

---

## Cloning a consumer project with submodules

When someone clones the consumer project fresh, the submodule directory will be empty. They must initialize it:

```bash
git clone <consumer-repo-url>
cd <consumer-repo>
git submodule update --init --recursive
```

In GitHub Desktop, you will be prompted to initialize submodules when cloning; click **Initialize submodules**.

---

## Common commands

| Task                              | Command (run from consumer root)                           |
| --------------------------------- | ---------------------------------------------------------- |
| Update submodule to latest remote | `git submodule update --remote shared-types`               |
| Initialize submodule after clone  | `git submodule update --init --recursive`                  |
| See current submodule commit      | `git submodule status`                                     |
| Enter submodule and pull manually | `cd shared-types && git pull origin main && cd ..`         |
| Commit updated submodule pointer  | `git add shared-types && git commit -m "Update submodule"` |

---

## Troubleshooting

### `cargo check` fails with “failed to get `shared-types` as a dependency”

- Make sure the submodule is initialized: run `git submodule update --init --recursive`.
- Check that the `path` in `Cargo.toml` points to the correct folder (usually `./shared-types`).

### The submodule folder appears empty or has a different commit than expected

- Run `git submodule update --init` to sync the working tree to the commit recorded in the parent repository.
- If you want the latest, use `git submodule update --remote`.

### I made changes in the submodule but the consumer project doesn’t see them

- **Path dependencies** use the files directly from the submodule directory. If you edited files outside the submodule (or in a different location), that won’t work. Ensure you are editing inside the `shared-types` folder.
- Run `cargo clean` in the consumer if you suspect stale build artifacts.

### The submodule’s `Cargo.toml` has a hyphen in the library name

- The library target name cannot contain hyphens. In `shared-types/Cargo.toml`, remove any `[lib] name = "shared-types"` line, or change it to `"shared_types"`. The default (derived from package name) is already correct.

### I forgot to commit the submodule pointer after updating

- If you run `git status` in the consumer project and see `modified: shared-types (new commits)`, that means the pointer is stale. Commit it as described above.

---
