// use std::fmt;
// use std::io;

// 1. because the return name is same it has confliting 


// fn fn1()->fmt::Result{
//     // snip
// }
// fn fn2()->io::Result<()>{
//     //snip
// }

// 2.we can rename the functions or modules that we are brinning into the scope 
use std::fmt::Result;
use std::io::Result as IoResult; // we can add pub keyword before use to use it external files also
// use rand::{Rng,CryptoRng etc...} // we can also do like this 
// fn fn1()->Result{
//     // snip
//     Ok(());
// }
// fn fn2()->IoResult<()>{
//     //snip
//     Ok(());
// }


// use std::io;
// use std::io::Write;

// instead opf this we can add like this 

use std::io::{self,Write};
// get all of the puiblic items 
use std::io::*;