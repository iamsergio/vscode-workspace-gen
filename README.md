# vscode-workspace-gen

A cli tool for generating vscode workspace files from a template.<br>

The goal is to workaround https://github.com/microsoft/vscode/issues/53557

vscode settings and workspace files suffer from huge json duplication. With this tool you can
specify json objects you want to reuse and then reference them instead of duplicating.

## Installation

`cargo install vscode-workspace-gen`

## Usage:

A real scenario could be factoring-out duplicated vscode launches, pretty printers and source map settings.<br>
For the purpose of this example we'll just show some dummy JSON instead.

Add every object you want to reuse to the `gen.globals` section.

```
{
    "gen.globals": {
        "numbers": {
            "one": 1,
            "two": 2,
            "three": 3
        }
    },
    "obj1": {
        "a": "@{numbers}"
    },
    "obj2": {
        "b": "@{numbers}"
    }
}
```

Run it on your template:<br>
`vscode-workspace-gen -t vscode.code-workspace.template`

and get:

```
{
    "obj1": {
        "a": {
          "one": 1,
          "two": 2,
          "three": 3
        }
    },
    "obj2": {
        "b": {
          "one": 1,
          "two": 2,
          "three": 3
        }
    }
}
```


## Syntax

### gen.description

You can add descriptions in your template objects. They won't be present in the output.


```
"obj1": {
    "gen.description" : "Some field with some purpose"
}
```

### inline expansion and nested expansion

Inline expansion, `@@{foo}` will expand the contents of `foo` into the parent object.<br>
Nested expansion, `@{foo}` is similar, but won't discard `foo`'s actual container, and will nest it.

For example, given `"list" : [10, 20, 30]`

The following template:

```
    "obj": {
        "somelist1" : [1, 2, 3, "@{list}"],
        "somelist2" : [1, 2, 3, "@@{list}"]
    }
```

results in:
```
    "obj": {
        "somelist1" : [1, 2, 3, [10, 20, 30]],
        "somelist2" : [1, 2, 3, 10, 20, 30]
    }
```

### gen.os

You can make certain objects only available on specific operating systems.
```
 "obj": {
    "gen.os" : [ "windows, "linux" ]
 }
 ```

If run on `macos`, the above object won't be included in the output.

### config

You can create a `.vscode-workspace-gen.json` file and change some settings.
Currently supported settings:

```
{
    "json_indent": 2,
    "output_filename": "vscode.code-workspace",
    "per_os_output_filenames": {
        "linux": "linux.code-workspace",
        "windows": "windows.code-workspace",
        "macos": "macos.code-workspace"
    }
}
```
- `json_indent` Specifies the amount of indentation for the JSON output
- `output_filename` Equivalent to passing `-c <output_filename>`. The commandline has priority though.
- `per_os_output_filenames` Allows to generate output for each operating system. Each file is potentially different, due to usage of `gen.os`
This option is incompatible with `output_filename`.

## Env var replacing

Since vscode won't replace `${env_var}` everywhere, we support replacing env vars as well, but in a more consistent manner.\n
Any occurrence of `$${env_var}` will be replaced with the env var's contents. This workarounds msvc launcher supporting expanding
`${env}` while the LLDB ones not.


## Convenience for Qt

If you passed `--features qt` to `cargo install`, you have some convenience options regarding Qt.
For now, it adds:
- `--download_qtnatvis` Downloads the `qt6.natvis`, which contains pretty printers for debugging Qt.
- `--create-default-vscode-workspace` Creates a vscode template suitable for Qt development.

![Build](https://github.com/iamsergio/vscode-workspace-gen/actions/workflows/tests.yml/badge.svg)</br>
![Build](https://github.com/iamsergio/vscode-workspace-gen/actions/workflows/sanitizers.yml/badge.svg)</br>
![Build](https://github.com/iamsergio/vscode-workspace-gen/actions/workflows/lints.yml/badge.svg)
</br>
[![No Maintenance Intended](http://unmaintained.tech/badge.svg)](http://unmaintained.tech/) Please fork and fix your issues.
