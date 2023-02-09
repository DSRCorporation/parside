use crate::error::ParsideResult;
use crate::message::cold_code::ColdCodes;
use crate::message::groups::parsers::Parsers;
use cesride::counter::Codex;
use cesride::{Counter, Matter};
use nom::multi::count;
use nom::sequence::tuple;

#[derive(Debug, Clone, Default)]
pub struct SealSourceCouples {
    pub value: Vec<SealSourceCouple>,
}

impl SealSourceCouples {
    pub const CODE: Codex = Codex::SealSourceCouples;

    pub fn new(value: Vec<SealSourceCouple>) -> Self {
        Self { value }
    }

    pub(crate) fn from_stream_bytes<'a>(
        bytes: &'a [u8],
        counter: &Counter,
        cold_code: &ColdCodes,
    ) -> ParsideResult<(&'a [u8], SealSourceCouples)> {
        let (rest, body) = count(
            tuple((
                Parsers::matter_parser(cold_code)?,
                Parsers::matter_parser(cold_code)?,
            )),
            counter.count() as usize,
        )(bytes)?;
        let body = body
            .into_iter()
            .map(|(seqner, saider)| SealSourceCouple { seqner, saider })
            .collect();

        return Ok((rest, SealSourceCouples { value: body }));
    }
}

#[derive(Debug, Clone, Default)]
pub struct SealSourceCouple {
    pub seqner: Matter,
    pub saider: Matter,
}

impl SealSourceCouple {
    pub fn new(seqner: Matter, saider: Matter) -> Self {
        Self { seqner, saider }
    }
}
