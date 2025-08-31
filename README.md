<p align="center">
  <img src="web/public/logo.svg" alt="Agin CI Logo" width="200"/>
</p>

<p align="center">
    <a href="LICENSE"><img src="https://img.shields.io/badge/License-GPLv3-blue.svg" alt="License"></a>
    <img src="https://img.shields.io/badge/made%20with-Rust-orange" alt="Made with Rust">
</p>

**Agin CI** is a modern, flexible, and self-hostable CI/CD platform designed to work with your favorite Git providers.

> [!WARNING]
> This project is currently in active development. Not all features may be fully implemented or stable.

## Features

- **Self-Hosted:** Take full control of your CI/CD infrastructure.
- **Broad Git Provider Support:** Works seamlessly with GitHub, Gitea, Forgejo, and other Git providers (soon!).
- **YAML-based Workflows:** Define your pipelines with an intuitive and familiar YAML syntax. Get autocompletions in your IDE.
- **OIDC Integration:** Secure your instance with OpenID Connect for authentication.
- **Scalable Architecture:** Built on top of Apache Pulsar and Redis for a scalable and resilient system.
- **Local:** Debug your workflows locally for faster iteration speed.

## Getting Started

To get started with self-hosting Agin CI, please refer to our official documentation for detailed installation and configuration instructions.

- [Server Installation & Configuration](https://agin.ci/docs/platform/server/installation)
- [Repository Setup](https://agin.ci/docs/platform/repo-setup)

## Example Workflow

Here is a simple example of an Agin CI workflow file:

```yaml
name: Build
on:
  push: {}
jobs:
  build:
    runs-on: linux
    base-image: node:latest
    name: Build Node Project
    steps:
      - uses: aginci/run
        run: npm run build
```

## Supported Git Providers

Agin CI offers first-class support for the following Git providers, including automatic webhook setup for builds and status reporting:

- **GitHub**
- **Gitea / Forgejo**

More providers support coming soon!

## Contributing

We welcome contributions! If you'd like to contribute, please feel free to open an issue or submit a pull request.

## License

Agin CI is open-source software licensed under the [GPL-3.0 license](LICENSE).
