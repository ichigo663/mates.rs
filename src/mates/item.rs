#[phase(plugin)]
extern crate peg_syntax_ext;

use std::collections::HashMap;

pub struct PropertyValue {
    params: String,
    value: String,
}

impl PropertyValue {
    pub fn get_raw_value(&self) -> &String { &self.value }
    pub fn get_raw_params(&self) -> &String { &self.params }
}


pub struct Item {
    pub props: HashMap<String, Vec<PropertyValue>>,
}

impl Item {
    pub fn single_value(&self, key: &String) -> Option<&String> {
        match self.props.find(key) {
            Some(x) => { if x.len() > 0 { Some(x[0].get_raw_value()) } else { None } },
            None => { None }
        }
    }

    pub fn all_values(&self, key: &String) -> Option<&Vec<PropertyValue>> {
        self.props.find(key)
    }
}


peg! parser(r#"
use super::{Item,PropertyValue};
use std::collections::HashMap;
use std::collections::hashmap::{Occupied, Vacant};

#[pub]

item -> Item
    = p:prop ++ eol {
        let mut rv = Item {
            props: HashMap::new()
        };

        for (k, v) in p.into_iter() {
            match rv.props.entry(k) {
                Occupied(values) => { values.into_mut().push(v); },
                Vacant(values) => { values.set(vec![v]); }
            };
        };
        rv
    }


prop -> (String, PropertyValue)
    = k:prop_name p:(";" p:prop_params {p})? ":" v:prop_value {
        print!("{} => {}\n", k, v);
        (k, PropertyValue {
            value: v,
            params: match p { Some(x) => x, None => "".to_string() }
        })
    }

prop_name -> String
    = name_char+ { match_str.into_string() }

prop_params -> String
    = prop_char+ { match_str.into_string() }

prop_value -> String
    = value_char+ { match_str.into_string() }

// Characters
name_char = ([a-zA-Z] / "-")
prop_char = name_char / [=;]
value_char = !eol .
eol = "\n" / "\r\n" / "\r" / "\u2028" / "\u2029"

"#)


pub fn parse_item(s: &String) -> Result<Item, String> {
    parser::item(s.as_slice())
}

pub fn parse_item_from_borrowed_string(s: &str) -> Result<Item, String> {
    parser::item(s)
}
