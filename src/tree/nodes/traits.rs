use crate::tree::{
    impls::show::{show_ids, DocTree, Show},
    IDMap, IDs, IdToID, SliceToIds, Tag, ID,
};
use rustdoc_types::{ItemEnum, Trait};

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct DTrait {
    pub id: ID,
    pub types: IDs,
    pub constants: IDs,
    pub functions: IDs,
    pub implementations: IDs,
}
impl DTrait {
    pub fn new(id: ID, item: &Trait, map: &IDMap) -> Self {
        let [mut types, mut constants, mut functions]: [Vec<ID>; 3] = Default::default();
        let trait_id = &id;
        let indexmap = &map.indexmap();
        for id in &item.items {
            if let Some(assoc) = indexmap.get(id) {
                let id = id.to_ID(); // id == assoc.id
                match &assoc.inner {
                    ItemEnum::AssocType { .. } => types.push(id),
                    ItemEnum::AssocConst { .. } => constants.push(id),
                    ItemEnum::Function(_) => functions.push(id),
                    _ => warn!(
                        "`{id}` should refer to an associated item \
                         (type/constant/function) in Trait `{trait_id}`"
                    ),
                }
            } else {
                warn!("the trait item {id:?} not found in Crate's index");
            }
        }
        DTrait {
            id,
            types: types.into(),
            constants: constants.into(),
            functions: functions.into(),
            implementations: item.implementations.to_ids(),
        }
    }

    /// External items need external crates compiled to know details,
    /// and the ID here is for PathMap, not IndexMap.
    pub fn new_external(id: ID) -> Self {
        let (types, constants, functions, implementations) = Default::default();
        DTrait {
            id,
            types,
            constants,
            functions,
            implementations,
        }
    }
}

impl Show for DTrait {
    fn show(&self) -> DocTree {
        format!("[trait] {}", self.id).show().with_leaves([
            "Associated Types".show().with_leaves(show_ids(&self.types)),
            "Associated Constants"
                .show()
                .with_leaves(show_ids(&self.constants)),
            "Associated Functions"
                .show()
                .with_leaves(show_ids(&self.functions)),
            "Implementors"
                .show()
                .with_leaves(show_ids(&self.implementations)),
        ])
    }

    fn show_prettier(&self, map: &IDMap) -> DocTree {
        let root = node!(Trait: map, &self.id);
        let leaves = names_node!(
            self map root.with_leaves([Tag::NoAssocOrImpls.show()]),
            AssocTypes   types     AssocType,
            AssocConsts  constants AssocConst,
            AssocFns     functions AssocFn,
            Implementors implementations Implementor,
        );
        root.with_leaves(leaves)
    }
}
