/// https://oeis.org/A000045
/// The fibonnacci sequence
use sequence_macros::create_recurrent;

create_recurrent!(A000045, [1, 1], [0, 1]);

#[test]
fn first_few() {
    let seq = A000045::default();
    let vals = seq.take(10).collect::<Vec<_>>();
    let known = vec![0, 1, 1, 2, 3, 5, 8, 13, 21, 34]
        .into_iter().map(|x| x.into()).collect::<Vec<_>>();
    assert_eq!(vals, known);
}
