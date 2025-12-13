mod error;
mod parser;
mod types;

pub use error::{ValidationError, ValidationResult};
pub use parser::{parse_file, parse_str};
pub use types::{
    EducationItem, ExperienceItem, JoblDocument, Person, ProjectItem,
};
