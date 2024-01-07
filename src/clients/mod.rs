mod errors;
pub mod indexer_client;
pub mod indexer_client_types;

// ========================================================
// Traits and macros for easier conversion for get/post
// ========================================================

trait CamelCaseify {
    fn to_camel_case(&self) -> Self;
}

impl CamelCaseify for String {
    fn to_camel_case(&self) -> Self {
        let mut convert_next = false;
        self.chars()
            .map(|c| {
                if c == '_' {
                    convert_next = true;
                    return '\0';
                }

                if convert_next {
                    convert_next = false;
                    c.to_ascii_uppercase()
                } else {
                    c
                }
            })
            .filter(|c| *c != '\0')
            .collect()
    }
}

#[macro_export]
macro_rules! arg_to_tuple {
    ($exp:expr) => {
        (
            stringify!($exp).to_string().to_camel_case(),
            Some($exp.to_string()),
        )
    };
}

#[macro_export]
macro_rules! option_to_tuple {
    ($exp:expr) => {
        (
            stringify!($exp).to_string().to_camel_case(),
            option_t_to_string_option!($exp),
        )
    };
}

#[macro_export]
macro_rules! option_t_to_string_option {
    ($exp:expr) => {
        if let Some(e) = $exp {
            Some(e.to_string())
        } else {
            None
        }
    };
}
