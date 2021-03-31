#![no_std]
use oxidation;

#[no_mangle]
fn scalbn() {}

#[no_mangle]
fn scalbnf() {}

#[no_mangle]
fn logbl() {}

#[no_mangle]
fn scalbnl() {}

#[no_mangle]
fn fmaxl() {}

//oxidation::declare_entry!(data);
oxidation::declare_panic!();