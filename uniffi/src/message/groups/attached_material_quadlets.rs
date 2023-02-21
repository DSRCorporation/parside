use parside::error::ParsideResult;
pub use parside::message::groups::{AttachedMaterialQuadlet, AttachedMaterialQuadlets};
use parside::Group;
use parside::Siger;

pub fn attached_material_quadlet_create(siger: Siger) -> AttachedMaterialQuadlet {
    AttachedMaterialQuadlet::new(siger)
}

pub fn attached_material_quadlets_create(
    value: Vec<AttachedMaterialQuadlet>,
) -> AttachedMaterialQuadlets {
    AttachedMaterialQuadlets::new(value)
}

pub fn attached_material_quadlets_qb64(
    attached_material_quadlets: &AttachedMaterialQuadlets,
) -> ParsideResult<String> {
    attached_material_quadlets.qb64()
}

pub fn attached_material_quadlets_qb64b(
    attached_material_quadlets: &AttachedMaterialQuadlets,
) -> ParsideResult<Vec<u8>> {
    attached_material_quadlets.qb64b()
}

pub fn attached_material_quadlets_qb2(
    attached_material_quadlets: &AttachedMaterialQuadlets,
) -> ParsideResult<Vec<u8>> {
    attached_material_quadlets.qb2()
}
