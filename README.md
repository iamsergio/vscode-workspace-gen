# vscode-workspace-gen

A cli tool that generates vscode workspace files from a template.<br>

The goal is to workaround https://github.com/microsoft/vscode/issues/53557 .

vscode settings and workspace files suffer from huge json duplication. With this tool you can,
specify json objects you want to reuse and then reference them instead of duplicating.

TODO: Add example

![Build](https://github.com/iamsergio/vscode-workspace-gen/actions/workflows/tests.yml/badge.svg)
![Build](https://github.com/iamsergio/vscode-workspace-gen/actions/workflows/sanitizers.yml/badge.svg)
![Build](https://github.com/iamsergio/vscode-workspace-gen/actions/workflows/lints.yml/badge.svg)
</br></br>
+[![No Maintenance Intended](http://unmaintained.tech/badge.svg)](http://unmaintained.tech/)