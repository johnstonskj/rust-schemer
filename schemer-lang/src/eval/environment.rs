/*!
One-line description.

More detailed description, with

# Example

*/

use crate::error::{Error, ErrorKind};
use crate::eval::callable::Callable;
use crate::eval::expression::Expression;
use crate::read::syntax_str::{
    PSEUDO_SYNTAX_LEFT_PROCEDURE, PSEUDO_SYNTAX_RIGHT_PROCEDURE, SYNTAX_HYPHEN, SYNTAX_SPACE_CHAR,
};
use crate::types::new_type::NewType;
use crate::types::{Identifier, MutableRef, Ref, SchemeRepr, SchemeValue};
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::Write;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq)]
pub struct Environment {
    name: Option<String>,
    bindings: ExportList,
    parent: Option<MutableRef<Environment>>,
    immutable: bool,
}

pub const TYPE_NAME_ENVIRONMENT: &str = "environment";

pub type ExportList = BTreeMap<Identifier, Expression>;

pub type Exports = NewType<ExportList>;

pub const TOP_ENVIRONMENT_NAME: &str = "*top*";

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub fn print(env: &Environment) {
    print_to(env, &mut std::io::stdout())
}

pub fn print_to<W: Write>(env: &Environment, w: &mut W) {
    print_inner(env, w, &String::new())
}

fn print_inner<W: Write>(env: &Environment, w: &mut W, prefix: &str) {
    if env.local_len() == 0 {
        let _ = writeln!(
            w,
            "{}╾╴ {}",
            prefix,
            match env.name() {
                None => "?",
                Some(v) => v.as_str(),
            }
        );
    } else {
        let _ = writeln!(
            w,
            "{}┌╴ {}",
            prefix,
            match env.name() {
                None => "?",
                Some(v) => v.as_str(),
            }
        );
        let bindings = env.bindings();
        let last = env.local_len() - if env.parent.is_some() { 0 } else { 1 };
        for (i, (k, v)) in bindings.enumerate() {
            let _ = writeln!(
                w,
                "{}{} ('{} . {})",
                prefix,
                if i < last { "│ " } else { "└╴" },
                k.to_repr_string(),
                v.to_repr_string()
            );
        }
    }
    if let Some(parent) = env.parent() {
        print_inner(&parent.borrow(), w, &format!("{}│  ", prefix));
        let _ = writeln!(w, "{}└╴ ", prefix);
    }
}

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
    pub fn top() -> MutableRef<Self> {
        Self {
            name: Some(TOP_ENVIRONMENT_NAME.to_string()),
            bindings: Default::default(),
            parent: None,
            immutable: false,
        }
        .into_ref()
    }

    pub fn new_child_named(parent: MutableRef<Self>, name: &str) -> MutableRef<Self> {
        Self {
            name: Some(format!(
                "{}",
                name.replace(SYNTAX_SPACE_CHAR, SYNTAX_HYPHEN)
            )),
            bindings: Default::default(),
            parent: Some(parent),
            immutable: false,
        }
        .into_ref()
    }

    pub fn name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    pub fn return_to_parent(self) -> Option<MutableRef<Self>> {
        self.parent
    }

    pub fn into_ref(self) -> MutableRef<Self> {
        Ref::from(RefCell::from(self))
    }

    pub fn insert(
        &mut self,
        name: Identifier,
        value: Expression,
    ) -> Result<Option<Expression>, Error> {
        if self.is_immutable() {
            Err(Error::from(ErrorKind::ImmutableEnvironment))
        } else {
            Ok(self.bindings.insert(name, value.into()))
        }
    }

    pub fn update(
        &mut self,
        name: Identifier,
        value: Expression,
    ) -> Result<Option<Expression>, Error> {
        if self.is_immutable() {
            Err(Error::from(ErrorKind::ImmutableEnvironment))
        } else {
            if let Some(old_value) = self.bindings.get(&name) {
                if old_value.is_form() || old_value.is_builtin_procedure() {
                    Err(Error::from(ErrorKind::ImmutableValue {
                        name,
                        type_name: old_value.type_name().to_string(),
                    }))
                } else {
                    Ok(self.bindings.insert(name, value))
                }
            } else if let Some(parent) = &mut self.parent {
                parent.borrow_mut().update(name, value)
            } else {
                Err(Error::from(ErrorKind::UnboundVariable { name }))
            }
        }
    }

    pub fn import(&mut self, other: Exports) -> Result<(), Error> {
        if self.is_immutable() {
            Err(Error::from(ErrorKind::ImmutableEnvironment))
        } else {
            for (id, expr) in other.iter() {
                // TODO: need to drain?
                self.bindings.insert(id.clone(), expr.clone());
            }
            Ok(())
        }
    }

    pub fn get(&self, name: &Identifier) -> Option<Expression> {
        match (self.bindings.get(name), &self.parent) {
            (None, Some(parent)) => parent.borrow().get(name),
            (Some(value), _) => Some(value.clone()),
            _ => None,
        }
    }

    pub fn is_bound(&self, name: &Identifier) -> bool {
        match (self.bindings.contains_key(name), &self.parent) {
            (false, Some(parent)) => parent.borrow().is_bound(name),
            (true, _) => true,
            _ => false,
        }
    }

    pub fn has_parent(&self) -> bool {
        self.parent.is_some()
    }

    pub fn parent(&self) -> Option<&MutableRef<Self>> {
        self.parent.as_ref()
    }

    pub fn is_immutable(&self) -> bool {
        self.immutable
    }

    pub fn make_immutable(&mut self) {
        self.immutable = true;
    }

    pub fn bindings(&self) -> impl Iterator<Item = (&Identifier, &Expression)> {
        self.bindings.iter()
    }

    pub fn binding_names(&self) -> impl Iterator<Item = &Identifier> {
        self.bindings.keys()
    }

    pub fn local_len(&self) -> usize {
        self.bindings.len()
    }

    pub fn completions(&self, prefix: &str) -> Vec<(String, String)> {
        if let Some(parent) = &self.parent {
            let parent = parent.borrow();
            let mut parent_bindings = parent.completions(prefix).clone();
            parent_bindings.extend(self.local_completions(prefix));
            parent_bindings
        } else {
            self.local_completions(prefix)
        }
    }

    pub fn local_completions(&self, prefix: &str) -> Vec<(String, String)> {
        self.bindings
            .iter()
            .filter_map(|(id, expr)| {
                if id.starts_with(prefix) {
                    Some((
                        id.to_string(),
                        format!("{}:{}", id.as_str(), expr.type_name()),
                    ))
                } else {
                    None
                }
            })
            .collect()
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
            let new_id = id_from_str!(&format!("{}{}", prefix.as_str(), id.as_str()));
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
