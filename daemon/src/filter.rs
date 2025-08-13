use aginsensors_core::connector::{ConnectorEvent, ConnectorEventBody};
use color_eyre::eyre::Result;

use crate::organizations::{Filter, Organization, OrganizationsState};

#[derive(Debug, Clone)]
pub struct FilteredConnectorEvent {
    pub body: ConnectorEventBody,
    pub organizations: Vec<Organization>,
}

pub fn filter(
    event: ConnectorEvent,
    organizations: &OrganizationsState,
) -> Result<FilteredConnectorEvent> {
    let matching_orgs: Vec<Organization> = organizations
        .organizations
        .values()
        .filter(|org| {
            if let Some(event_orgs) = &event.metadata.organizations
                && event.metadata.bucket.is_some()
            {
                if event_orgs.contains(&org.name) {
                    return true;
                }
            } else if let Some(mac) = &event.metadata.mac {
                if org.filters.iter().any(|f| match f {
                    Filter::MacFilter(mac_filter) => mac_filter.macs.contains(mac),
                    _ => false,
                }) {
                    return true;
                }
            } else if let Some(auth_token) = &event.metadata.auth_token {
                if org.filters.iter().any(|f| match f {
                    Filter::TokenFilter(token_filter) => token_filter.tokens.contains(auth_token),
                    _ => false,
                }) {
                    return true;
                }
            }
            false
        })
        .cloned()
        .collect();

    if matching_orgs.is_empty() {
        Err(color_eyre::eyre::eyre!(
            "Couldn't find organization for event",
        ))
    } else {
        Ok(FilteredConnectorEvent {
            body: event.body.clone(),
            organizations: matching_orgs,
        })
    }
}
