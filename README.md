# gptfeed - Output files in a specific format for LLM consumption

A command-line utility that formats files for easy consumption by Large Language Models (LLMs). It wraps file contents in customizable container tags with appropriate comment syntax for different file types.

## Usage

```
gptfeed [OPTIONS] [FILES]...
```

If no files are provided, or if "-" is specified as a filename, the tool will read from stdin.

## Options

- `-c, --container <CONTAINER>`: Specify the container tag (default: "code")
- `-m, --comment-prefix <COMMENT_PREFIX>`: Set a custom comment prefix (default: auto-detect based on file extension)

## Sample Output

The filename in the output makes it easy for LLMs to differentiate where one file ends and another begins.

```
<code>
// main.ts
const greet = () => {
    console.log("Hello world!");
};

# script.py
def hello():
    print("Hello world!")
</code>
```

By default, the comment style is automatically determined based on the file extension (`//` for JavaScript, `#` for Python, etc.).

This can be overridden with the `--comment-prefix` option, which will be applied for all files passed to the command.

The `<code>` XML tag is used as a container for the file contents. This can be customized with the `--container` option, e.g.:

```
gptfeed --container hello --comment-prefix '#' main.ts

<hello>
# main.ts
const greet = () => {
  console.log("Hello, world!");
};
</hello>
```

## Examples

### Reading from stdin

You can pipe content directly to gptfeed:

```
cat file.js | gptfeed
```

Or:

```
echo "console.log('Hello');" | gptfeed
```

When reading from stdin, no filename comment header is printed, only the content is included.

You can also use the "-" placeholder to mix stdin with files:

```
cat file.js | gptfeed - other_file.py
```

### Pipe to Clipboard on macOS

You can easily pipe the output of gptfeed directly to your clipboard using pbcopy:

```
gptfeed path/to/your/file.js | pbcopy
```

This copies the formatted file content to your clipboard, ready to be pasted into your LLM interface.

### Processing Multiple Files

Process multiple files at once and pipe to clipboard:

```
gptfeed main.js script.py | pbcopy
```

## License

This project is licensed under either of

 * ISC license ([LICENSE](LICENSE) or
   https://opensource.org/licenses/ISC)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   https://opensource.org/licenses/MIT)
 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   https://www.apache.org/licenses/LICENSE-2.0)
