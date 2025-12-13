+++
title = "JOBL"
template = "index.html"
+++

# JOBL

A TOML-based resume format.

## What is JOBL?

JOBL is a simple, human-readable format for resumes. Write your resume in TOML, generate HTML and PDF with SRG.

## Format

- TOML-based
- Strict validation
- Clear error messages
- No version field required

## Tools

- [SRG](https://github.com/ducks/SRG) - Static Resume Generator
- [hostedresumes.com](https://hostedresumes.com) - Hosting service

## Resources

- [Specification](/spec)
- [GitHub](https://github.com/ducks/JOBL)
- [Example Resume](https://github.com/ducks/JOBL/blob/main/examples/resume.jobl)

## Quick Example

```toml
[person]
name = "Jane Developer"
email = "jane@example.com"
headline = "Senior Software Engineer"

[skills]
languages = ["Rust", "Python", "Go"]
tools = ["Docker", "Kubernetes"]

[[experience]]
title = "Senior Engineer"
company = "Tech Corp"
start = "2020-01"
end = "present"
highlights = [
  "Built distributed systems",
  "Led team of 5 engineers"
]
```
