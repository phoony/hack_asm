use pest::iterators::Pair;

use crate::instructions::Label;

use super::Rule;

pub fn label(label: Pair<Rule>) -> Label {
    let label = label.into_inner().next().unwrap();

    Label {
        name: label.as_str(),
    }
}
