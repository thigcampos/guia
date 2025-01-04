# Guia

Take a look at your favorite project documentation without leaving the terminal.
Guia is a simple command line tool that allows you to read offline documentation
sets.

## Documentation Sets

All documentation sets are stored in the `docsets` directory. You may find
`docsets` under the folder `guia` in your OS local config directory.

Value on Linux/Redox                                                   | Value on Windows                  | Value on macOS                              |
| ---------------------------------------------------------------------- |-----------------------------------| ------------------------------------------- |
| `Some($XDG_CONFIG_HOME)`        or `Some($HOME`/.config`)`             | `Some({FOLDERID_LocalAppData})`   | `Some($HOME`/Library/Application Support`)` |

As of now, you will need to install desired documentation sets manually, but
we are working on a feature to allow you to install documentation sets from
web using `guia`.

## License

Guia it's released under the MIT license. See [LICENSE](LICENSE) for more information.
However, projects documentation may not be under the MIT license, so please
check the project's documentation license before using it.
