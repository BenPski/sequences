/// https://oeis.org/A000203
/// Lucas numbers starting at 1
///

use sequence_macros::create_recurrent;

create_recurrent!(A000204, [1, 1], [1, 3]);
