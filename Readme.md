# Language server protocol (LSP) support for Zed

## Setup

Assumes ast-grep is available globally. You must install it yourself and either make it available on the `$PATH` or manually override the binary path (see below).

## Using a non default installation of ast-grep

If you cannot or do not want to have ast-grep globally installed on your system, you must manually specify the path to the `ast-grep` executable. Note that an absolute path is necessary and placeholders (ex. `~`, `$HOME`) are not expanded.

Example:

```json
{
  "lsp": {
    "ast-grep": {
      "binary": {
        "path": "/Users/user-name/.nix-profile/bin/ast-grep"
      }
    }
  }
}
```

## Passing the `sgconfig.yml` path to ast-grep

If your `sgconfig.yml` is not at the project root, you must manually specify it's location in Zed's project specific settings (normally in `.zed/settings.json`). You can do so by overriding the command line arguments sent to ast-grep but **don't forget the lsp argument**. Note that an absolute path is necessary and placeholders (ex. `~`, `$HOME`) are not expanded.

```json
{
  "lsp": {
    "ast-grep": {
      "binary": {
        "arguments": [
          "lsp",
          "-c",
          "/Users/user-name/project-name/some-subfolder/sgconfig.yml"
        ]
      }
    }
  }
}
```
