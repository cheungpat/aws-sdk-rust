// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct GetHomeRegionOutput {
    /// <p>The name of the home region of the calling account.</p>
    pub home_region: std::option::Option<std::string::String>,
}
impl GetHomeRegionOutput {
    /// <p>The name of the home region of the calling account.</p>
    pub fn home_region(&self) -> std::option::Option<&str> {
        self.home_region.as_deref()
    }
}
impl std::fmt::Debug for GetHomeRegionOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("GetHomeRegionOutput");
        formatter.field("home_region", &self.home_region);
        formatter.finish()
    }
}
/// See [`GetHomeRegionOutput`](crate::output::GetHomeRegionOutput)
pub mod get_home_region_output {
    /// A builder for [`GetHomeRegionOutput`](crate::output::GetHomeRegionOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) home_region: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>The name of the home region of the calling account.</p>
        pub fn home_region(mut self, input: impl Into<std::string::String>) -> Self {
            self.home_region = Some(input.into());
            self
        }
        /// <p>The name of the home region of the calling account.</p>
        pub fn set_home_region(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.home_region = input;
            self
        }
        /// Consumes the builder and constructs a [`GetHomeRegionOutput`](crate::output::GetHomeRegionOutput)
        pub fn build(self) -> crate::output::GetHomeRegionOutput {
            crate::output::GetHomeRegionOutput {
                home_region: self.home_region,
            }
        }
    }
}
impl GetHomeRegionOutput {
    /// Creates a new builder-style object to manufacture [`GetHomeRegionOutput`](crate::output::GetHomeRegionOutput)
    pub fn builder() -> crate::output::get_home_region_output::Builder {
        crate::output::get_home_region_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct DescribeHomeRegionControlsOutput {
    /// <p>An array that contains your <code>HomeRegionControl</code> objects.</p>
    pub home_region_controls: std::option::Option<std::vec::Vec<crate::model::HomeRegionControl>>,
    /// <p>If a <code>NextToken</code> was returned by a previous call, more results are available. To retrieve the next page of results, make the call again using the returned token in <code>NextToken</code>.</p>
    pub next_token: std::option::Option<std::string::String>,
}
impl DescribeHomeRegionControlsOutput {
    /// <p>An array that contains your <code>HomeRegionControl</code> objects.</p>
    pub fn home_region_controls(&self) -> std::option::Option<&[crate::model::HomeRegionControl]> {
        self.home_region_controls.as_deref()
    }
    /// <p>If a <code>NextToken</code> was returned by a previous call, more results are available. To retrieve the next page of results, make the call again using the returned token in <code>NextToken</code>.</p>
    pub fn next_token(&self) -> std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl std::fmt::Debug for DescribeHomeRegionControlsOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("DescribeHomeRegionControlsOutput");
        formatter.field("home_region_controls", &self.home_region_controls);
        formatter.field("next_token", &self.next_token);
        formatter.finish()
    }
}
/// See [`DescribeHomeRegionControlsOutput`](crate::output::DescribeHomeRegionControlsOutput)
pub mod describe_home_region_controls_output {
    /// A builder for [`DescribeHomeRegionControlsOutput`](crate::output::DescribeHomeRegionControlsOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) home_region_controls:
            std::option::Option<std::vec::Vec<crate::model::HomeRegionControl>>,
        pub(crate) next_token: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// Appends an item to `home_region_controls`.
        ///
        /// To override the contents of this collection use [`set_home_region_controls`](Self::set_home_region_controls).
        ///
        /// <p>An array that contains your <code>HomeRegionControl</code> objects.</p>
        pub fn home_region_controls(mut self, input: crate::model::HomeRegionControl) -> Self {
            let mut v = self.home_region_controls.unwrap_or_default();
            v.push(input);
            self.home_region_controls = Some(v);
            self
        }
        /// <p>An array that contains your <code>HomeRegionControl</code> objects.</p>
        pub fn set_home_region_controls(
            mut self,
            input: std::option::Option<std::vec::Vec<crate::model::HomeRegionControl>>,
        ) -> Self {
            self.home_region_controls = input;
            self
        }
        /// <p>If a <code>NextToken</code> was returned by a previous call, more results are available. To retrieve the next page of results, make the call again using the returned token in <code>NextToken</code>.</p>
        pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.next_token = Some(input.into());
            self
        }
        /// <p>If a <code>NextToken</code> was returned by a previous call, more results are available. To retrieve the next page of results, make the call again using the returned token in <code>NextToken</code>.</p>
        pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.next_token = input;
            self
        }
        /// Consumes the builder and constructs a [`DescribeHomeRegionControlsOutput`](crate::output::DescribeHomeRegionControlsOutput)
        pub fn build(self) -> crate::output::DescribeHomeRegionControlsOutput {
            crate::output::DescribeHomeRegionControlsOutput {
                home_region_controls: self.home_region_controls,
                next_token: self.next_token,
            }
        }
    }
}
impl DescribeHomeRegionControlsOutput {
    /// Creates a new builder-style object to manufacture [`DescribeHomeRegionControlsOutput`](crate::output::DescribeHomeRegionControlsOutput)
    pub fn builder() -> crate::output::describe_home_region_controls_output::Builder {
        crate::output::describe_home_region_controls_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct CreateHomeRegionControlOutput {
    /// <p>This object is the <code>HomeRegionControl</code> object that's returned by a successful call to <code>CreateHomeRegionControl</code>.</p>
    pub home_region_control: std::option::Option<crate::model::HomeRegionControl>,
}
impl CreateHomeRegionControlOutput {
    /// <p>This object is the <code>HomeRegionControl</code> object that's returned by a successful call to <code>CreateHomeRegionControl</code>.</p>
    pub fn home_region_control(&self) -> std::option::Option<&crate::model::HomeRegionControl> {
        self.home_region_control.as_ref()
    }
}
impl std::fmt::Debug for CreateHomeRegionControlOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("CreateHomeRegionControlOutput");
        formatter.field("home_region_control", &self.home_region_control);
        formatter.finish()
    }
}
/// See [`CreateHomeRegionControlOutput`](crate::output::CreateHomeRegionControlOutput)
pub mod create_home_region_control_output {
    /// A builder for [`CreateHomeRegionControlOutput`](crate::output::CreateHomeRegionControlOutput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) home_region_control: std::option::Option<crate::model::HomeRegionControl>,
    }
    impl Builder {
        /// <p>This object is the <code>HomeRegionControl</code> object that's returned by a successful call to <code>CreateHomeRegionControl</code>.</p>
        pub fn home_region_control(mut self, input: crate::model::HomeRegionControl) -> Self {
            self.home_region_control = Some(input);
            self
        }
        /// <p>This object is the <code>HomeRegionControl</code> object that's returned by a successful call to <code>CreateHomeRegionControl</code>.</p>
        pub fn set_home_region_control(
            mut self,
            input: std::option::Option<crate::model::HomeRegionControl>,
        ) -> Self {
            self.home_region_control = input;
            self
        }
        /// Consumes the builder and constructs a [`CreateHomeRegionControlOutput`](crate::output::CreateHomeRegionControlOutput)
        pub fn build(self) -> crate::output::CreateHomeRegionControlOutput {
            crate::output::CreateHomeRegionControlOutput {
                home_region_control: self.home_region_control,
            }
        }
    }
}
impl CreateHomeRegionControlOutput {
    /// Creates a new builder-style object to manufacture [`CreateHomeRegionControlOutput`](crate::output::CreateHomeRegionControlOutput)
    pub fn builder() -> crate::output::create_home_region_control_output::Builder {
        crate::output::create_home_region_control_output::Builder::default()
    }
}
