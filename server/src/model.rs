use anyhow::anyhow;
use std::fmt::{Display, Formatter};

#[derive(Clone)]
pub struct Repository {
    pub name: String,
    pub group: String,
}

impl Repository {
    pub fn from_slug(slug: &str) -> anyhow::Result<Self> {
        let parts: Vec<&str> = slug.split('/').collect();

        let group = parts
            .first()
            .ok_or_else(|| anyhow!("Could not parse group name from {}.", slug))?;
        let name = parts
            .last()
            .ok_or_else(|| anyhow!("Could not parse repository name from {}.", slug))?;

        Ok(Self {
            name: name.to_string(),
            group: group.to_string(),
        })
    }
}

impl Display for Repository {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.group, self.name)
    }
}
