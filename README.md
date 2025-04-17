# gptfeed - Output files in a specific format for LLM consumption

A command-line utility that formats files for easy consumption by Large Language Models (LLMs). It wraps file contents in customizable container tags with appropriate comment syntax for different file types.

## Usage

```
gptfeed [OPTIONS] [FILES]...
```

## Options

- `-c, --container <CONTAINER>`: Specify the container tag (default: "code")
- `-m, --comment-char <COMMENT_CHAR>`: Set a custom comment character (default: auto-detect based on file extension)

## License

This project is licensed under either of

 * ISC license ([LICENSE](LICENSE) or
   https://opensource.org/licenses/ISC)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   https://opensource.org/licenses/MIT)
 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   https://www.apache.org/licenses/LICENSE-2.0)
