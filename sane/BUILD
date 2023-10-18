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
        "sane.rs",
        "util.rs",
    ],
    edition = "2018",
)

rust_test(
    name = "sane_test",
    size = "small",
    srcs = ["sane_test.rs"],
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
