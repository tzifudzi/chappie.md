### pre-commit

[pre-commit](https://pre-commit.com/) manages git hooks defined in `.pre-commit-config.yaml` at the project root. Hooks run automatically on commit.

### Bifrost MCP

When using VS Code, typical workflow is to use Bifrost MCP, a VS Code extension that exposes language server features (go-to-definition, find usages, rename, completions, etc.) to AI coding assistants over HTTP/SSE. A bifrost.config.json in the project root assigns this workspace a dedicated port (e.g. 8080) so multiple VS Code windows can run Bifrost simultaneously without port conflicts. The Roo Code MCP client is configured in .roo/mcp.json to connect at <http://localhost:8009/chappie-cli/sse>. Install the Bifrost extension, open the project, and it starts automatically — no manual server launch required.

See <https://github.com/biegehydra/BifrostMCP>
