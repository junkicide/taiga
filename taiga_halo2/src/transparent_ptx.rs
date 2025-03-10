use crate::{
    error::TransactionError, executable::Executable, note::NoteCommitment, nullifier::Nullifier,
    value_commitment::ValueCommitment,
};
use pasta_curves::pallas;

#[derive(Debug, Clone)]
pub struct TransparentPartialTransaction {
    pub inputs: Vec<InputResource>,
    pub outputs: Vec<OutputResource>,
}

impl Executable for TransparentPartialTransaction {
    fn execute(&self) -> Result<(), TransactionError> {
        // TODO: figure out how transparent ptx executes
        unimplemented!()
    }

    fn get_nullifiers(&self) -> Vec<Nullifier> {
        unimplemented!()
    }

    fn get_output_cms(&self) -> Vec<NoteCommitment> {
        unimplemented!()
    }

    fn get_value_commitments(&self) -> Vec<ValueCommitment> {
        unimplemented!()
    }

    fn get_anchors(&self) -> Vec<pallas::Base> {
        unimplemented!()
    }
}

#[derive(Debug, Clone)]
pub struct InputResource {
    pub resource_logic: ResourceLogic,
    pub prefix: ContentHash,
    pub suffix: Vec<ContentHash>,
    pub resource_data_static: ResourceDataStatic,
    pub resource_data_dynamic: ResourceDataDynamic,
}

#[derive(Debug, Clone)]
pub struct OutputResource {
    pub resource_logic: ResourceLogic,
    pub resource_data_static: ResourceDataStatic,
    pub resource_data_dynamic: ResourceDataDynamic,
}

#[derive(Debug, Clone)]
pub struct ResourceLogic {}

#[derive(Debug, Clone)]
pub struct ContentHash {}

#[derive(Debug, Clone)]
pub struct ResourceDataStatic {}

#[derive(Debug, Clone)]
pub struct ResourceDataDynamic {}
