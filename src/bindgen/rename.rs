use std::str::FromStr;

/// The type of identifier to be renamed.
#[derive(Debug, Clone, Copy)]
pub enum IdentifierType {
    StructMember,
    EnumVariant,
    FunctionArg,
}
impl IdentifierType {
    fn to_str(&self) -> &'static str {
        match *self {
            IdentifierType::StructMember => "m",
            IdentifierType::EnumVariant => "",
            IdentifierType::FunctionArg => "a",
        }
    }
}

/// A rule to apply to an identifier when generating bindings.
#[derive(Debug, Clone, Copy)]
pub enum RenameRule {
    /// Do not apply any renaming. The default.
    None,
    /// Converts the identifier to PascalCase and adds a prefix based on where the identifier is used.
    GeckoCase,
    /// Converts the identifier to lower case.
    LowerCase,
    /// Converts the identifier to upper case.
    UpperCase,
    /// Converts the identifier to PascalCase.
    PascalCase,
    /// Converts the identifier to camelCase.
    CamelCase,
    /// Converts the identifier to snake_case.
    SnakeCase,
    /// Converts the identifier to SCREAMING_SNAKE_CASE.
    ScreamingSnakeCase,
}

impl RenameRule {
    /// Applies the rename rule to a string that is formatted in PascalCase.
    pub fn apply_to_pascal_case(&self, text: &str, context: IdentifierType) -> String {
        if text.len() == 0 {
            return String::new();
        }

        match *self {
            RenameRule::None => String::from(text),
            RenameRule::GeckoCase => context.to_str().to_owned() + text,
            RenameRule::LowerCase => text.to_lowercase(),
            RenameRule::UpperCase => text.to_uppercase(),
            RenameRule::PascalCase => text.to_owned(),
            RenameRule::CamelCase => {
                text[..1].to_lowercase() + &text[1..]
            }
            RenameRule::SnakeCase => {
                let mut result = String::new();
                for (i, c) in text.char_indices() {
                    if c.is_uppercase() && i != 0 {
                        result.push_str("_");
                    }
                    for x in c.to_lowercase() {
                        result.push(x);
                    }
                }
                result
            }
            RenameRule::ScreamingSnakeCase => {
                // Same as SnakeCase code above, but uses to_uppercase
                let mut result = String::new();
                for (i, c) in text.char_indices() {
                    if c.is_uppercase() && i != 0 {
                        result.push_str("_");
                    }
                    for x in c.to_uppercase() {
                        result.push(x);
                    }
                }
                result
            }
        }
    }

    /// Applies the rename rule to a string that is formatted in snake_case.
    pub fn apply_to_snake_case(&self, mut text: &str, context: IdentifierType) -> String {
        if text.len() == 0 {
            return String::new();
        }

        match *self {
            RenameRule::None => String::from(text),
            RenameRule::GeckoCase => {
                if &text[..1] == "_" {
                    text = &text[1..];
                }

                context.to_str().to_owned() +
                    &RenameRule::PascalCase.apply_to_snake_case(text, context)
            }
            RenameRule::LowerCase => text.to_lowercase(),
            RenameRule::UpperCase => text.to_uppercase(),
            RenameRule::PascalCase => {
                let mut result = String::new();
                let mut is_uppercase = true;
                for c in text.chars() {
                    if c == '_' {
                        is_uppercase = true;
                        continue;
                    }

                    if is_uppercase {
                        for x in c.to_uppercase() {
                            result.push(x);
                        }
                        is_uppercase = false;
                    } else {
                        result.push(c);
                    }
                }
                result
            }
            RenameRule::CamelCase => {
                // Same as PascalCase code above, but is_uppercase = false to start
                let mut result = String::new();
                let mut is_uppercase = false;
                for c in text.chars() {
                    if c == '_' {
                        is_uppercase = true;
                        continue;
                    }

                    if is_uppercase {
                        for x in c.to_uppercase() {
                            result.push(x);
                        }
                        is_uppercase = false;
                    } else {
                        result.push(c);
                    }
                }
                result
            }
            RenameRule::SnakeCase => text.to_owned(),
            RenameRule::ScreamingSnakeCase => text.to_owned().to_uppercase(),
        }
    }
}

impl Default for RenameRule {
    fn default() -> RenameRule {
        RenameRule::None
    }
}

impl FromStr for RenameRule {
    type Err = String;

    fn from_str(s: &str) -> Result<RenameRule, Self::Err> {
        match s {
            "none" => Ok(RenameRule::None),
            "None" => Ok(RenameRule::None),

            "mGeckoCase" => Ok(RenameRule::GeckoCase),
            "GeckoCase" => Ok(RenameRule::GeckoCase),
            "gecko_case" => Ok(RenameRule::GeckoCase),

            "lowercase" => Ok(RenameRule::LowerCase),
            "LowerCase" => Ok(RenameRule::LowerCase),
            "lower_case" => Ok(RenameRule::LowerCase),

            "UPPERCASE" => Ok(RenameRule::UpperCase),
            "UpperCase" => Ok(RenameRule::UpperCase),
            "upper_case" => Ok(RenameRule::UpperCase),

            "PascalCase" => Ok(RenameRule::PascalCase),
            "pascal_case" => Ok(RenameRule::PascalCase),

            "camelCase" => Ok(RenameRule::CamelCase),
            "CamelCase" => Ok(RenameRule::CamelCase),
            "camel_case" => Ok(RenameRule::CamelCase),

            "snake_case" => Ok(RenameRule::SnakeCase),
            "SnakeCase" => Ok(RenameRule::SnakeCase),

            "SCREAMING_SNAKE_CASE" => Ok(RenameRule::ScreamingSnakeCase),
            "ScreamingSnakeCase" => Ok(RenameRule::ScreamingSnakeCase),
            "screaming_snake_case" => Ok(RenameRule::ScreamingSnakeCase),

            _ => Err(format!("unrecognized RenameRule: '{}'", s)),
        }
    }
}
deserialize_enum_str!(RenameRule);
