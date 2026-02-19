use serde::{Deserialize, Serialize};

mod common;
mod v1;
mod v2;

// re-export types
pub use self::{common::*, v1::*, v2::*};

/// A VPC Lattice request that can be either V1 or V2 format.
/// Deserialization tries V2 first (which has a `version` and `requestContext` field),
/// then falls back to V1.
#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum VpcLatticeRequest {
    V2(VpcLatticeRequestV2),
    V1(VpcLatticeRequestV1),
}
