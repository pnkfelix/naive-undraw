use boxdraw;
use boxdraw::grid::Grid;

use SimpleSearch;
use ExactMatch;
use TryMatchAt;

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
fn check_that_hidden_char_works() {
    fn check_for_1x1_picture(picture: &str) {
        let grid = Grid::from_str(picture).unwrap();
        let match_at = TryMatchAt {
            grid: &grid,
            hidden: '?',
            x: 1,
            y: 1,
        };
        assert_eq!(match_at.try(3,3), ExactMatch(boxdraw::rect(1, 1, 3, 3, 'b')));
    }

    check_for_1x1_picture(".....\n\
                           .+-+.\n\
                           .|b|.\n\
                           .+-+.\n\
                          ");

    check_for_1x1_picture(".....\n\
                           .+-+.\n\
                           .|b|.\n\
                           .+-?.\n\
                          ");

    check_for_1x1_picture(".....\n\
                           .+-+.\n\
                           .|b?.\n\
                           .+-+.\n\
                          ");

    check_for_1x1_picture(".....\n\
                           .+-+.\n\
                           .|b|.\n\
                           .+?+.\n\
                          ");

    check_for_1x1_picture(".....\n\
                           .+-+.\n\
                           .|b?.\n\
                           .+?+.\n\
                          ");

    check_for_1x1_picture(".....\n\
                           .+-+.\n\
                           .|b?.\n\
                           .+??.\n\
                          ");

    check_for_1x1_picture(".....\n\
                           .+-?.\n\
                           .|b?.\n\
                           .+?+.\n\
                          ");

    check_for_1x1_picture(".....\n\
                           .+-?.\n\
                           .?b?.\n\
                           .??+.\n\
                          ");

}

#[test]
fn check_specific_expected_case_from_overlapping_test() {
    let picture = ".............\n\
                   ....+---+....\n\
                   ....|bbb|....\n\
                   ....|bb????..\n\
                   ....|bb????..\n\
                   ....+--????..\n\
                   .......????..\n\
                   .............\n\
                  ";
    let grid = Grid::from_str(picture).unwrap();
    let match_at = TryMatchAt {
        grid: &grid,
        hidden: '?',
        x: 4,
        y: 1,
    };
    assert_eq!(match_at.try(8,5), ExactMatch(boxdraw::rect(4, 1, 5, 5, 'b')));

}

#[test]
fn overlapping_boxes() {
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

#[test]
fn nested() {
    boxdraw::check_undraw(".............\n\
                           ..+--------+.\n\
                           ..|aaaaaaaa|.\n\
                           ..|aaaaaaaa|.\n\
                           ..|aa+--+aa|.\n\
                           ..|aa|bb|aa|.\n\
                           ..|aa|bb|aa|.\n\
                           ..|aa+--+aa|.\n\
                           ..|aaaaaaaa|.\n\
                           ..|aaaaaaaa|.\n\
                           ..+--------+.\n\
                           .............\n\
                          ",
                          &SimpleSearch).unwrap();
}


#[test]
fn narrow_lines() {
    boxdraw::check_undraw(".............\n\
                           ..++..++..++.\n\
                           ..||..||..||.\n\
                           ..||..||..||.\n\
                           ..||..||..||.\n\
                           ..||..||..||.\n\
                           ..||..||..||.\n\
                           ..||..||..||.\n\
                           ..||..||..||.\n\
                           ..||..||..||.\n\
                           ..++..++..++.\n\
                           .............\n\
                          ",
                          &SimpleSearch).unwrap();
}
