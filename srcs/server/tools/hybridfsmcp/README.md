# Hybrid File System MCP Proxy

This directory contains the integration of a Hybrid File System into the OHC OS via the Model Context Protocol (MCP).

## Architecture

- `FileSystemProvider`: The core interface abstracting file operations.
- `LocalFSProvider`: Binds file operations for Standalone mode with local host machine safety bounds.
- `CloudFSProvider`: Uses tenant-scoped persistent volumes to allow strict multi-tenant isolation based on Context Claims.
- `HybridFSMCP`: The Model Context Protocol adapter for exposing standardized `filesystem` tools (`read_file`, `write_file`, `list_directory`, `search_files`).

## Provider Selection
Providers are returned based on the `OHC_STANDALONE` environment variable to bridge the K8s and Desktop environments automatically.
