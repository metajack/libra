// Copyright (c) The Libra Core Contributors
// SPDX-License-Identifier: Apache-2.0

use std::collections::HashMap;
use types::{account_address::AccountAddress, language_storage::ModuleId};
use vm::file_format::{Kind, StructHandleIndex};

/// Struct representing the expected definition for a native struct
pub struct NativeStruct {
    /// The expected kind of the struct/resource
    pub expected_kind: Kind,
    /// The expected kind constraints of the type parameters.
    pub expected_type_parameters: Vec<Kind>,
    /// The expected index for the struct
    /// Helpful for ensuring proper typing of native functions
    pub expected_index: StructHandleIndex,
}

/// Looks up the expected native struct definition from the module id (address and module) and
/// function name where it was expected to be declared
pub fn dispatch_native_struct(
    module: &ModuleId,
    struct_name: &str,
) -> Option<&'static NativeStruct> {
    NATIVE_STRUCT_MAP.get(module)?.get(struct_name)
}

macro_rules! add {
    ($m:ident, $addr:expr, $module:expr, $name:expr, $kind: expr) => {{
        add!($m, $addr, $module, $name, $kind, vec![])
    }};
    ($m:ident, $addr:expr, $module:expr, $name:expr, $kind: expr, $ty_kinds: expr) => {{
        let addr = AccountAddress::from_hex_literal($addr).unwrap();
        let id = ModuleId::new(addr, $module.into());
        let struct_table = $m.entry(id).or_insert_with(HashMap::new);
        let expected_index = StructHandleIndex(struct_table.len() as u16);

        let s = NativeStruct {
            expected_kind: $kind,
            expected_type_parameters: $ty_kinds,
            expected_index,
        };
        let old = struct_table.insert($name.into(), s);
        assert!(old.is_none());
    }};
}

type NativeStructMap = HashMap<ModuleId, HashMap<String, NativeStruct>>;

lazy_static! {
    static ref NATIVE_STRUCT_MAP: NativeStructMap = {
        use Kind::*;
        let mut m: NativeStructMap = HashMap::new();
        add!(m, "0x0", "Vector", "T", Copyable);
        m
    };
}
