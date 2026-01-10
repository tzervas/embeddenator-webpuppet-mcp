# Changelog

All notable changes to webpuppet-mcp will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0-alpha.4] - 2025-01-22

### Changed
- **BREAKING**: Renamed crate from `embeddenator-webpuppet-mcp` to `webpuppet-mcp`
- Updated dependency from `embeddenator-webpuppet` to `webpuppet` (published on crates.io)
- Simplified all references from embeddenator naming to cleaner webpuppet naming

## [0.1.0-alpha.3] - 2025-01-19

### Added
- Initial MCP server implementation
- JSON-RPC 2.0 over stdio transport
- Tool exposure for webpuppet functionality:
  - `webpuppet_prompt` - Send prompts through browser automation
  - `webpuppet_screenshot` - Take screenshots
  - `webpuppet_list_providers` - List available AI providers
  - `webpuppet_provider_capabilities` - Get provider capabilities
  - `webpuppet_detect_browsers` - Detect installed browsers
  - `webpuppet_check_permission` - Permission checking
  - `webpuppet_intervention_*` - Human intervention controls
- Integration with webpuppet's security guardrails
- Response screening for prompt injections

[Unreleased]: https://github.com/tzervas/webpuppet-rs-mcp/compare/v0.1.0-alpha.4...HEAD
[0.1.0-alpha.4]: https://github.com/tzervas/webpuppet-rs-mcp/compare/v0.1.0-alpha.3...v0.1.0-alpha.4
[0.1.0-alpha.3]: https://github.com/tzervas/webpuppet-rs-mcp/releases/tag/v0.1.0-alpha.3
