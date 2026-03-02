/// Application roles mapped from Keycloak realm roles.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum Role {
    Admin,
    #[default]
    Staff,
    Volunteer,
    ReadOnly,
}

impl Role {
    /// Convert from string representation.
    pub fn from_str_name(s: &str) -> Self {
        match s {
            "admin" => Role::Admin,
            "volunteer" => Role::Volunteer,
            "readonly" => Role::ReadOnly,
            _ => Role::Staff,
        }
    }

    /// Convert to string representation.
    pub fn as_str(&self) -> &'static str {
        match self {
            Role::Admin => "admin",
            Role::Staff => "staff",
            Role::Volunteer => "volunteer",
            Role::ReadOnly => "readonly",
        }
    }
}

impl std::fmt::Display for Role {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
