// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use sagat_sdk::{
    accept_multisig_invitation_message, add_multisig_proposer_message, cancel_proposal_message,
    connect_message, reject_multisig_invitation_message, remove_multisig_proposer_message,
};

#[test]
fn connect_message_matches_typescript_sdk() {
    assert_eq!(
        connect_message("2026-05-29T12:00:00.000Z"),
        "Verifying address ownership until: 2026-05-29T12:00:00.000Z"
    );
}

#[test]
fn accept_multisig_invitation_message_matches_typescript_sdk() {
    assert_eq!(
        accept_multisig_invitation_message("0xabc"),
        "Participating in multisig 0xabc"
    );
}

#[test]
fn reject_multisig_invitation_message_matches_typescript_sdk() {
    assert_eq!(
        reject_multisig_invitation_message("0xabc"),
        "Rejecting multisig invitation 0xabc"
    );
}

#[test]
fn add_multisig_proposer_message_matches_typescript_sdk() {
    assert_eq!(
        add_multisig_proposer_message("0xproposer", "0xmultisig", "2026-05-29T12:00:00.000Z"),
        "Adding proposer 0xproposer to multisig 0xmultisig. Valid until: 2026-05-29T12:00:00.000Z"
    );
}

#[test]
fn remove_multisig_proposer_message_matches_typescript_sdk() {
    assert_eq!(
        remove_multisig_proposer_message("0xproposer", "0xmultisig", "2026-05-29T12:00:00.000Z"),
        "Removing proposer 0xproposer from multisig 0xmultisig. Valid until: 2026-05-29T12:00:00.000Z"
    );
}

#[test]
fn cancel_proposal_message_matches_typescript_sdk() {
    assert_eq!(cancel_proposal_message(42), "Cancel proposal 42");
}
