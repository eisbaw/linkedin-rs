---
name: linkedin-pii-scan
description: "LinkedIn-specific PII scanner. Scans tracked files for LinkedIn profile URNs, member IDs, session cookies, real names in JSON, vanity URLs, and message URNs. Run before pushing."
allowed-tools: Bash, Read, Grep
user-invocable: true
version: 1.0.0
---

# LinkedIn PII Scan

Scans tracked git files for LinkedIn-specific PII that generic secret scanners miss.

## Usage

```bash
# Scan all tracked files
./scripts/linkedin-pii-scan.sh

# Scan only staged/unstaged changes (for pre-commit)
./scripts/linkedin-pii-scan.sh --diff
```

## What it detects

| Severity | Pattern | Example |
|----------|---------|---------|
| CRITICAL | `li_at` session cookie values | `AQE...{200+ chars}` |
| HIGH | LinkedIn profile URNs | `urn:li:fsd_profile:ACoAA...` |
| HIGH | Member numeric IDs | `member:123456789` |
| HIGH | JSESSIONID values | `ajax:0000000000000000001` |
| HIGH | Real names in JSON | `"firstName": "Jane"` |
| HIGH | Public identifiers | `"publicIdentifier": "john-doe"` |
| MEDIUM | Message/conversation URNs | `urn:li:msg_conversation:...` |
| MEDIUM | Real occupations in JSON | `"occupation": "Engineer at ..."` |

## Exclusions

- Skips `secrets/`, `extracted/`, `decompiled/`, `target/`, `har/`
- Skips `.rs` source code for patterns that appear in string formatting/construction
- Skips lines containing `test`, `example`, `placeholder`, `mock`
