// #26207: Show all methods reachable via Deref impls, recursing through multiple dereferencing
// levels and across multiple crates.

// @has 'foo/struct.Foo.html'
// @has '-' '//*[@id="deref-methods-PathBuf"]' 'Methods from Deref<Target = PathBuf>'
// @has '-' '//*[@class="impl-items"]//*[@id="method.as_path"]' 'pub fn as_path(&self)'
// @has '-' '//*[@id="deref-methods-Path"]' 'Methods from Deref<Target = Path>'
// @has '-' '//*[@class="impl-items"]//*[@id="method.exists"]' 'pub fn exists(&self)'
// @has '-' '//*[@class="sidebar-title"]/a[@href="#deref-methods-PathBuf"]' 'Methods from Deref<Target=PathBuf>'
// @has '-' '//*[@class="sidebar-links"]/a[@href="#method.as_path"]' 'as_path'
// @has '-' '//*[@class="sidebar-title"]/a[@href="#deref-methods-Path"]' 'Methods from Deref<Target=Path>'
// @has '-' '//*[@class="sidebar-links"]/a[@href="#method.exists"]' 'exists'

#![crate_name = "foo"]

use std::ops::Deref;
use std::path::PathBuf;

pub struct Foo(PathBuf);

impl Deref for Foo {
    type Target = PathBuf;
    fn deref(&self) -> &PathBuf { &self.0 }
}
