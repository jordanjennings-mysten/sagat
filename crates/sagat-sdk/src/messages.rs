// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

pub fn connect_message(expiry: &str) -> String {
    format!("Verifying address ownership until: {expiry}")
}

pub fn accept_multisig_invitation_message(multisig_address: &str) -> String {
    format!("Participating in multisig {multisig_address}")
}

pub fn reject_multisig_invitation_message(multisig_address: &str) -> String {
    format!("Rejecting multisig invitation {multisig_address}")
}

pub fn add_multisig_proposer_message(
    proposer: &str,
    multisig_address: &str,
    expiry: &str,
) -> String {
    format!("Adding proposer {proposer} to multisig {multisig_address}. Valid until: {expiry}")
}

pub fn remove_multisig_proposer_message(
    proposer: &str,
    multisig_address: &str,
    expiry: &str,
) -> String {
    format!("Removing proposer {proposer} from multisig {multisig_address}. Valid until: {expiry}")
}

pub fn cancel_proposal_message(proposal_id: u64) -> String {
    format!("Cancel proposal {proposal_id}")
}
