### pre-commit

[pre-commit](https://pre-commit.com/) manages git hooks defined in `.pre-commit-config.yaml` at the project root. Hooks run automatically on commit.

### VSCode MCP Server

The VS Code MCP Server extension provides file operations, symbol search, hover info, diagnostics, shell execution, and line replacements.

### Bifrost MCP

Available via the VSCode MCP for refactoring support. To start:

```powershell
npx -y supergateway --sse http://localhost:8008/sse --port 8000
```

See <https://github.com/biegehydra/BifrostMCP>
