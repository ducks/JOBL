# JOBL Format Specification

A TOML-encoded resume file format with strict validation.

## File Extension

`.jobl`

## Top-Level Keys

- `person` (required): Personal information table
- `skills` (optional): Skills organized by category
- `experience` (optional): Array of work experience
- `projects` (optional): Array of projects
- `education` (optional): Array of education entries

Unknown top-level keys are errors.

## Person Section

**Required:**
- `name`: Full name

**Optional:**
- `headline`: Professional headline
- `location`: Current location
- `email`: Contact email
- `website`: Personal website URL
- `phone`: Contact phone
- `summary`: Professional summary

Unknown fields in the person section are errors.

## Skills Section

A table mapping category names to arrays of skill strings.

Example:
```toml
[skills]
languages = ["Rust", "Python", "JavaScript"]
devops = ["Docker", "Kubernetes"]
```

## Experience Items

Each item in the `experience` array.

**Required:**
- `title`: Job title
- `company`: Company name

**Optional:**
- `location`: Job location
- `start`: Start date (recommend 'YYYY-MM' format)
- `end`: End date or 'present'
- `summary`: Role summary
- `technologies`: Array of technologies used
- `highlights`: Array of achievement highlights

Unknown fields in experience items are errors.

## Project Items

Each item in the `projects` array.

**Required:**
- `name`: Project name

**Optional:**
- `url`: Project URL
- `summary`: Project description
- `role`: Your role
- `start`: Start date
- `end`: End date
- `technologies`: Array of technologies

Unknown fields in project items are errors.

## Education Items

Each item in the `education` array.

**Required:**
- `institution`: School or institution name
- `degree`: Degree or certification

**Optional:**
- `location`: Institution location
- `start`: Start date
- `end`: End date
- `details`: Array of additional details

Unknown fields in education items are errors.

## Validation

The parser performs strict validation:

- Unknown top-level keys are errors
- Unknown fields in sections are errors
- Type mismatches are errors
- Multiple errors are collected and reported together

Errors include path context for debugging:
- `person.name`: Missing required field
- `experience[0].unknown`: Unknown field in first experience item
- `skills.languages`: Invalid type

## Versioning

JOBL does not require a version field in documents. The library version determines which specification features are supported.

## Example

```toml
[person]
name = "Jane Developer"
email = "jane@example.com"
headline = "Senior Software Engineer"
location = "San Francisco, CA"
website = "https://jane.dev"
summary = "Experienced engineer specializing in distributed systems"

[skills]
languages = ["Rust", "Python", "Go", "JavaScript"]
frameworks = ["Tokio", "Django", "React"]
tools = ["Docker", "Kubernetes", "PostgreSQL"]

[[experience]]
title = "Senior Software Engineer"
company = "Tech Corp"
location = "San Francisco, CA"
start = "2020-01"
end = "present"
summary = "Lead engineer for distributed data platform"
technologies = ["Rust", "Kubernetes", "PostgreSQL"]
highlights = [
  "Designed and built distributed event processing system handling 1M+ events/sec",
  "Led team of 5 engineers through migration to microservices architecture",
  "Reduced infrastructure costs by 40% through optimization"
]

[[experience]]
title = "Software Engineer"
company = "Startup Inc"
location = "Remote"
start = "2018-06"
end = "2019-12"
summary = "Full-stack engineer on core product team"
technologies = ["Python", "Django", "React", "PostgreSQL"]
highlights = [
  "Built REST API serving 10k+ requests/minute",
  "Implemented real-time collaboration features"
]

[[projects]]
name = "open-source-project"
url = "https://github.com/jane/project"
summary = "High-performance data processing library"
role = "Creator and maintainer"
technologies = ["Rust", "SIMD"]

[[education]]
institution = "University of California"
degree = "BS Computer Science"
location = "Berkeley, CA"
start = "2014"
end = "2018"
details = ["GPA: 3.8", "Dean's List"]
```
