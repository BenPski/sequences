/// https://oeis.org/A000032
/// Lucas numbers starting at 2
///

use sequence_macros::create_recurrent;

create_recurrent!(A000032, [1, 1], [2, 1]);
