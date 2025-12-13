# JOBL

A TOML-encoded resume file format with strict validation.

## Format Specification

JOBL (v20251211) is a structured resume format using TOML encoding.

### File Extension

`.jobl`

### Top-Level Keys

- `jobl_version` (required): Version string in `YYYYMMDD` format
- `person` (required): Personal information table
- `skills` (optional): Skills organized by category
- `experience` (optional): Array of work experience
- `projects` (optional): Array of projects
- `education` (optional): Array of education entries

### Person Section

Required:
- `name`: Full name

Optional:
- `headline`: Professional headline
- `location`: Current location
- `email`: Contact email
- `website`: Personal website URL
- `phone`: Contact phone
- `summary`: Professional summary

### Skills Section

A table mapping category names to arrays of skill strings.

Example:
```toml
[skills]
languages = ['Rust', 'Python', 'JavaScript']
devops = ['Docker', 'Kubernetes']
```

### Experience Items

Required:
- `title`: Job title
- `company`: Company name

Optional:
- `location`: Job location
- `start`: Start date (recommend 'YYYY-MM')
- `end`: End date or 'present'
- `summary`: Role summary
- `technologies`: Array of technologies used
- `highlights`: Array of achievement highlights

### Project Items

Required:
- `name`: Project name

Optional:
- `url`: Project URL
- `summary`: Project description
- `role`: Your role
- `start`: Start date
- `end`: End date
- `technologies`: Array of technologies

### Education Items

Required:
- `institution`: School or institution name
- `degree`: Degree or certification

Optional:
- `location`: Institution location
- `start`: Start date
- `end`: End date
- `details`: Array of additional details

## Usage

```rust
use jobl::{parse_str, parse_file};

// Parse from string
let doc = parse_str(jobl_content)?;

// Parse from file
let doc = parse_file("resume.jobl")?;

// Access data
println!("Name: {}", doc.person.name);
```

## Validation

The parser performs strict validation:

- Version must be '20251211'
- Unknown top-level keys are errors
- Unknown fields in sections are errors
- Type mismatches are errors
- Multiple errors are collected and reported

Errors include path context:
- `person.name`: Missing required field
- `experience[0].unknown`: Unknown field in first experience item
- `skills.languages`: Invalid type

## License

MIT
