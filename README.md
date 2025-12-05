# Processing for Zed

Processing language support for Zed with completions and diagnostics driven by the Processing language server.

## Features

- Syntax highlighting, indentation, and bracket matching
- Completions, signature help, and diagnostics directly from Processing

## Requirements

1. Install [Processing 4.4.6 or newer](https://processing.org/download) and launch it once so the CLI is fully initialized.
2. Ensure the `processing` executable that ships with Processing 4.x is reachable from your `PATH`, exposed through one of the following environment variables, or configured explicitly via Zed settings:
   - `PROCESSING_PATH`
   - `PROCESSING_BIN`
   - `PROCESSING_CLI`
3. Zed 0.158.0 or newer (any release that can load extensions built with `zed_extension_api` 0.7.0).

If the executable lives outside `PATH`, add a workspace or global setting similar to:

```jsonc
// settings.json
{
  "lsp": {
    "processing-language-server": {
      "binary": {
        "path": "/Applications/Processing.app/Contents/MacOS/Processing",
        "arguments": [],
        "env": {
          "JAVA_HOME": "/Library/Java/JavaVirtualMachines/jdk-21.jdk/Contents/Home"
        }
      }
    }
  }
}
```

The `arguments` array is optional; the extension always appends the `lsp` subcommand.

## Running sketches

Zed's public extension API does not yet expose an API for adding editor run buttons or an integrated console surface. Once Zed surfaces a general task or runnable API for extensions, this project can hook into Processing's `cli --run/--export` commands to provide a run button similar to the official VS Code extension.

## Development

```bash
# build the WebAssembly module
cargo build --release --target wasm32-wasip2
```

The highlight queries in `languages/processing/highlights.scm` originate from the MIT-licensed [`tree-sitter-java`](https://github.com/tree-sitter/tree-sitter-java) project.
