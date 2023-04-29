//! Utility macros

/// Easily create a vector of Boxed Card trait objects
///
/// For example:
/// ```
/// # use victoire_macros::card_vec;
/// # use victoire::cards::base::*;
/// # use victoire::types::*;
///
///
/// let cards = card_vec![Copper, Silver, Gold];
/// ```
#[macro_export]
macro_rules! card_vec {
    ( $( $card:expr ),* ) => {
        {
            let mut v: CardList = Vec::new();
            $(v.push(Box::new($card));)*

            v
        }
    };
}
