use std::path::{Path, PathBuf};

use crate::{PostgresExplorer, TableDefinition};
use askama::Template;
use heck::ToUpperCamelCase;
use indexmap::IndexMap;

pub mod wri_index;
pub mod wri_table;
