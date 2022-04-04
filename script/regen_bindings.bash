#!/bin/bash

cargo build --package naifspice-build --features "naifspice-build/copy_bindgen_includes" --verbose

rm src/naifspice-sys/src/generated.rs
bindgen "$1/SpiceUsr.h" -o src/naifspice-sys/src/generated.rs