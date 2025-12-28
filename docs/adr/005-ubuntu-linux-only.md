# ADR 005: Ubuntu-Based Linux Distribution Support Only

## Status

Accepted

## Date

2024-12-28

## Context

The cwrdd project requires a development environment with specific tooling including:
- Podman and podman-compose for containerization
- Liquibase for database migrations
- PostgreSQL client tools
- OpenSSL for TLS certificate generation
- System certificate trust management

Supporting multiple operating systems (Linux distributions, macOS, Windows) requires:
- Different package managers (apt, dnf, pacman, brew, chocolatey)
- Different certificate trust mechanisms
- Different container runtime configurations
- Testing across all platforms
- Maintaining platform-specific code paths

As a small project with limited resources, we need to focus our efforts where they matter most.

## Decision

We will only officially support Ubuntu-based Linux distributions (Ubuntu, Linux Mint, Pop!_OS, etc.) for development.

Specifically:
- The `cwrdd-make` build tool will only implement Ubuntu/Debian-specific commands
- Setup scripts target Ubuntu with apt package management
- Certificate trust uses `/usr/local/share/ca-certificates/` and `update-ca-certificates`
- Tool installation assumes apt and systemd availability
- Documentation will focus on Ubuntu setup instructions

## Consequences

### Positive

1. **Reduced complexity**: Single code path for all system-level operations
2. **Faster development**: No need to test across multiple platforms
3. **Clearer documentation**: One set of instructions to maintain
4. **Consistent developer experience**: All developers use the same environment
5. **Easier debugging**: Issues can be reproduced in a known environment

### Negative

1. **Limited contributor base**: Developers on macOS/Windows must use VMs or WSL2
2. **No macOS support**: Team members with Macs need Ubuntu VMs or dual-boot
3. **Potential future work**: If we need to support other platforms later, significant refactoring may be required

### Mitigations

- Developers on other platforms can use Ubuntu in a VM (recommended: multipass, VirtualBox)
- Windows users can use WSL2 with Ubuntu
- The core Rust application code remains cross-platform; only the build tooling is Ubuntu-specific
- Cloud-based development environments (GitHub Codespaces, Gitpod) can provide Ubuntu environments

## Notes

This decision can be revisited if:
- The team grows and includes developers who strongly prefer other platforms
- We receive significant community interest in supporting other distributions
- A cross-platform abstraction layer becomes available that simplifies multi-OS support
