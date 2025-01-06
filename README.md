# Guia

Take a look at your favorite project documentation without leaving the terminal.
Guia is a simple command line tool that allows you to read offline documentation
sets.

## How to use

To display a documentation set, you can use the following command:

```sh
guia bun 
```

To download a supported documentation set, you can use the following command:

```sh
guia add bun
```

## Documentation Sets

### Supported documentation sets

- Bun

### Stores

All documentation sets are stored in the `docsets` directory. You may find
`docsets` under the folder `guia` in your OS local config directory.

- Linux/Redox: `Some($XDG_CONFIG_HOME)` or `Some($HOME/.config)`;
- macOS: `Some($HOME/Library/Application Support)`;
- Windows: `Some({FOLDERID_LocalAppData})`;

`guia` reads `docsets` to display the available docs. As of now,
you may need to install desired documentation sets manually,
since we have just a few of them officially supported.

## License

Guia it's released under the MIT license. See [LICENSE](LICENSE) for more information.
However, projects documentation may not be under the MIT license, so please
check the project's documentation license before using it.
