load(
    "@rules_rust//rust:defs.bzl",
    "rust_doc",
    "rust_doc_test",
    "rust_library",
    "rust_test",
)

package(default_visibility = ["//visibility:public"])

rust_library(
    name = "sane",
    srcs = [
        "net.rs",
        "net/io.rs",
        "net/rpc_00_init.rs",
        "net/rpc_01_get_devices.rs",
        "net/rpc_02_open.rs",
        "net/rpc_03_close.rs",
        "net/rpc_04_get_option_descriptors.rs",
        "net/rpc_05_control_option.rs",
        "net/rpc_06_get_parameters.rs",
        "net/rpc_07_start.rs",
        "net/rpc_08_cancel.rs",
        "net/rpc_09_authorize.rs",
        "sane.rs",
        "util.rs",
    ],
    crate_features = [
        "alloc",
        "std",
    ],
    edition = "2018",
)

rust_test(
    name = "net_test",
    size = "small",
    srcs = ["net_test.rs"],
    edition = "2018",
    deps = [":sane"],
)

rust_test(
    name = "sane_test",
    size = "small",
    srcs = ["sane_test.rs"],
    edition = "2018",
    deps = [":sane"],
)

rust_test(
    name = "util_test",
    size = "small",
    srcs = ["util_test.rs"],
    edition = "2018",
    deps = [":sane"],
)

rust_doc(
    name = "sane_doc",
    crate = ":sane",
)

rust_doc_test(
    name = "sane_doc_test",
    size = "small",
    crate = ":sane",
)
