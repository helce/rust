//! Strip all private items from the output. Additionally implies strip_priv_imports.
//! Basically, the goal is to remove items that are not relevant for public documentation.
use crate::clean::{self, ItemIdSet};
use crate::core::DocContext;
use crate::fold::DocFolder;
use crate::passes::{ImplStripper, ImportStripper, Pass, Stripper};

pub(crate) const STRIP_PRIVATE: Pass = Pass {
    name: "strip-private",
    run: strip_private,
    description: "strips all private items from a crate which cannot be seen externally, \
                  implies strip-priv-imports",
};

/// Strip private items from the point of view of a crate or externally from a
/// crate, specified by the `xcrate` flag.
pub(crate) fn strip_private(mut krate: clean::Crate, cx: &mut DocContext<'_>) -> clean::Crate {
    // This stripper collects all *retained* nodes.
    let mut retained = ItemIdSet::default();

    // strip all private items
    {
        let mut stripper = Stripper {
            retained: &mut retained,
            access_levels: &cx.cache.access_levels,
            update_retained: true,
            is_json_output: cx.output_format.is_json() && !cx.show_coverage,
        };
        krate = ImportStripper.fold_crate(stripper.fold_crate(krate));
    }

    // strip all impls referencing private items
    let mut stripper = ImplStripper { retained: &retained, cache: &cx.cache };
    stripper.fold_crate(krate)
}
