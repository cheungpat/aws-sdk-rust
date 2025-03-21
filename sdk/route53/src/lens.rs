// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn reflens_structure_crate_output_list_health_checks_output_next_marker(
    input: &crate::output::ListHealthChecksOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_marker {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_structure_crate_output_list_hosted_zones_output_next_marker(
    input: &crate::output::ListHostedZonesOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_marker {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_structure_crate_output_list_query_logging_configs_output_next_token(
    input: &crate::output::ListQueryLoggingConfigsOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_structure_crate_output_list_health_checks_output_health_checks(
    input: crate::output::ListHealthChecksOutput,
) -> std::option::Option<std::vec::Vec<crate::model::HealthCheck>> {
    let input = match input.health_checks {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_structure_crate_output_list_hosted_zones_output_hosted_zones(
    input: crate::output::ListHostedZonesOutput,
) -> std::option::Option<std::vec::Vec<crate::model::HostedZone>> {
    let input = match input.hosted_zones {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_structure_crate_output_list_query_logging_configs_output_query_logging_configs(
    input: crate::output::ListQueryLoggingConfigsOutput,
) -> std::option::Option<std::vec::Vec<crate::model::QueryLoggingConfig>> {
    let input = match input.query_logging_configs {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}
