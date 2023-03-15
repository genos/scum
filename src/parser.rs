#[cfg(test)]
mod test {
    use crate::expression::{Expression, Identifier};
    use proptest::prelude::*;

    fn arb_identifier() -> impl Strategy<Value = Identifier> {
        "[^\\d\\s]\\S*".prop_map(Identifier)
    }

    fn arb_expression() -> impl Strategy<Value = Expression> {
        // https://docs.rs/proptest/latest/proptest/prelude/trait.Strategy.html#method.prop_recursive
        let leaf = prop_oneof![
            arb_identifier().prop_map(Expression::Atom),
            any::<bool>().prop_map(Expression::Bool),
            any::<i64>().prop_map(Expression::Int),
            any::<f64>().prop_map(Expression::Float),
        ];
        leaf.prop_recursive(
            4,  // No more than 4 branch levels deep
            64, // Target around 64 total elements
            16, // Each collection is up to 16 elements long
            |element| {
                prop_oneof![
                    prop::collection::vec(element.clone(), 0..16).prop_map(Expression::List),
                ]
            },
        )
    }

    proptest! {

        #[test]
        fn arb_id_ok(i in arb_identifier()) {
            prop_assert_eq!(i.clone(), i.clone())
        }

        #[test]
        fn arb_exp_ok(exp in arb_expression()) {
            prop_assert_eq!(exp.clone(), exp.clone())
        }

    }
}
