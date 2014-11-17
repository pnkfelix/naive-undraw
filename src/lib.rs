#![feature(phase)]

extern crate "boxdraw-rs" as boxdraw;

#[phase(plugin)]
extern crate log;
extern crate log;

use boxdraw::{Undraw, Script, Command};
use boxdraw::grid::Grid;

use std::char::from_u32;

pub struct SimpleSearch;

impl Undraw for SimpleSearch {
    fn undraw(&self, picture: &str) -> Script {
        debug!("undraw:\n{}", picture);

        let unused_char = self.find_unused_char(picture);

        let mut grid = Grid::from_str(picture).unwrap();
        let w = grid.width();
        let h = grid.height();

        let mut s = Script::new(w, h);

        // We find the commands in reverse order.
        let mut rev_commands = vec![];
        let mut cleared_some;
        loop {
            cleared_some = false;
            for y in range(0, h) {
                for x in range(0, w) {

                    match self.try_upper_left(&grid, unused_char, x, y) {
                        Some(cmd) => {
                            // Save the command for a box at (x,y),
                            // and record the effects it will have on
                            // the board (by clearing the area it
                            // covers).
                            rev_commands.push(cmd);
                            self.clear_area_drawn_by_cmd(&mut grid,
                                                         cmd,
                                                         unused_char);
                            cleared_some = true;
                        }

                        None => {
                            continue;
                        }
                    }

                }
            }

            // If we managed to clear *any* of the areas, then we
            // should go back and look to see if we can now find
            // matches for boxes that were previously obscured by the
            // boxes that were on top of them.
            if !cleared_some {
                break;
            }
        }

        // Done accumulating commands; now put them into a proper
        // script.
        let commands = {
            rev_commands.as_mut_slice().reverse();
            rev_commands
        };

        for cmd in commands.into_iter() {
            s.add_end_command(cmd);
        }

        return s;
    }
}

#[deriving(Show)]
enum MatchBox {
    ExactMatch(boxdraw::Command),
    PartialMatch,

    NonMatch(WhyNonMatch),
}

#[deriving(PartialEq, Show)]
struct WhyNonMatch {
    reason: String,
}

fn non_match(why: String) -> MatchBox {
    NonMatch(WhyNonMatch { reason: why })
}

impl PartialEq for MatchBox {
    fn eq(&self, that: &MatchBox) -> bool {
        match (self, that) {
            (&ExactMatch(c1), &ExactMatch(c2)) => {
                c1.x == c2.x &&
                    c1.y == c2.y &&
                    c1.w == c2.w &&
                    c1.h == c2.h &&
                    c1.fill == c2.fill
            }
            (&PartialMatch, &PartialMatch) => true,
            (&NonMatch(_), &NonMatch(_)) => true,
            _ => false
        }
    }
}

impl SimpleSearch {
    fn clear_area_drawn_by_cmd(&self,
                               grid: &mut Grid,
                               cmd: Command,
                               unused_char: char) {
        let Command { x, y, w, h, fill: _ } = cmd;
        debug!("clearing matched area ({},{}) of w:{} h:{}",
               x, y, w, h);
        for i in range(x, x+w) {
            for j in range(y, y+h) {
                grid.set(i, j, unused_char);
            }
        }
    }

    fn find_unused_char(&self, picture: &str) -> char {
        static AVOID : [char, ..5] = ['.', '+', '-', '|', '\n'];
        let mut guess = '?' as u32;
        loop {
            match from_u32(guess) {
                Some(char_guess) if !AVOID.contains(&char_guess) && !picture.contains_char(char_guess) => {
                    return char_guess;
                }
                _ => {
                    guess += 1;
                }
            }
        }
    }

    /// Attempts to match a box at `(x,y)`, using `hidden` as a marker
    /// for spaces that were overwritten by a later command.
    fn try_upper_left(&self, grid: &Grid, hidden: char, x: u32, y: u32) -> Option<boxdraw::Command> {

        let w = grid.width();
        let h = grid.height();

        // Found a left-corner; find the extent of the box.
        // println!("inspecting ({},{})", x, y);
        // println!("char at: ({},{}): '{}'", x, y, grid.get(x, y));

        let mut saw_partial = false;
        assert!(x < w); assert!(y < h);
        let c = grid.get(x,y);
        if c == '+' {
            'next_width:
            for i in range(x+1, w) {
                for j in range(y+1, h) {
                    let match_at = TryMatchAt {
                        grid: grid,
                        hidden: hidden,
                        x: x,
                        y: y,
                    };
                    match match_at.try(i, j) {
                        ExactMatch(cmd) => return Some(cmd),
                        PartialMatch => {
                            saw_partial = true;
                            continue;
                        }
                        NonMatch(ref why) => {
                            if saw_partial {
                                debug!("nonmatch for {}-{} because {}", (x,y), (i,j), why.reason);
                            }
                            continue 'next_width;
                        }
                    }
                }
            }
        }

        // if we fell through to here, then we were not able to find a
        // box at `(x,y)` on this attempt.
        return None;
    }
}

struct TryMatchAt<'a> {
    grid: &'a Grid,
    hidden: char,
    x: u32,
    y: u32,
}

impl<'a> TryMatchAt<'a> {

    /// Attempts to match a box at `(self.x, self.y)` extending to
    /// `(right, below)` using `self.hidden` as a marker for spaces
    /// that were overwritten by a later command.
    fn try(&self, right: u32, below: u32) -> MatchBox {
        let x = self.x;
        let y = self.y;
        let grid = self.grid;
        let w = grid.width();
        let h = grid.height();

        let box_w = right - x + 1;
        let box_h = below - y + 1;

        let trial = (x, y, box_w, box_h);

        debug!("try: x {} y {} box_w {} box_h {} right {} below {} w {} h {}",
               x, y, box_w, box_h, right, below, w, h);

        assert!(x < w); assert!(y < h);
        let upper_left = grid.get(x, y);
        assert!(x < w); assert!(below < h);
        let lower_left = grid.get(x, below);
        assert!(right < w); assert!(y < h);
        let upper_right = grid.get(right, y);
        assert!(right < w); assert!(below < h);
        let lower_right = grid.get(right, below);

        fn fail(wanted: &str, loc: (u32,u32), value: char) -> MatchBox {
            let msg = format!("failed to match {} at {}: {}",
                              wanted, loc, value);
            non_match(msg)
        };
        let upper_left_fail = || fail("upper_left corner", (x,y), upper_left);
        let lower_left_fail = || {
            fail("lower_left corner", (x,below), lower_left)
        };
        let upper_right_fail = || {
            fail("upper_right corner", (right,y), upper_right)
        };
        let lower_right_fail = || {
            fail("lower_right corner", (right,below), lower_right)
        };

        let mut extends_right = false;
        let mut extends_down = false;

        // Tracks the character to use for the box interior, which we
        // do not necessarily know until scanning deep into the box.
        let mut interior = None;

        // First, check the four corners.
        //
        // We want either:
        //
        // 1. '+', meaning this is (probably) a real corner, and worth trying to match
        //
        // 2. self.hidden, meaning that this could be a corner that
        //    was hidden by another box above it, and so we can
        //    speculatively try to match it, or
        //
        // 3. a wall ('|' or '-'), meaning that this is not the
        //    corner, but we might successfully find the corner if we
        //    keep looking (and thus we should return `PartialMatch`)

        if !self.matches_corner(upper_left) {
            return upper_left_fail();
        }
        if !self.matches_corner(lower_left) {
            if lower_left == '|' {
                extends_down = true;
            } else {
                return lower_left_fail();
            }
        }

        if !self.matches_corner(upper_right) {
            if upper_right == '-' {
                extends_right = true;
            } else {
                return upper_right_fail();
            }
        }

        if !self.matches_corner(lower_right) {
            if extends_down && lower_right == '|' {
                // okay
            } else if extends_right && lower_right == '-' {
                // okay
            } else if extends_down && extends_right {
                if lower_right != self.hidden {
                    interior = Some(lower_right);
                }
            } else {
                return lower_right_fail();
            }
        }

        // Second: Okay, at this point we have either seen all four
        // corners, or we saw walls where we expected corners, and
        // thus might hope to extend the box.
        //
        // Given the above, check that walls of box are in place.

        for i in range(x + 1, right) { // across
            assert!(i < w); assert!(y < h);
            let c = grid.get(i, y);
            if !self.matches_horizontal_wall(c) {
                return fail("top horizontal_wall", (i,y), c);
            }
            assert!(i < w); assert!(below < h);
            let c = grid.get(i, below);
            if !extends_down {
                if !self.matches_horizontal_wall(c) {
                    return fail("bot horizontal_wall", (i,below), c);
                }
            } else if c == self.hidden {
                // keep going
            } else {
                if interior.is_none() {
                    interior = Some(c);
                }
                if interior.is_some() && Some(c) != interior {
                    return fail("bot interior", (i,below), c);
                }
            }
        }
        for j in range(y + 1, below) { // down
            assert!(x < w); assert!(j < h);
            let c = grid.get(x, j);
            if !self.matches_vertical_wall(c) {
                return fail("left vertical wall", (x,j), c);
            }
            assert!(right < w); assert!(j < h);
            let c = grid.get(right, j);
            if !extends_right {
                if !self.matches_vertical_wall(c) {
                    return fail("right vertical wall", (right,j), c);
                }
            } else if c == self.hidden {
                // keep going
            } else {
                if interior.is_none() {
                    interior = Some(c);
                }
                if interior.is_some() && Some(c) != interior {
                    return fail("right interior", (right,j), c);
                }
            }
        }

        // Third: We have corners and walls; check that the interior
        // is sound.

        for i in range(x + 1, right) {
            for j in range(y + 1, below) {
                assert!(i < w); assert!(j < h);
                match (interior, grid.get(i, j)) {
                    (None, c) => {
                        if c == self.hidden {
                            // Okay, clean match and we still do not know
                            // what the interior is.
                            continue;
                        } else {
                            // finally, a non-hidden marker.  Assume
                            // that this is the interior.
                            interior = Some(c);
                            continue;
                        }
                    }
                    (Some(c1), c2) => {
                        if c1 == c2 || c2 == self.hidden {
                            // the color c2 matches the interior c1; keep
                            // going.
                            continue;
                        } else {
                            return fail("interior", (i,j), c2);
                        }
                    }
                }
            }
        }

        // Got this far; return either full or partial box, as
        // appropriate.

        if extends_down || extends_right {
            debug!("partial match {} interior: {}", trial, interior);
            PartialMatch
        } else {
            assert!(x+1 < w); assert!(y+1 < h);
            let fill = grid.get(x+1, y+1);
            let cmd = boxdraw::rect(x, y, box_w, box_h, fill);

            debug!("exact match: {}", cmd);
            ExactMatch(cmd)
        }
    }

    fn matches_corner(&self, c: char) -> bool {
        c == '+' || c == self.hidden
    }

    fn matches_horizontal_wall(&self, c: char) -> bool {
        c == '-' || c == self.hidden
    }

    fn matches_vertical_wall(&self, c: char) -> bool {
        c == '|' || c == self.hidden
    }
}

#[cfg(test)]
mod tests;
#[cfg(test)]
mod exercises;
