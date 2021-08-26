/*!
One-line description.

More detailed description, with

# Example

*/

use crate::read::datum::Datum;
use crate::read::syntax_str::{
    SYNTAX_CONS_DOT, SYNTAX_LEFT_PARENTHESIS, SYNTAX_NOTHING, SYNTAX_RIGHT_PARENTHESIS,
    SYNTAX_SPACE,
};
use crate::types::SchemeRepr;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq)]
pub struct Pair {
    car: Box<Datum>,
    cdr: Box<Datum>,
}

pub const NULL_LIST_REPR_STRING: &str = "()";

pub const NULL_LIST_REPR_NAME: &str = "null";

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub fn empty_list() -> Box<Pair> {
    Box::new(Pair {
        car: Box::new(Datum::Null),
        cdr: Box::new(Datum::Null),
    })
}

pub fn make_list(k: usize) -> Box<Pair> {
    if k == 0 {
        empty_list()
    } else {
        let mut head = Box::new(Pair::cons_nil(Datum::from(false)));
        for _ in 1..k {
            head = Box::new(Pair::cons_list(Datum::from(false), *head));
        }
        head
    }
}

pub fn make_filled_list(k: usize, fill: &Datum) -> Box<Pair> {
    if k == 0 {
        empty_list()
    } else {
        let mut head = Box::new(Pair::cons_nil(Datum::from(fill.clone())));
        for _ in 1..k {
            head = Box::new(Pair::cons_list(Datum::from(fill.clone()), *head));
        }
        head
    }
}

pub fn list(data: Vec<Datum>) -> Box<Pair> {
    let mut head = empty_list();
    for datum in data.into_iter().rev() {
        if is_null(&head) {
            head = Box::new(Pair::cons_nil(datum));
        } else {
            head = Box::new(Pair::cons_list(datum, *head));
        }
    }
    head
}

pub fn is_null(list: &Pair) -> bool {
    list.car.is_null() && list.cdr.is_null()
}

pub fn is_proper_pair(pair: &Pair) -> bool {
    pair.cdr.is_list_or_null()
}

pub fn is_proper_list(list: &Pair) -> bool {
    let cdr = &list.cdr;
    match &**cdr {
        Datum::List(list) => is_proper_list(list),
        Datum::Null => true,
        _ => false,
    }
}

pub fn length(list: &Pair) -> usize {
    let cdr = &list.cdr;
    if is_null(list) {
        0
    } else if let Datum::List(pair) = &**cdr {
        1 + length(pair)
    } else {
        1
    }
}

// pub fn append(lhs: List, rhs: &[List]) -> usize {}
//
// pub fn reverse(list: List) -> List {}

pub fn head(list: &Pair) -> &Datum {
    list.car()
}

// pub fn tail(list: &List) -> List {}
//
// pub fn tail_after(list: &List, k: usize) -> List {}
//
// pub fn list_ref(list: &List, k: usize) -> Option<&Datum> {}
//
// pub fn list_set(list: &List, k: usize, datum: Datum) -> Option<&Datum> {}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl SchemeRepr for Pair {
    fn to_repr_string(&self) -> String {
        self.repr_string(true)
    }
}

impl From<Datum> for Pair {
    fn from(v: Datum) -> Self {
        Self::cons_nil(v)
    }
}

impl Pair {
    pub fn cons(car: Datum, cdr: Datum) -> Self {
        Self {
            car: car.into(),
            cdr: cdr.into(),
        }
    }

    pub fn cons_pair(car: Datum, cdr: Pair) -> Self {
        Self {
            car: car.into(),
            cdr: Box::new(Datum::List(Box::new(cdr))),
        }
    }

    pub fn cons_list(car: Datum, cdr: Pair) -> Self {
        Self {
            car: car.into(),
            cdr: Box::new(Datum::List(cdr.into())),
        }
    }

    pub fn cons_nil(car: Datum) -> Self {
        Self {
            car: car.into(),
            cdr: Datum::Null.into(),
        }
    }

    pub fn car(&self) -> &Datum {
        &self.car
    }

    pub fn set_car(&mut self, datum: Datum) {
        self.car = Box::new(datum);
    }

    pub fn cdr(&self) -> &Box<Datum> {
        &self.cdr
    }

    pub fn set_cdr(&mut self, datum: Datum) {
        self.cdr = Box::new(datum);
    }

    fn repr_string(&self, bounded: bool) -> String {
        let proper = is_proper_pair(self);
        // TODO: use global CONS write flag
        format!(
            "{}{}{}{}{}",
            if bounded {
                SYNTAX_LEFT_PARENTHESIS
            } else {
                SYNTAX_NOTHING
            },
            self.car().to_repr_string(),
            if proper && self.cdr.is_null() {
                SYNTAX_NOTHING
            } else if proper {
                SYNTAX_SPACE
            } else {
                SYNTAX_CONS_DOT
            },
            match &*self.cdr {
                Datum::List(p) => p.repr_string(!proper),
                Datum::Null => if proper {
                    SYNTAX_NOTHING
                } else {
                    NULL_LIST_REPR_STRING
                }
                .to_string(),
                cdr => cdr.to_repr_string(),
            },
            if bounded {
                SYNTAX_RIGHT_PARENTHESIS
            } else {
                SYNTAX_NOTHING
            },
        )
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
