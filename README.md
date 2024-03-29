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

Add every object you want to reuse to the `globals` section.

```
{
    "globals": {
        "numbers": {
            "one": 1,
            "two": 2,
            "three": 3
        }
    },
    "obj1": {
        "a": "@{numbers}",
    },
    "obj2": {
        "b": "@{numbers}",
    }
}
```

Run it on your template:<br>
`vscode-workspace-gen vscode.code-workspace.template`

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


![Build](https://github.com/iamsergio/vscode-workspace-gen/actions/workflows/tests.yml/badge.svg)</br>
![Build](https://github.com/iamsergio/vscode-workspace-gen/actions/workflows/sanitizers.yml/badge.svg)</br>
![Build](https://github.com/iamsergio/vscode-workspace-gen/actions/workflows/lints.yml/badge.svg)
</br>
+[![No Maintenance Intended](http://unmaintained.tech/badge.svg)](http://unmaintained.tech/)
