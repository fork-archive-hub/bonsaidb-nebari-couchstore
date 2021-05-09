use serde::{Deserialize, Serialize};

use crate::{
    schema::{Collection, CollectionName, InvalidNameError, Name, Schematic, View},
    Error,
};

/// An assignable role, which grants permissions based on the associated [`PermissionGroup`](crate::permissions::PermissionGroup)s.
#[derive(Debug, Serialize, Deserialize)]
#[allow(clippy::module_name_repetitions)]
pub struct Role {
    /// The name of the role. Must be unique.
    pub name: String,
    /// The IDs of the permission groups this role belongs to.
    pub groups: Vec<u64>,
}

impl Collection for Role {
    fn collection_name() -> Result<CollectionName, InvalidNameError> {
        CollectionName::new("khonsulabs", "role")
    }

    fn define_views(schema: &mut Schematic) -> Result<(), Error> {
        schema.define_view(ByName)
    }
}

/// A unique view of roles by name.
#[derive(Debug)]
pub struct ByName;

impl View for ByName {
    type Collection = Role;
    type Key = String;
    type Value = ();

    fn unique(&self) -> bool {
        true
    }

    fn version(&self) -> u64 {
        1
    }

    fn name(&self) -> Result<Name, InvalidNameError> {
        Name::new("by-name")
    }

    fn map(
        &self,
        document: &crate::document::Document<'_>,
    ) -> crate::schema::MapResult<Self::Key, Self::Value> {
        let role = document.contents::<Role>()?;
        Ok(Some(document.emit_key(role.name)))
    }
}
