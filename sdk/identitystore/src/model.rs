// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(
    std::clone::Clone,
    std::cmp::Eq,
    std::cmp::Ord,
    std::cmp::PartialEq,
    std::cmp::PartialOrd,
    std::fmt::Debug,
    std::hash::Hash,
)]
pub enum ResourceType {
    #[allow(missing_docs)] // documentation missing in model
    Group,
    #[allow(missing_docs)] // documentation missing in model
    IdentityStore,
    #[allow(missing_docs)] // documentation missing in model
    User,
    /// Unknown contains new variants that have been added since this code was generated.
    Unknown(String),
}
impl std::convert::From<&str> for ResourceType {
    fn from(s: &str) -> Self {
        match s {
            "GROUP" => ResourceType::Group,
            "IDENTITY_STORE" => ResourceType::IdentityStore,
            "USER" => ResourceType::User,
            other => ResourceType::Unknown(other.to_owned()),
        }
    }
}
impl std::str::FromStr for ResourceType {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(ResourceType::from(s))
    }
}
impl ResourceType {
    /// Returns the `&str` value of the enum member.
    pub fn as_str(&self) -> &str {
        match self {
            ResourceType::Group => "GROUP",
            ResourceType::IdentityStore => "IDENTITY_STORE",
            ResourceType::User => "USER",
            ResourceType::Unknown(s) => s.as_ref(),
        }
    }
    /// Returns all the `&str` values of the enum members.
    pub fn values() -> &'static [&'static str] {
        &["GROUP", "IDENTITY_STORE", "USER"]
    }
}
impl AsRef<str> for ResourceType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

/// <p>A user object, which contains a specified user’s metadata and attributes.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct User {
    /// <p>Contains the user’s user name value. The length limit is 128 characters. This value can consist of letters, accented characters, symbols, numbers, and punctuation. The characters <code>&lt;&gt;;:%</code> are excluded. This value is specified at the time the user is created and stored as an attribute of the user object in the identity store.</p>
    pub user_name: std::option::Option<std::string::String>,
    /// <p>The identifier for a user in the identity store.</p>
    pub user_id: std::option::Option<std::string::String>,
}
impl User {
    /// <p>Contains the user’s user name value. The length limit is 128 characters. This value can consist of letters, accented characters, symbols, numbers, and punctuation. The characters <code>&lt;&gt;;:%</code> are excluded. This value is specified at the time the user is created and stored as an attribute of the user object in the identity store.</p>
    pub fn user_name(&self) -> std::option::Option<&str> {
        self.user_name.as_deref()
    }
    /// <p>The identifier for a user in the identity store.</p>
    pub fn user_id(&self) -> std::option::Option<&str> {
        self.user_id.as_deref()
    }
}
impl std::fmt::Debug for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("User");
        formatter.field("user_name", &"*** Sensitive Data Redacted ***");
        formatter.field("user_id", &self.user_id);
        formatter.finish()
    }
}
/// See [`User`](crate::model::User)
pub mod user {
    /// A builder for [`User`](crate::model::User)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) user_name: std::option::Option<std::string::String>,
        pub(crate) user_id: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>Contains the user’s user name value. The length limit is 128 characters. This value can consist of letters, accented characters, symbols, numbers, and punctuation. The characters <code>&lt;&gt;;:%</code> are excluded. This value is specified at the time the user is created and stored as an attribute of the user object in the identity store.</p>
        pub fn user_name(mut self, input: impl Into<std::string::String>) -> Self {
            self.user_name = Some(input.into());
            self
        }
        /// <p>Contains the user’s user name value. The length limit is 128 characters. This value can consist of letters, accented characters, symbols, numbers, and punctuation. The characters <code>&lt;&gt;;:%</code> are excluded. This value is specified at the time the user is created and stored as an attribute of the user object in the identity store.</p>
        pub fn set_user_name(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.user_name = input;
            self
        }
        /// <p>The identifier for a user in the identity store.</p>
        pub fn user_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.user_id = Some(input.into());
            self
        }
        /// <p>The identifier for a user in the identity store.</p>
        pub fn set_user_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.user_id = input;
            self
        }
        /// Consumes the builder and constructs a [`User`](crate::model::User)
        pub fn build(self) -> crate::model::User {
            crate::model::User {
                user_name: self.user_name,
                user_id: self.user_id,
            }
        }
    }
}
impl User {
    /// Creates a new builder-style object to manufacture [`User`](crate::model::User)
    pub fn builder() -> crate::model::user::Builder {
        crate::model::user::Builder::default()
    }
}

/// <p>A query filter used by <code>ListUsers</code> and <code>ListGroup</code>. This filter object provides the attribute name and attribute value to search users or groups.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct Filter {
    /// <p>The attribute path that is used to specify which attribute name to search. Length limit is 255 characters. For example, <code>UserName</code> is a valid attribute path for the <code>ListUsers</code> API, and <code>DisplayName</code> is a valid attribute path for the <code>ListGroups</code> API.</p>
    pub attribute_path: std::option::Option<std::string::String>,
    /// <p>Represents the data for an attribute. Each attribute value is described as a name-value pair. </p>
    pub attribute_value: std::option::Option<std::string::String>,
}
impl Filter {
    /// <p>The attribute path that is used to specify which attribute name to search. Length limit is 255 characters. For example, <code>UserName</code> is a valid attribute path for the <code>ListUsers</code> API, and <code>DisplayName</code> is a valid attribute path for the <code>ListGroups</code> API.</p>
    pub fn attribute_path(&self) -> std::option::Option<&str> {
        self.attribute_path.as_deref()
    }
    /// <p>Represents the data for an attribute. Each attribute value is described as a name-value pair. </p>
    pub fn attribute_value(&self) -> std::option::Option<&str> {
        self.attribute_value.as_deref()
    }
}
impl std::fmt::Debug for Filter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("Filter");
        formatter.field("attribute_path", &self.attribute_path);
        formatter.field("attribute_value", &"*** Sensitive Data Redacted ***");
        formatter.finish()
    }
}
/// See [`Filter`](crate::model::Filter)
pub mod filter {
    /// A builder for [`Filter`](crate::model::Filter)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) attribute_path: std::option::Option<std::string::String>,
        pub(crate) attribute_value: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>The attribute path that is used to specify which attribute name to search. Length limit is 255 characters. For example, <code>UserName</code> is a valid attribute path for the <code>ListUsers</code> API, and <code>DisplayName</code> is a valid attribute path for the <code>ListGroups</code> API.</p>
        pub fn attribute_path(mut self, input: impl Into<std::string::String>) -> Self {
            self.attribute_path = Some(input.into());
            self
        }
        /// <p>The attribute path that is used to specify which attribute name to search. Length limit is 255 characters. For example, <code>UserName</code> is a valid attribute path for the <code>ListUsers</code> API, and <code>DisplayName</code> is a valid attribute path for the <code>ListGroups</code> API.</p>
        pub fn set_attribute_path(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.attribute_path = input;
            self
        }
        /// <p>Represents the data for an attribute. Each attribute value is described as a name-value pair. </p>
        pub fn attribute_value(mut self, input: impl Into<std::string::String>) -> Self {
            self.attribute_value = Some(input.into());
            self
        }
        /// <p>Represents the data for an attribute. Each attribute value is described as a name-value pair. </p>
        pub fn set_attribute_value(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.attribute_value = input;
            self
        }
        /// Consumes the builder and constructs a [`Filter`](crate::model::Filter)
        pub fn build(self) -> crate::model::Filter {
            crate::model::Filter {
                attribute_path: self.attribute_path,
                attribute_value: self.attribute_value,
            }
        }
    }
}
impl Filter {
    /// Creates a new builder-style object to manufacture [`Filter`](crate::model::Filter)
    pub fn builder() -> crate::model::filter::Builder {
        crate::model::filter::Builder::default()
    }
}

/// <p>A group object, which contains a specified group’s metadata and attributes.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct Group {
    /// <p>The identifier for a group in the identity store.</p>
    pub group_id: std::option::Option<std::string::String>,
    /// <p>Contains the group’s display name value. The length limit is 1,024 characters. This value can consist of letters, accented characters, symbols, numbers, punctuation, tab, new line, carriage return, space, and nonbreaking space in this attribute. The characters <code>&lt;&gt;;:%</code> are excluded. This value is specified at the time the group is created and stored as an attribute of the group object in the identity store.</p>
    pub display_name: std::option::Option<std::string::String>,
}
impl Group {
    /// <p>The identifier for a group in the identity store.</p>
    pub fn group_id(&self) -> std::option::Option<&str> {
        self.group_id.as_deref()
    }
    /// <p>Contains the group’s display name value. The length limit is 1,024 characters. This value can consist of letters, accented characters, symbols, numbers, punctuation, tab, new line, carriage return, space, and nonbreaking space in this attribute. The characters <code>&lt;&gt;;:%</code> are excluded. This value is specified at the time the group is created and stored as an attribute of the group object in the identity store.</p>
    pub fn display_name(&self) -> std::option::Option<&str> {
        self.display_name.as_deref()
    }
}
impl std::fmt::Debug for Group {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("Group");
        formatter.field("group_id", &self.group_id);
        formatter.field("display_name", &self.display_name);
        formatter.finish()
    }
}
/// See [`Group`](crate::model::Group)
pub mod group {
    /// A builder for [`Group`](crate::model::Group)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) group_id: std::option::Option<std::string::String>,
        pub(crate) display_name: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>The identifier for a group in the identity store.</p>
        pub fn group_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.group_id = Some(input.into());
            self
        }
        /// <p>The identifier for a group in the identity store.</p>
        pub fn set_group_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.group_id = input;
            self
        }
        /// <p>Contains the group’s display name value. The length limit is 1,024 characters. This value can consist of letters, accented characters, symbols, numbers, punctuation, tab, new line, carriage return, space, and nonbreaking space in this attribute. The characters <code>&lt;&gt;;:%</code> are excluded. This value is specified at the time the group is created and stored as an attribute of the group object in the identity store.</p>
        pub fn display_name(mut self, input: impl Into<std::string::String>) -> Self {
            self.display_name = Some(input.into());
            self
        }
        /// <p>Contains the group’s display name value. The length limit is 1,024 characters. This value can consist of letters, accented characters, symbols, numbers, punctuation, tab, new line, carriage return, space, and nonbreaking space in this attribute. The characters <code>&lt;&gt;;:%</code> are excluded. This value is specified at the time the group is created and stored as an attribute of the group object in the identity store.</p>
        pub fn set_display_name(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.display_name = input;
            self
        }
        /// Consumes the builder and constructs a [`Group`](crate::model::Group)
        pub fn build(self) -> crate::model::Group {
            crate::model::Group {
                group_id: self.group_id,
                display_name: self.display_name,
            }
        }
    }
}
impl Group {
    /// Creates a new builder-style object to manufacture [`Group`](crate::model::Group)
    pub fn builder() -> crate::model::group::Builder {
        crate::model::group::Builder::default()
    }
}
