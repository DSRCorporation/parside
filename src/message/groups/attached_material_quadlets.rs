use crate::error::{ParsideError, ParsideResult};
use crate::message::cold_code::ColdCode;
use crate::message::{Group, GroupItem};
use cesride::counter::Codex;
use cesride::{Counter, Siger, Indexer};


// FIXME: Implement proper definition
#[derive(Debug, Clone, Default)]
pub struct AttachedMaterialQuadlets {
    pub value: Vec<AttachedMaterialQuadlet>,
}

impl Group<AttachedMaterialQuadlet> for AttachedMaterialQuadlets {
    const CODE: &'static str = Codex::AttachedMaterialQuadlets;

    fn new(value: Vec<AttachedMaterialQuadlet>) -> Self {
        Self { value }
    }

    fn value(&self) -> &Vec<AttachedMaterialQuadlet> {
        &self.value
    }
}

impl AttachedMaterialQuadlets {
    pub(crate) fn from_stream_bytes<'a>(
        _bytes: &'a [u8],
        _counter: &Counter,
        _cold_code: &ColdCode,
    ) -> ParsideResult<(&'a [u8], AttachedMaterialQuadlets)> {
        unimplemented!();
    }
}

#[derive(Debug, Clone, Default)]
pub struct AttachedMaterialQuadlet {
    pub siger: Siger
}

impl AttachedMaterialQuadlet {
    pub fn new(siger: Siger) -> Self {
        Self { siger }
    }
}


impl GroupItem for AttachedMaterialQuadlet {
    fn qb64(&self) -> ParsideResult<String> {
        self.siger.qb64().map_err(ParsideError::from)
    }

    fn qb64b(&self) -> ParsideResult<Vec<u8>> {
        self.siger.qb64b().map_err(ParsideError::from)
    }

    fn qb2(&self) -> ParsideResult<Vec<u8>> {
        self.siger.qb2().map_err(ParsideError::from)
    }
}