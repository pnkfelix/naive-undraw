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




// EXERCISE 2: One might think that once you generate a working
// Script, there's no further way to improve upon it. However,
// consider a case like this:
//
//
//                  ....................
//                  ......+------+......
//                  ......|bbbbbb|......
//                  ......|bbbbbb|......
//                  .+----|bbbbbb|----+.
//                  .|aaaa|bbbbbb|aaaa|.
//                  .|aaaa|bbbbbb|aaaa|.
//                  .+----|bbbbbb|----+.
//                  ......|bbbbbb|......
//                  ......+------+......
//                  ....................
//
// There are two "obvious" relatively short scripts that generate the
// above picture: one that draws two 'a' rectangles and then the
// single 'b' rectangle (thus requiring three commands in the script).
//
//  ....................       ....................
//  ....................       ......+------+......
//  ....................       ......|bbbbbb|......
//  ....................       ......|bbbbbb|......
//  .+----+......+----+.       ......|bbbbbb|......
//  .|aaaa|......|aaaa|. then  ......|bbbbbb|......
//  .|aaaa|......|aaaa|.       ......|bbbbbb|......
//  .+----+......+----+.       ......|bbbbbb|......
//  ....................       ......|bbbbbb|......
//  ....................       ......+------+......
//  ....................       ....................
//
//
// But there is also this approach, which requires only two commands:
//
//  ....................       ....................
//  ....................       ......+------+......
//  ....................       ......|bbbbbb|......
//  ....................       ......|bbbbbb|......
//  .+----------------+.       ......|bbbbbb|......
//  .|aaaaaaaaaaaaaaaa|. then  ......|bbbbbb|......
//  .|aaaaaaaaaaaaaaaa|.       ......|bbbbbb|......
//  .+----------------+.       ......|bbbbbb|......
//  ....................       ......|bbbbbb|......
//  ....................       ......+------+......
//  ....................       ....................
//
// The undraw strategy of the code above will tend to generate the
// three-command script. What ideas do you have to try to make it
// generate "optimal" scripts (where "optimal" here means "minimal
// number of commands."




// EXERCISE 3: Up until now, we have assumed that all boxes are at
// least width and height >= 2.  So for example `test::narrow_lines()`
// shows boxes of width == 2, so narrow that they have no interior.
//
// BUT: The boxdraw-rs library supports boxes of width or height == 1
// as well, and it has *very* different behavior for them. Find out
// what that behavior is (perhaps by looking at the tests in the
// boxdraw library).
//
// Consider what a box with both width and height equal to 1 would
// look like. What implications does this have? For example, what kind
// of pictures can you make with just boxes of *both* width and height 1?
//
// Try to solve the undraw problem even for such cases.
//
// Note 1: to do this, you may want to start from scratch; or maybe
// some of the code here could be of use.
//
// Note 2: You may want to play around first with approaches that are
// not anywhere close to optimal.




// EXERCISE 4: Like exercise 2 above: Again, solve the generalized
// undraw problem (that is, including boxes of width and/or height 1),
// but this time, try to generate relatively small scripts.
//
// Note that you can make short scripts that first draw large boxes
// (which will then have borders) and then write over the borders with
// boxes of width or height 1, like so:
//
//  ....................       ....................   ....................
//  ....................       ....................   ....................
//  ....................       ....................   ....................
//  ....................       ....................   ....................
//  .+----------------+.       .aaaaaaaaaaaaaaaaaa.   .a................a.
//  .|aaaaaaaaaaaaaaaa|. then  .................... ; .a................a.
//  .|aaaaaaaaaaaaaaaa|.       ....................   .a................a.
//  .+----------------+.       .aaaaaaaaaaaaaaaaaa.   .a................a.
//  ....................       ....................   ....................
//  ....................       ....................   ....................
//  ....................       ....................   ....................
//
// and this generates a picture with a borderless box:
//
//  ....................
//  ....................
//  ....................
//  ....................
//  .aaaaaaaaaaaaaaaaaa.
//  .aaaaaaaaaaaaaaaaaa.
//  .aaaaaaaaaaaaaaaaaa.
//  .aaaaaaaaaaaaaaaaaa.
//  ....................
//  ....................
//  ....................
