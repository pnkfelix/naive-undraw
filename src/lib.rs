extern crate "boxdraw-rs" as boxdraw;

use boxdraw::{Undraw, Script};
use boxdraw::grid::Grid;

pub struct SimpleSearch;

impl Undraw for SimpleSearch {
    fn undraw(&self, picture: &str) -> Script {
        let mut grid = Grid::from_str(picture).unwrap();
        let w = grid.width();
        let h = grid.height();

        let mut s = Script::new(w, h);

        for x in range(0, w) {
            for y in range(0, h) {
                // Found a left-corner; find the extent of the box.
                println!("inspecting ({},{})", x, y);
                println!("char at: ({},{}): '{}'", x, y, grid.get(x, y));
                if grid.get(x, y) == '+' {
                    let mut box_w = 1;
                    for i in range(x+1, w) {
                        let c = grid.get(i, y);
                        println!("scanning across, c: '{}'", c);
                        if c == '-' {
                            continue;
                        }
                        if c == '+' {
                            box_w = i - x + 1;
                            break;
                        }
                    }

                    let mut box_h = 1;
                    for j in range(y+1, h) {
                        assert!(j < h);
                        let left = grid.get(x, j);
                        assert!(j < h);
                        let right = grid.get(x + box_w - 1, j);
                        println!("scanning down, left: '{}' right: '{}'",
                                 left, right);
                        if left == '|' && right == '|' {
                            continue
                        }
                        if left == '+' && right == '+' {
                            box_h = j - y + 1;
                            break;
                        }
                    }

                    assert!(y+1 < h);
                    let fill = grid.get(x+1, y+1);
                    let cmd = boxdraw::rect(x, y, box_w, box_h, fill);
                    s.add_end_command(cmd);

                    // Now, clear the matched area.
                    println!("clearing matched area ({},{}) of w:{} h:{}",
                             x, y, box_w, box_h);
                    for i in range(x, x+box_w) {
                        for j in range(y, y+box_h) {
                            grid.set(i, j, '?');
                        }
                    }
                }
            }
        }

        s
    }
}

#[test]
fn check_simple() {
    boxdraw::check_undraw("...\n", &SimpleSearch).unwrap();

    boxdraw::check_undraw(".....\n\
                           .+-+.\n\
                           .|b|.\n\
                           .+-+.\n\
                          ",
                          &SimpleSearch).unwrap();

    boxdraw::check_undraw(".............\n\
                           .......+--+..\n\
                           .+-+...|cc|..\n\
                           .|b|...|cc|..\n\
                           .+-+...+--+..\n\
                           .............\n\
                          ",
                          &SimpleSearch).unwrap();

    boxdraw::check_undraw(".........\n\
                           .........\n\
                           .+-+.....\n\
                           .|b|.....\n\
                           .+-+.....\n\
                           .+--+....\n\
                           .|cc|....\n\
                           .|cc|....\n\
                           .+--+....\n\
                           .........\n\
                          ",
                          &SimpleSearch).unwrap();
}

#[test]
#[should_fail]
fn too_naive_to_pass_this_yet() {
    boxdraw::check_undraw(".............\n\
                           ....+---+....\n\
                           ....|bbb|....\n\
                           ....|bb+--+..\n\
                           ....|bb|cc|..\n\
                           ....+--|cc|..\n\
                           .......+--+..\n\
                           .............\n\
                          ",
                          &SimpleSearch).unwrap();

}
