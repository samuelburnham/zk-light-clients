// Copyright (c) Yatima, Inc.
// SPDX-License-Identifier: Apache-2.0

use thiserror::Error;

/// Errors possible during type conversions.
#[derive(Debug, Error)]
pub enum MerkleError {
    #[error("Index not supported: {0}")]
    Index(usize),
    #[error("Error while computing the Merkle root: {source}")]
    Hash {
        #[source]
        source: Box<dyn std::error::Error + Sync + Send>,
    },
    #[error("Unexpected branch length received. Expected {expected}, got {actual}")]
    InvalidBranchLength { expected: usize, actual: usize },
    #[error("Unexpected generalized index for the depth. Generalized index {generalized_index} (depth {generalized_index_depth}) is not valid for depth {depth}")]
    InvalidGeneralizedIndex {
        depth: usize,
        generalized_index: usize,
        generalized_index_depth: u32,
    },
}