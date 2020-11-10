#![feature(test)]

extern crate test;

use test::Bencher;

#[derive(serde::Serialize, serde::Deserialize)]
struct Object<'a> {
    #[serde(borrow)]
    ext: json_ext::Ext<'a>,
}

const JSON: &str = include_str!("json/sample.json");

#[bench]
fn bench_deserialize(b: &mut Bencher) {
    b.iter(|| serde_json::from_str::<Object>(JSON).unwrap());
}
