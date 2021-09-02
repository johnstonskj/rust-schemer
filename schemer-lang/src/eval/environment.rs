/*!
One-line description.

More detailed description, with

# Example

*/

use std::collections::BTreeMap;

use crate::error::{Error, ErrorKind};
use crate::eval::callable::Callable;
use crate::eval::expression::Expression;
use crate::eval::procedures::Procedure;
use crate::read::syntax_str::{
    PSEUDO_SYNTAX_LEFT_PROCEDURE, PSEUDO_SYNTAX_RIGHT_PROCEDURE, SYNTAX_HYPHEN, SYNTAX_SPACE_CHAR,
};
use crate::types::new_type::NewType;
use crate::types::{Identifier, Ref, SchemeRepr, SchemeValue};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq)]
pub struct Environment {
    name: Option<String>,
    values: ExportList,
    parent: Option<Ref<Environment>>,
    immutable: bool,
}

pub const TYPE_NAME_ENVIRONMENT: &str = "environment";

pub type ExportList = BTreeMap<Identifier, Expression>;

pub type Exports = NewType<ExportList>;

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl SchemeValue for Environment {
    fn type_name(&self) -> &'static str {
        TYPE_NAME_ENVIRONMENT
    }
}

impl SchemeRepr for Environment {
    fn to_repr_string(&self) -> String {
        format!(
            "{}{}{}",
            PSEUDO_SYNTAX_LEFT_PROCEDURE, TYPE_NAME_ENVIRONMENT, PSEUDO_SYNTAX_RIGHT_PROCEDURE
        )
    }
}

impl Environment {
    pub fn named(name: &str) -> Ref<Self> {
        Self {
            name: Some(name.to_string()),
            values: Default::default(),
            parent: None,
            immutable: false,
        }
        .into()
    }

    pub fn top_level() -> Ref<Self> {
        Environment::named("*top*").into()
    }

    pub fn empty() -> Ref<Self> {
        Environment::named("*empty*").into()
    }

    pub fn new_child(parent: &Ref<Self>) -> Self {
        Self {
            name: None,
            values: Default::default(),
            parent: Some(parent.clone()),
            immutable: false,
        }
    }

    pub fn new_child_named(parent: &Ref<Self>, name: &str) -> Self {
        Self {
            name: Some(format!(
                "*{}*",
                name.replace(SYNTAX_SPACE_CHAR, SYNTAX_HYPHEN)
            )),
            values: Default::default(),
            parent: Some(parent.clone()),
            immutable: false,
        }
    }

    pub fn return_to_parent(self) -> Option<Ref<Self>> {
        self.parent
    }

    pub fn insert(
        &mut self,
        name: Identifier,
        value: Expression,
    ) -> Result<Option<Expression>, Error> {
        if self.is_immutable() {
            Err(Error::from(ErrorKind::ImmutableEnvironment))
        } else {
            Ok(self.values.insert(name, value.into()))
        }
    }

    pub fn insert_procedure(&mut self, value: Procedure) -> Result<Option<Expression>, Error> {
        if self.is_immutable() {
            Err(Error::from(ErrorKind::ImmutableEnvironment))
        } else {
            Ok(self
                .values
                .insert(value.id().clone(), Expression::Procedure(value.into())))
        }
    }

    pub fn import(&mut self, other: Exports) -> Result<(), Error> {
        if self.is_immutable() {
            Err(Error::from(ErrorKind::ImmutableEnvironment))
        } else {
            for (id, expr) in other.iter() {
                // TODO: need to drain?
                self.values.insert(id.clone(), expr.clone());
            }
            Ok(())
        }
    }

    pub fn get(&self, name: &Identifier) -> Option<&Expression> {
        match (self.values.get(name), &self.parent) {
            (None, Some(parent)) => parent.get(name),
            (Some(value), _) => Some(value),
            _ => None,
        }
    }

    pub fn is_bound(&self, name: &Identifier) -> bool {
        match (self.values.contains_key(name), &self.parent) {
            (false, Some(parent)) => parent.is_bound(name),
            (true, _) => true,
            _ => false,
        }
    }

    pub fn print(&self) {
        self.print_inner("")
    }

    fn print_inner(&self, prefix: &str) {
        if self.values.is_empty() {
            println!(
                "{}╾╴ {}",
                prefix,
                match &self.name {
                    None => "?",
                    Some(v) => v.as_str(),
                }
            );
        } else {
            println!(
                "{}┌╴ {}",
                prefix,
                match &self.name {
                    None => "?",
                    Some(v) => v.as_str(),
                }
            );
            let last = self.values.len() - if self.parent.is_some() { 0 } else { 1 };
            for (i, (k, v)) in self.values.iter().enumerate() {
                println!(
                    "{}{} ('{} . {})",
                    prefix,
                    if i < last { "│ " } else { "└╴" },
                    k.to_repr_string(),
                    v.to_repr_string()
                );
            }
        }
        if let Some(parent) = &self.parent {
            parent.print_inner(&format!("{}│  ", prefix));
            println!("{}└╴ ", prefix);
        }
    }

    pub fn has_parent(&self) -> bool {
        self.parent.is_some()
    }

    pub fn parent(&self) -> Option<&Ref<Environment>> {
        self.parent.as_ref()
    }

    pub fn is_immutable(&self) -> bool {
        self.immutable
    }

    pub fn make_immutable(mut self) -> Self {
        self.immutable = true;
        self
    }
}

// ------------------------------------------------------------------------------------------------

impl Exports {
    pub fn only(&mut self, names: &[&Identifier]) -> &mut Self {
        self.retain(|id, _| names.contains(&id));
        self
    }

    pub fn except(&mut self, names: &[&Identifier]) -> &mut Self {
        self.retain(|id, _| !names.contains(&id));
        self
    }

    pub fn rename(&mut self, renames: &BTreeMap<&Identifier, &Identifier>) -> &mut Self {
        for (from, to) in renames {
            self.rename_one(from, to);
        }
        self
    }

    pub fn prefix(&mut self, prefix: &Identifier) -> &mut Self {
        let all_ids: Vec<Identifier> = self.keys().cloned().collect();
        for id in all_ids {
            let new_id =
                Identifier::from_str_unchecked(&format!("{}{}", prefix.as_str(), id.as_str()));
            self.rename_one(&id, &new_id);
        }
        self
    }

    fn rename_one(&mut self, from: &Identifier, to: &Identifier) -> &mut Self {
        // TODO: error handling?
        let mut expr = self.remove(from).unwrap();
        if let Expression::Procedure(p) = &mut expr {
            p.rename(to.clone())
        }
        self.insert(to.clone(), expr);
        self
    }

    pub fn import(&mut self, other: Exports) {
        self.extend(other.iter().map(|(id, expr)| (id.clone(), expr.clone())))
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
