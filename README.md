# Guia

Take a look at your favorite project documentation without leaving the terminal.
Guia is a simple command line tool that allows you to read offline documentation
sets.

## Install

### cURL

#### Requirements

- rustup;
- cargo;

```sh
curl -sSL https://raw.githubusercontent.com/thigcampos/guia/main/install.sh | bash -
```

### Homebrew (macOS-only)

Guia is now available on Homebrew as a Tap. You can install
it using the following command:

```sh
brew tap thigcampos/guia
```

```sh
brew install guia
```

## How to use

To list all documentation sets available in your system,
you can use the following command:

```sh
guia list
```

To display a documentation set, you can use the following command:

```sh
guia bun 
```

To download a supported documentation set, you can use the following command:

```sh
guia add bun
```

## Markdown renderer

Guia uses `less` as the default markdown renderer, since it's available
in widely used systems, with a few exceptions, most notably Windows.

If you want to use a different markdown renderer, you must assign a new value to `GUIA_MARKDOWN`.
You're free to choose any command-line tool or application that can handle markdown files, including,
but not limited to, `cat`, `vim`, `code`, `more`, and etc.

```sh
export GUIA_MARKDOWN="glow" # Recommended markdown renderer
```

> Disclaimer: [Glow](https://github.com/charmbracelet/glow) is an external dependency, so you need to install it manually.
It is not distributed or related with Guia.

## Documentation Sets

We had an important changes of plans regarding the documentation sets. We withdraw the goal of 
having a centralized index of documentation sets, since it would require a lot of 
effort to maintain it. Instead, we are focusing on the tool itself, and we are providing 
an utility tool to help you manage your documentation sets.

Check [docsets](https://github.com/thigcampos/docsets) for more information.

### Documentation sets location

All documentation sets are stored in the `docsets` directory. You may find
`docsets` under the folder `guia` in your OS local config directory.

- Linux/Redox: `$XDG_CONFIG_HOME` or `$HOME/.config`;
- macOS: `$HOME/Library/Application Support`;
- Windows: `{FOLDERID_LocalAppData}`;

`guia` reads `docsets` to display the available docs. As of now,
you may need to install desired documentation sets manually,
since we have just a few of them officially supported.

## License

Guia it's released under the MIT license. However, projects documentation
may not be under the MIT license, so please check the project's documentation
license before using it. See [LICENSE](LICENSE) for more information.
