# Git Providers

This crate creates an abstraction over different Git providers, allowing Agin CI to interact with them in a unified way. Only a subset of options and response fields is supprted, as this crate is meant to be used internally by Agin CI.

> [!NOTE]
> This crate does not aim to be a full-featured Git provider client. It is designed to provide the minimum necessary functionality for Agin CI. If you miss a feature, contributions are welcome!

## Supported Providers
- [x] GitHub
- [x] Gitea / Forgejo / Gogs
- [ ] GitLab

*We don't plan to support other Git providers for now.*
