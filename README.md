# QLSyntaxHighlight

Syntax Highlighting/Plain Text [Quick Look](http://en.wikipedia.org/wiki/Quick_Look) plugin written in Rust

## Work in progress!
- tries to open most plaintext files
- no max file size
- no highlighting for natively supported filetypes
- doesn't generate thumbnails

### Installation

- Download from [Releases](https://github.com/Memorion/QLSyntaxHighlight/releases)
- Move the downloaded .qlgenerator file to `~/Library/QuickLook`
- Run `qlmanage -r`

### Configuration

Setting the font (default is `Menlo`):

    defaults write de.bastianrinsche.QLSyntaxHighlight fontFamily 'Menlo, monospace'

Setting the font size (default is `11`):

    defaults write de.bastianrinsche.QLSyntaxHighlight fontSize 11

Setting a theme (default is an [included xcode-like theme](qlhighlight/res/Xcodelike.tmTheme), set to empty string to go back):

    // Single quote for theme names with spaces, Double quote for spaces + parentheses
    defaults write de.bastianrinsche.QLSyntaxHighlight theme "'Solarized (dark)'" 

Available themes are the default theme and [these](https://docs.rs/syntect/2.0.0/syntect/highlighting/struct.ThemeSet.html#method.load_defaults).

Adding additional themes. Set to absolute path of directory containing `.tmTheme` files:

    defaults write de.bastianrinsche.QLSyntaxHighlight themeDirectory '/path/to/themes'
    
Adding addiontal syntaxes. Set to absolute path of directory containing `.sublime-syntax` files:

    defaults write de.bastianrinsche.QLSyntaxHighlight syntaxDirectory '/path/to/syntaxes'

![screenshot](img/screenshot.png)
