# Language server protocol (LSP) support for Zed

## Setup

Assumes ast-grep is available globally. You must [install it yourself](https://ast-grep.github.io/guide/quick-start.html#installation) and either make it available on the `$PATH` or manually override the binary path (see below).

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

## Language Support

- [list of ast-grep supported languages][ast-grep-langs]
- [list of this extension supported languages][ext-langs]

[ext-langs]: https://github.com/mathieulj/ast-grep-zed/blob/main/extension.toml#L11
[ast-grep-langs]: https://ast-grep.github.io/guide/introduction#supported-languages

### Toml

`Ast-grep` doesn't support `toml`, but it's easy to add it following the [instruction for custom languages][custom-language].
Tested with [tree-sitter-grammars/tree-sitter-toml](https://github.com/tree-sitter-grammars/tree-sitter-toml).

[custom-language]: https://ast-grep.github.io/advanced/custom-language.html#custom-language-support
