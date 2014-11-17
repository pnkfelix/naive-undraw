use boxdraw;

use SimpleSearch;

// EXERCISE 1: Can you make an undraw implementation that can handle
// the case below?

#[test]
fn three_by_three() {
    boxdraw::check_undraw(".............\n\
                           ..+--+--+--+.\n\
                           ..|aa|bb|cc|.\n\
                           ..|aa|bb|cc|.\n\
                           ..+--+--+--+.\n\
                           ..|dd|ee|ff|.\n\
                           ..|dd|ee|ff|.\n\
                           ..+--+--+--+.\n\
                           ..|gg|hh|ii|.\n\
                           ..|gg|hh|ii|.\n\
                           ..+--+--+--+.\n\
                           .............\n\
                          ",
                          &SimpleSearch).unwrap();

}


// EXERCISE 2: Up until now, we have assumed that all boxes are at
// least width and height >= 2.  So for example `narrow_lines()` above
// shows boxes of width == 2, so narrow that they have no interior.
//
// BUT: The boxdraw-rs library supports boxes of width or height == 1
// as well, and it has *very* different behavior for them. Find out
// what that behavior is (perhaps by looking at the tests in the
// boxdraw library).
//
// Consider what a box with both width and height equal to 1 would
// look like. What implications does this have? For example, what kind
// of pictures can you make with *just* boxes width and height 1?
//
// Try to solve the undraw problem even for such cases.
//
// Note: to do this, you may want to start from scratch; or maybe some
// of the code here could be of use.


// EXERCISE 3: Again, solve the undraw problem, but this time, try to
// generate minimally sized scripts. Note that you can make short
// scripts that first draw large boxes (which will then have borders)
// and then write over the borders with boxes of width or height 1.
