#![feature(const_fn)]
#![feature(extern_types)]
#![feature(or_patterns)]
#![allow(const_item_mutation)]

#[cfg(test)]
#[macro_use]
extern crate quickcheck;

#[macro_use]
mod dbg;

#[macro_use]
mod paging;

pub mod cpu;

pub mod js_api;
pub mod profiler;

mod analysis;
mod codegen;
mod config;
mod control_flow;
mod cpu_context;
mod gen;
mod jit;
mod jit_instructions;
mod leb;
mod modrm;
mod opstats;
mod page;
mod prefix;
mod regs;
mod softfloat;
mod state_flags;
mod util;
mod wasmgen;
mod zstd;
