use std::marker::PhantomData;

use archway_sdk::custom::query::ArchwayQuery;
use cosmwasm_std::{
    testing::{MockApi, MockQuerier, MockStorage},
    OwnedDeps,
};

pub type ArchwayDeps = OwnedDeps<MockStorage, MockApi, MockQuerier, ArchwayQuery>;

pub fn mock_deps_archway() -> ArchwayDeps {
    OwnedDeps {
        storage: MockStorage::default(),
        api: MockApi::default(),
        querier: MockQuerier::default(),
        custom_query_type: PhantomData,
    }
}
