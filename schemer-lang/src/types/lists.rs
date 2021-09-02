/*!
One-line description.

More detailed description, with

# Example

*/

use crate::read::datum::Datum;
use crate::read::syntax_str::{
    EMPTY_STR, SYNTAX_CONS_DOT, SYNTAX_LEFT_PARENTHESIS, SYNTAX_RIGHT_PARENTHESIS, SYNTAX_SPACE,
    VALUE_NULL_LIST,
};
use crate::types::{Ref, SchemeRepr, SchemeValue, Vector};
use std::iter::FromIterator;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq)]
pub struct Pair {
    car: Ref<Datum>,
    cdr: Ref<Datum>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PairIterator<'a> {
    current: Option<&'a Pair>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ListIterator<'a> {
    current: Option<&'a Pair>,
    take_cdr: bool,
}

pub const TYPE_NAME_LIST: &str = "list";

pub const TYPE_NAME_PAIR: &str = "pair";

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

lazy_static! {
    static ref SHARED_NULL: Ref<Datum> = Ref::new(Datum::Null);
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub fn make_list(k: usize) -> Pair {
    make_filled_list(k, &Datum::Boolean(false.into()))
}

pub fn make_filled_list(k: usize, fill: &Datum) -> Pair {
    if k == 0 {
        Pair::empty()
    } else {
        let mut head = Pair::cons_nil(Datum::from(fill.clone()).into());
        for _ in 1..k {
            head = Pair::cons_list(Datum::from(fill.clone()).into(), head);
        }
        head
    }
}

pub fn vector_to_list(data: Vector<Datum>) -> Pair {
    let mut head = Pair::empty();
    for datum in data.iter().rev().cloned() {
        if head.is_null() {
            head = Pair::cons_nil(datum);
        } else {
            head = Pair::cons_list(datum, head);
        }
    }
    head
}

pub fn vec_to_list(data: Vec<Datum>) -> Pair {
    vector_to_list(data.into())
}

pub fn list_to_vec(list: Pair) -> Vec<Ref<Datum>> {
    let result: Vec<Ref<Datum>> = Vec::from_iter(list.into_iter().cloned());
    result
}

pub fn list_to_vector(list: Pair) -> Vector<Datum> {
    Vector::from(list_to_vec(list))
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl SchemeRepr for Pair {
    fn to_repr_string(&self) -> String {
        self.repr_string(true)
    }
}

impl SchemeValue for Pair {
    fn type_name(&self) -> &'static str
    where
        Self: Sized,
    {
        if self.is_proper_list() {
            TYPE_NAME_LIST
        } else {
            TYPE_NAME_PAIR
        }
    }
}

impl FromIterator<Datum> for Pair {
    #[allow(unused_assignments)]
    fn from_iter<I: IntoIterator<Item = Datum>>(iter: I) -> Self {
        vector_to_list(iter.into_iter().collect())
    }
}

impl<'a> IntoIterator for &'a Pair {
    type Item = &'a Ref<Datum>;
    type IntoIter = ListIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        ListIterator {
            current: Some(self),
            take_cdr: false,
        }
    }
}

impl From<Datum> for Pair {
    fn from(v: Datum) -> Self {
        Self::cons_nil(Ref::new(v))
    }
}

impl From<(Datum, Datum)> for Pair {
    fn from(v: (Datum, Datum)) -> Self {
        Self::cons(Ref::new(v.0), Ref::new(v.1))
    }
}

impl Pair {
    pub fn empty() -> Self {
        Self::cons(SHARED_NULL.clone(), SHARED_NULL.clone())
    }

    pub fn cons(car: Ref<Datum>, cdr: Ref<Datum>) -> Self {
        Self { car, cdr }
    }

    pub fn cons_list(car: Ref<Datum>, cdr: Pair) -> Self {
        Self::cons(car, Datum::List(cdr).into())
    }

    pub fn cons_nil(car: Ref<Datum>) -> Self {
        Self::cons(car, SHARED_NULL.clone())
    }

    pub fn car(&self) -> &Ref<Datum> {
        &self.car
    }

    pub fn set_car(&mut self, datum: Ref<Datum>) {
        self.car = datum;
    }

    pub fn cdr(&self) -> &Ref<Datum> {
        &self.cdr
    }

    pub fn set_cdr(&mut self, datum: Ref<Datum>) {
        self.cdr = datum;
    }

    pub fn iter(&self) -> impl Iterator<Item = &Ref<Datum>> {
        ListIterator {
            current: Some(self),
            take_cdr: false,
        }
    }

    pub fn pairs(&self) -> impl Iterator<Item = &Pair> {
        PairIterator {
            current: Some(self),
        }
    }

    pub fn is_null(&self) -> bool {
        self.car.is_null() && self.cdr.is_null()
    }

    pub fn is_proper_pair(&self) -> bool {
        self.cdr.is_list_or_null()
    }

    pub fn is_proper_list(&self) -> bool {
        match &*self.cdr {
            Datum::List(list) => list.is_proper_list(),
            Datum::Null => true,
            _ => false,
        }
    }

    pub fn length(&self) -> usize {
        if self.is_null() {
            0
        } else if let Datum::List(pair) = &*self.cdr {
            1 + pair.length()
        } else {
            1
        }
    }

    pub fn append(&mut self, rhs: Pair) -> Option<()> {
        if let Some(last) = self.last_mut() {
            last.cdr = Ref::new(Datum::List(rhs));
            Some(())
        } else {
            None
        }
    }

    // pub fn reverse(list: List) -> List {}

    pub fn head(&self) -> &Datum {
        self.car()
    }

    pub fn tail(&self) -> Option<&Pair> {
        if let Datum::List(pair) = &*self.cdr {
            Some(pair)
        } else {
            None
        }
    }

    pub fn last(&self) -> Option<&Pair> {
        if self.cdr.is_null() {
            Some(self)
        } else if let Datum::List(pair) = &*self.cdr {
            pair.last()
        } else {
            None
        }
    }

    pub fn last_mut(&mut self) -> Option<&mut Pair> {
        if self.cdr.is_null() {
            Some(self)
        } else if let Some(Datum::List(pair)) = Ref::get_mut(&mut self.cdr) {
            pair.last_mut()
        } else {
            None
        }
    }

    // pub fn tail_after(list: &List, k: usize) -> List {}
    //
    // pub fn list_ref(list: &List, k: usize) -> Option<&Datum> {}
    //
    // pub fn list_set(list: &List, k: usize, datum: Datum) -> Option<&Datum> {}

    fn repr_string(&self, bounded: bool) -> String {
        let proper = self.is_proper_pair();
        // TODO: use global CONS write flag
        format!(
            "{}{}{}{}{}",
            if bounded {
                SYNTAX_LEFT_PARENTHESIS
            } else {
                EMPTY_STR
            },
            self.car().to_repr_string(),
            if proper && self.cdr.is_null() {
                EMPTY_STR
            } else if proper {
                SYNTAX_SPACE
            } else {
                SYNTAX_CONS_DOT
            },
            match &*self.cdr {
                Datum::List(p) => p.repr_string(!proper),
                Datum::Null => if proper { EMPTY_STR } else { VALUE_NULL_LIST }.to_string(),
                cdr => cdr.to_repr_string(),
            },
            if bounded {
                SYNTAX_RIGHT_PARENTHESIS
            } else {
                EMPTY_STR
            },
        )
    }
}

// ------------------------------------------------------------------------------------------------

impl<'a> Iterator for PairIterator<'a> {
    type Item = &'a Pair;

    fn next(&mut self) -> Option<Self::Item> {
        match self.current {
            None => None,
            Some(pair) => {
                if let Datum::List(next) = &**pair.cdr() {
                    self.current = Some(next);
                } else {
                    self.current = None;
                }
                Some(pair)
            }
        }
    }
}

impl<'a> Iterator for ListIterator<'a> {
    type Item = &'a Ref<Datum>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.current {
            None => None,
            Some(pair) => {
                if let Datum::List(next) = &**pair.cdr() {
                    self.current = Some(next);
                } else {
                    self.current = None;
                }
                if self.take_cdr {
                    self.current = None;
                    Some(&pair.cdr)
                } else {
                    Some(&pair.car)
                }
            }
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
