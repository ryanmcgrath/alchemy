//! Implements parsing, originally written by Bodil Stokke
//! over in [typed-html](https://github.com/bodil/typed-html).

use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub grammar);
