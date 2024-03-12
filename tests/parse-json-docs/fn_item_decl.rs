use crate::{doc, snap};
use term_rustdoc::decl::fn_item;

#[test]
fn fn_items() {
    let map = &doc();
    let dmod = map.dmodule();
    let mut fns_str = Vec::new();
    for fn_ in &dmod.functions {
        fns_str.push(fn_item(&fn_.id, map));
    }
    // for struct_ in &dmod.structs {
    //     for inh in &*struct_.impls.merged_inherent.functions {
    //         fns_str.push(fn_item(&inh.id, map));
    //     }
    // }
    // for enum_ in &dmod.enums {
    //     for inh in &*enum_.impls.merged_inherent.functions {
    //         fns_str.push(fn_item(&inh.id, map));
    //     }
    // }
    // for union_ in &dmod.unions {
    //     for inh in &*union_.impls.merged_inherent.functions {
    //         fns_str.push(fn_item(&inh.id, map));
    //     }
    // }
    // for trait_ in &dmod.traits {
    //     for fn_ in &*trait_.functions {
    //         fns_str.push(fn_item(&fn_.id, map));
    //     }
    // }
    snap!(fns_str, @r###"
    [
        "pub func_dyn_trait(d: &dyn  ATrait +  Send +  Sync) -> &dyn  ATrait",
        "pub func_with_1arg(_: FieldsNamedStruct)",
        "pub func_with_1arg_and_ret(f: FieldsNamedStruct) -> AUnitEnum",
        "pub func_with_no_args()",
    ]
    "###);
}
