// Code generated by the dharitri-sc multi-contract system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                            8
// Async Callback (empty):               1
// Total number of exported functions:  10

#![no_std]
#![allow(internal_features)]
#![feature(lang_items)]

dharitri_sc_wasm_adapter::allocator!();
dharitri_sc_wasm_adapter::panic_handler!();

dharitri_sc_wasm_adapter::external_view_init! {}

dharitri_sc_wasm_adapter::external_view_endpoints! {
    multisig
    (
        getPendingActionFullInfo => get_pending_action_full_info
        userRole => user_role
        getAllBoardMembers => get_all_board_members
        getAllProposers => get_all_proposers
        getActionData => get_action_data
        getActionSigners => get_action_signers
        getActionSignerCount => get_action_signer_count
        getActionValidSignerCount => get_action_valid_signer_count
    )
}

dharitri_sc_wasm_adapter::async_callback_empty! {}
