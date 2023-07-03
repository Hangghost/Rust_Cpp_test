

use autocxx::prelude::*;

// use crate::ffi::kb::math::FiltFilt; // use all the main autocxx functions
// use autocxx::subclass::prelude::*;

include_cpp! {
    #include "include/Filt.h"
    #include "include/FiltFilt.h"
    safety!(unsafe) // see details of unsafety policies described in the 'safety' section of the book
    generate!("kb::math::FiltFilt") // generate bindings for this type
}



struct FilterCoefficients {
    coefficients_a: [f64; 4],
    coefficients_b: [f64; 4],
}

impl FilterCoefficients {
    fn new() -> FilterCoefficients {
        FilterCoefficients {
            coefficients_a: [1.0000, -2.374094743709352, 1.929355669091215, -0.532075368312092],
            coefficients_b: [2.898194633721429e-03, 8.694583901164288e-03, 8.694583901164288e-03, 2.898194633721429e-03],
        }
    }
}



fn main() {
    let fc = FilterCoefficients::new();
    println!("{:?}", fc.coefficients_a);
    println!("{:?}", fc.coefficients_b);
    
    let signal = [10780, 10784, 10777, 10770, 10788, 10800, 10804, 10798, 10784, 10765, 10772, 10760, 10720, 10708, 10709, 10700, 10697, 10696, 10696, 10706, 10709];

    // let filter_get: Vec<cxx::UniquePtr<ffi::filter_a>> = vec![cxx::UniquePtr::new(ffi::fiflter_a{&fc})];



}
