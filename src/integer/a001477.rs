/// https://oeis.org/A001477
/// The non-negative integers
use sequence_macros::create_additive;

create_additive!(A001477, 0, 1);

#[test]
fn first_few() {
    let seq = A001477::default();
    let vals = seq.take(10).collect::<Vec<_>>();
    let known = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
        .into_iter().map(|x| x.into()).collect::<Vec<_>>();
    assert_eq!(vals, known);
}
