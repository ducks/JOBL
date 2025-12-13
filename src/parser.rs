use crate::error::{ValidationError, ValidationResult};
use crate::types::JoblDocument;
use std::collections::{BTreeMap, HashSet};
use std::fs;
use std::path::Path;

/// Parse a JOBL file from disk.
pub fn parse_file(path: impl AsRef<Path>) -> ValidationResult<JoblDocument> {
    let content = fs::read_to_string(path.as_ref())
        .map_err(|e| vec![ValidationError::new("file", e.to_string())])?;
    parse_str(&content)
}

/// Parse a JOBL document from a string.
pub fn parse_str(input: &str) -> ValidationResult<JoblDocument> {
    let mut errors = Vec::new();

    // Parse TOML into generic value first for validation
    let value: toml::Value = toml::from_str(input)
        .map_err(|e| vec![ValidationError::new("document", e.to_string())])?;

    // Check for unknown top-level keys
    if let toml::Value::Table(table) = &value {
        let allowed_keys: HashSet<&str> =
            ["person", "skills", "experience", "projects", "education"]
                .into_iter()
                .collect();

        for key in table.keys() {
            if !allowed_keys.contains(key.as_str()) {
                errors.push(ValidationError::new(
                    key.clone(),
                    format!("unknown top-level key '{}'", key),
                ));
            }
        }
    }

    // Attempt to deserialize into typed structure
    let doc: JoblDocument = match toml::from_str(input) {
        Ok(d) => d,
        Err(e) => {
            errors.push(ValidationError::new("document", e.to_string()));
            if !errors.is_empty() {
                return Err(errors);
            }
            unreachable!();
        }
    };

    // Validate person.name exists (checked by serde, but explicit)
    if doc.person.name.is_empty() {
        errors.push(ValidationError::new(
            "person.name",
            "name cannot be empty",
        ));
    }

    // Validate skills section (values must be arrays of strings)
    if let Some(skills) = &doc.skills {
        validate_skills(skills, &mut errors);
    }

    // Validate unknown fields in sections by re-parsing
    validate_no_unknown_fields(&value, &mut errors);

    if errors.is_empty() {
        Ok(doc)
    } else {
        Err(errors)
    }
}

/// Validate skills: all values must be arrays of strings.
fn validate_skills(
    skills: &BTreeMap<String, Vec<String>>,
    errors: &mut Vec<ValidationError>,
) {
    for (category, items) in skills {
        if items.is_empty() {
            errors.push(ValidationError::new(
                format!("skills.{}", category),
                "category cannot be empty",
            ));
        }
    }
}

/// Validate no unknown fields in known sections.
/// This checks person, experience items, project items, education items.
fn validate_no_unknown_fields(
    value: &toml::Value,
    errors: &mut Vec<ValidationError>,
) {
    let table = match value.as_table() {
        Some(t) => t,
        None => return,
    };

    // Check person section
    if let Some(toml::Value::Table(person_table)) = table.get("person") {
        let allowed: HashSet<&str> = [
            "name", "headline", "location", "email", "website", "phone",
            "summary",
        ]
        .into_iter()
        .collect();

        for key in person_table.keys() {
            if !allowed.contains(key.as_str()) {
                errors.push(ValidationError::new(
                    format!("person.{}", key),
                    format!("unknown field '{}'", key),
                ));
            }
        }
    }

    // Check experience items
    if let Some(toml::Value::Array(exp_array)) = table.get("experience") {
        let allowed: HashSet<&str> = [
            "title",
            "company",
            "location",
            "start",
            "end",
            "summary",
            "technologies",
            "highlights",
        ]
        .into_iter()
        .collect();

        for (idx, item) in exp_array.iter().enumerate() {
            if let toml::Value::Table(item_table) = item {
                for key in item_table.keys() {
                    if !allowed.contains(key.as_str()) {
                        errors.push(ValidationError::new(
                            format!("experience[{}].{}", idx, key),
                            format!("unknown field '{}'", key),
                        ));
                    }
                }
            }
        }
    }

    // Check project items
    if let Some(toml::Value::Array(proj_array)) = table.get("projects") {
        let allowed: HashSet<&str> = [
            "name", "url", "summary", "role", "start", "end",
            "technologies",
        ]
        .into_iter()
        .collect();

        for (idx, item) in proj_array.iter().enumerate() {
            if let toml::Value::Table(item_table) = item {
                for key in item_table.keys() {
                    if !allowed.contains(key.as_str()) {
                        errors.push(ValidationError::new(
                            format!("projects[{}].{}", idx, key),
                            format!("unknown field '{}'", key),
                        ));
                    }
                }
            }
        }
    }

    // Check education items
    if let Some(toml::Value::Array(edu_array)) = table.get("education") {
        let allowed: HashSet<&str> = [
            "institution", "degree", "location", "start", "end", "details",
        ]
        .into_iter()
        .collect();

        for (idx, item) in edu_array.iter().enumerate() {
            if let toml::Value::Table(item_table) = item {
                for key in item_table.keys() {
                    if !allowed.contains(key.as_str()) {
                        errors.push(ValidationError::new(
                            format!("education[{}].{}", idx, key),
                            format!("unknown field '{}'", key),
                        ));
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_minimal() {
        let input = r#"
[person]
name = 'Jane Doe'
"#;
        let result = parse_str(input);
        assert!(result.is_ok());
        let doc = result.unwrap();
        assert_eq!(doc.person.name, "Jane Doe");
    }

    #[test]
    fn test_valid_complete() {
        let input = r#"
[person]
name = 'Jane Doe'
headline = 'Software Engineer'
email = 'jane@example.com'

[skills]
languages = ['Rust', 'Python']
devops = ['Docker', 'Kubernetes']

[[experience]]
title = 'Senior Engineer'
company = 'Acme Corp'
start = '2020-01'
end = 'present'

[[projects]]
name = 'My Project'
url = 'https://example.com'

[[education]]
institution = 'University'
degree = 'BS Computer Science'
"#;
        let result = parse_str(input);
        assert!(result.is_ok());
        let doc = result.unwrap();
        assert_eq!(doc.experience.len(), 1);
        assert_eq!(doc.projects.len(), 1);
        assert_eq!(doc.education.len(), 1);
    }

    #[test]
    fn test_missing_person_name() {
        let input = r#"
[person]
email = 'jane@example.com'
"#;
        let result = parse_str(input);
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert!(errors
            .iter()
            .any(|e| e.message.contains("missing field `name`")));
    }

    #[test]
    fn test_unknown_top_level_key() {
        let input = r#"
[person]
name = 'Jane Doe'

[unknown_section]
foo = 'bar'
"#;
        let result = parse_str(input);
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert!(errors
            .iter()
            .any(|e| e.path == "unknown_section"
                && e.message.contains("unknown top-level key")));
    }

    #[test]
    fn test_wrong_skills_type() {
        let input = r#"
[person]
name = 'Jane Doe'

[skills]
languages = 'Rust'
"#;
        let result = parse_str(input);
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert!(errors.iter().any(|e| e.message.contains("invalid type")));
    }

    #[test]
    fn test_unknown_field_in_experience() {
        let input = r#"
[person]
name = 'Jane Doe'

[[experience]]
title = 'Engineer'
company = 'Acme'
unknown_field = 'value'
"#;
        let result = parse_str(input);
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert!(errors.iter().any(
            |e| e.path == "experience[0].unknown_field"
                && e.message.contains("unknown field")
        ));
    }

    #[test]
    fn test_multiple_errors_collected() {
        let input = r#"
[person]
name = 'Jane Doe'
unknown_person_field = 'value'

[unknown_section]
foo = 'bar'
"#;
        let result = parse_str(input);
        assert!(result.is_err());
        let errors = result.unwrap_err();
        // Should have multiple errors collected
        assert!(errors.len() >= 2);
    }
}
