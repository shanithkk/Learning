#[derive(Debug)]
struct Authorization {
    action: String,
    condition: String,
    condition2: String,
}

impl Authorization {
    pub fn with_action(s:String) -> AuthorizationBuilder {
        AuthorizationBuilder::with_action(s)
    }
}

struct AuthorizationBuilder {
    action: Option<String>,
    condition: Option<String>,
    condition2: Option<String>,
}

impl Default for AuthorizationBuilder {
    fn default() -> Self {
        AuthorizationBuilder {
            action: None,
            condition: None,
            condition2: Some("Hello".into()),
        }
    }
}

impl AuthorizationBuilder {
    fn with_action(s: String) -> Self {
        AuthorizationBuilder {
            action: Some(s),
            condition: None,
            condition2: None,
        }
    }
    fn with_conditon(mut self, access_key: String) -> Self {
        self.condition = Some(access_key);
        self
    }

    fn with_condition2(mut self, secret_key: String) -> Self {
        self.condition2 = Some(secret_key);
        self
    }

    fn build(self) -> Authorization {
        Authorization {
            action: self.action.expect("Action key is required"),
            condition: self.condition.expect("Secret key is required"),
            condition2: self.condition2.unwrap_or_default(),
        }
    }
}

fn main() {
    let auth = Authorization::with_action("test".into())
        .with_conditon("condition1".to_string())
        .build();

    println!(" authorization: {:?}", auth);
}
