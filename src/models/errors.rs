#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphQlErrorLocation {
    pub line: i32,
    pub column: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphQLError {
    pub message: String,
    pub locations: Vec<GraphQlErrorLocation>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphQLErrors {
    pub errors: Vec<GraphQLError>,
}

impl GraphQLErrors {
    pub fn new(message: &str) -> GraphQLErrors {
        GraphQLErrors {
            errors: vec![GraphQLError {
                message: message.to_owned(),
                locations: Vec::new(),
            }],
        }
    }
}

impl From<diesel::result::Error> for GraphQLErrors {
    fn from(error: diesel::result::Error) -> Self {
        GraphQLErrors::new(&error.to_string())
    }
}

impl From<r2d2::Error> for GraphQLErrors {
    fn from(error: r2d2::Error) -> Self {
        GraphQLErrors::new(&error.to_string())
    }
}
