use crate::{shot, snap, INTEGRATION};
use similar_asserts::assert_eq;
use term_rustdoc::tree::{DModule, IDMap, Show, TreeLines};

#[test]
fn parse_module() {
    let doc = &INTEGRATION.doc;
    let dmod = DModule::new(doc);
    snap!("DModule", dmod);
    shot!("show-id", dmod.show());

    let idmap = IDMap::from_crate(doc);
    let tree = dmod.show_prettier(&idmap);
    let display = tree.to_string();
    shot!("show-prettier", display);

    let (flatten_tree, empty) = TreeLines::new(dmod, &idmap);
    let display_new = flatten_tree.display_as_plain_text();
    assert_eq!(expected: display, actual: display_new);
    snap!("flatten-tree", flatten_tree.all_lines());
    shot!("empty-tree-with-same-depth", empty);

    let dmod = flatten_tree.modules_tree();
    snap!(dmod.current_items_counts(), @r###"
    ItemCount {
        modules: 1,
        structs: 2,
        functions: 3,
        traits: 1,
        constants: 2,
        macros_decl: 1,
    }
    "###);
    snap!(dmod.recursive_items_counts(), @r###"
    ItemCount {
        modules: 2,
        structs: 2,
        enums: 1,
        functions: 3,
        traits: 1,
        constants: 2,
        macros_decl: 1,
    }
    "###);

    snap!(dmod.current_impls_counts(), @r###"
    ImplCounts {
        total: ImplCount {
            kind: Both,
            total: 3,
            structs: 3,
        },
        inherent: ImplCount {
            kind: Inherent,
            total: 1,
            structs: 1,
        },
        trait: ImplCount {
            kind: Trait,
            total: 2,
            structs: 2,
        },
    }
    "###);
    snap!(dmod.recursive_impls_counts(), @r###"
    ImplCounts {
        total: ImplCount {
            kind: Both,
            total: 5,
            structs: 3,
            enums: 2,
        },
        inherent: ImplCount {
            kind: Both,
            total: 2,
            structs: 1,
            enums: 1,
        },
        trait: ImplCount {
            kind: Trait,
            total: 3,
            structs: 2,
            enums: 1,
        },
    }
    "###);
}
