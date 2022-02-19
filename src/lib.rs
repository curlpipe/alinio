/// Export alignment utilties
pub mod align;

/// Export table and column utilities
pub mod table;

#[cfg(test)]
mod tests {
    use crate::align;
    use crate::table::{Align, Table};
    use unicode_width::UnicodeWidthStr;

    #[test]
    fn test_center_align() {
        // Test with normal usage but odd left over space
        let out = align::center("hello, world!", 20);
        let expected = "   hello, world!    ".to_string();
        assert_eq!(out, Some(expected));
        assert_eq!(out.unwrap().width(), 20);
        // Test with normal usage but even left over space
        let out = align::center("hello, world!", 21);
        let expected = "    hello, world!    ".to_string();
        assert_eq!(out, Some(expected));
        assert_eq!(out.unwrap().width(), 21);
        // Test returning None when there isn't enough space
        let out = align::center("too long!", 2);
        let expected = None;
        assert_eq!(out, expected);
        // Test behaviour with zero space
        let out = align::center("too long!", 0);
        let expected = None;
        assert_eq!(out, expected);
        // Test behaviour with empty string
        let out = align::center("", 5);
        let expected = "     ".to_string();
        assert_eq!(out, Some(expected));
        assert_eq!(out.unwrap().width(), 5);
        // Test behaviour with empty string and zero space
        let out = align::center("", 0);
        let expected = "".to_string();
        assert_eq!(out, Some(expected));
        assert_eq!(out.unwrap().width(), 0);
        // Test behaviour with predefined space
        let out = align::center("  ", 4);
        let expected = "    ".to_string();
        assert_eq!(out, Some(expected));
        assert_eq!(out.unwrap().width(), 4);
    }

    #[test]
    fn test_left_align() {
        // Test with normal usage
        let out = align::left("hello, world!", 20);
        let expected = "hello, world!       ".to_string();
        assert_eq!(out, Some(expected));
        assert_eq!(out.unwrap().width(), 20);
        // Test returning None when there isn't enough space
        let out = align::left("hello, world!", 3);
        let expected = None;
        assert_eq!(out, expected);
        // Test with zero space
        let out = align::left("hello, world!", 0);
        let expected = None;
        assert_eq!(out, expected);
        // Test behaviour with empty string and zero space
        let out = align::left("", 0);
        let expected = "".to_string();
        assert_eq!(out, Some(expected));
        assert_eq!(out.unwrap().width(), 0);
        // Test behaviour with predefined space
        let out = align::center("  ", 6);
        let expected = "      ".to_string();
        assert_eq!(out, Some(expected));
        assert_eq!(out.unwrap().width(), 6);
    }

    #[test]
    fn test_right_align() {
        // Test with normal usage
        let out = align::right("hello, world!", 20);
        let expected = "       hello, world!".to_string();
        assert_eq!(out, Some(expected));
        assert_eq!(out.unwrap().width(), 20);
        // Test returning None when there isn't enough space
        let out = align::right("hello, world!", 3);
        let expected = None;
        assert_eq!(out, expected);
        // Test with zero space
        let out = align::right("hello, world!", 0);
        let expected = None;
        assert_eq!(out, expected);
        // Test behaviour with empty string and zero space
        let out = align::right("", 0);
        let expected = "".to_string();
        assert_eq!(out, Some(expected));
        assert_eq!(out.unwrap().width(), 0);
        // Test behaviour with predefined space
        let out = align::right("  ", 6);
        let expected = "      ".to_string();
        assert_eq!(out, Some(expected));
        assert_eq!(out.unwrap().width(), 6);
    }

    #[test]
    fn test_between_align() {
        // Test with 2 inputs
        let out = align::between(&["hello", "world!"], 20);
        let expected = "hello         world!".to_string();
        assert_eq!(out, Some(expected));
        assert_eq!(out.unwrap().width(), 20);
        // Test with 3 inputs
        let out = align::between(&["NORMAL", "test.txt", "20/25"], 23);
        let expected = "NORMAL  test.txt  20/25".to_string();
        assert_eq!(out, Some(expected));
        assert_eq!(out.unwrap().width(), 23);
        // Test with 4 inputs
        let out = align::between(&["Title", "Artist", "Album", "Year"], 25);
        let expected = "Title  Artist  Album Year".to_string();
        assert_eq!(out, Some(expected));
        assert_eq!(out.unwrap().width(), 25);
        // Test not enough space on 2
        let out = align::between(&["hello", "world!"], 4);
        let expected = None;
        assert_eq!(out, expected);
        // Test not enough space on 3
        let out = align::between(&["NORMAL", "test.txt", "20/25"], 2);
        let expected = None;
        assert_eq!(out, expected);
        // Test not enough space on 4
        let out = align::between(&["Title", "Artist", "Album", "Year"], 10);
        let expected = None;
        assert_eq!(out, expected);
        // Test with 0 inputs
        let out = align::between(&[], 10);
        let expected = "          ".to_string();
        assert_eq!(out, Some(expected));
        assert_eq!(out.unwrap().width(), 10);
        // Test with 0 inputs and space 0
        let out = align::between(&[], 0);
        let expected = "".to_string();
        assert_eq!(out, Some(expected));
        assert_eq!(out.unwrap().width(), 0);
        // Test with 1 input
        let out = align::between(&["yeet"], 10);
        let expected = "yeet      ".to_string();
        assert_eq!(out, Some(expected));
        assert_eq!(out.unwrap().width(), 10);
        // Test with a few empty inputs
        let out = align::between(&["", "", "", "", "", "", "", ""], 10);
        let expected = "          ".to_string();
        assert_eq!(out, Some(expected));
        assert_eq!(out.unwrap().width(), 10);
        // Test with a few predefined spaces
        let out = align::between(&[" ", "  ", "   ", " ", " ", "   ", " ", ""], 12);
        let expected = "            ".to_string();
        assert_eq!(out, Some(expected));
        assert_eq!(out.unwrap().width(), 12);
    }

    #[test]
    fn test_around_align() {
        // Test with 2 inputs
        let out = align::around(&["hello", "world!"], 20);
        let expected = "   hello   world!   ".to_string();
        assert_eq!(out, Some(expected));
        assert_eq!(out.unwrap().width(), 20);
        // Test with 3 inputs
        let out = align::around(&["NORMAL", "test.txt", "20/25"], 23);
        let expected = " NORMAL test.txt 20/25 ".to_string();
        assert_eq!(out, Some(expected));
        assert_eq!(out.unwrap().width(), 23);
        // Test with 4 inputs
        let out = align::around(&["Title", "Artist", "Album", "Year"], 25);
        let expected = " Title Artist Album Year ".to_string();
        assert_eq!(out, Some(expected));
        assert_eq!(out.unwrap().width(), 25);
        // Test not enough space on 2
        let out = align::around(&["hello", "world!"], 4);
        let expected = None;
        assert_eq!(out, expected);
        // Test not enough space on 3
        let out = align::around(&["NORMAL", "test.txt", "20/25"], 2);
        let expected = None;
        assert_eq!(out, expected);
        // Test not enough space on 4
        let out = align::around(&["Title", "Artist", "Album", "Year"], 10);
        let expected = None;
        assert_eq!(out, expected);
        // Test with 0 inputs
        let out = align::around(&[], 10);
        let expected = "          ".to_string();
        assert_eq!(out, Some(expected));
        assert_eq!(out.unwrap().width(), 10);
        // Test with 0 inputs and space 0
        let out = align::around(&[], 0);
        let expected = "".to_string();
        assert_eq!(out, Some(expected));
        assert_eq!(out.unwrap().width(), 0);
        // Test with 1 input
        let out = align::around(&["yeet"], 10);
        let expected = "   yeet   ".to_string();
        assert_eq!(out, Some(expected));
        assert_eq!(out.unwrap().width(), 10);
        // Test with a few empty inputs
        let out = align::around(&["", "", "", "", "", "", "", ""], 10);
        let expected = "          ".to_string();
        assert_eq!(out, Some(expected));
        assert_eq!(out.unwrap().width(), 10);
        // Test with a few predefined spaces
        let out = align::around(&[" ", "  ", "   ", " ", " ", "   ", " ", ""], 12);
        let expected = "            ".to_string();
        assert_eq!(out, Some(expected));
        assert_eq!(out.unwrap().width(), 12);
    }

    #[test]
    fn test_table() {
        // Test simple table rendering with various alignments
        let mut table = Table::new(
            vec![
                vec![
                    "Title".to_string(),
                    "Artist".to_string(),
                    "Year".to_string(),
                ],
                vec![
                    "Once in a Lifetime".to_string(),
                    "Talking Heads".to_string(),
                    "1981".to_string(),
                ],
            ],
            40,
        );
        assert_eq!(
            table.render().unwrap(),
            vec![
                "Title                Artist         Year".to_string(),
                "Once in a Lifetime   Talking Heads  1981".to_string()
            ]
        );
        table.set_alignment(Align::Right);
        assert_eq!(
            table.render().unwrap(),
            vec![
                "             Title          Artist  Year".to_string(),
                "Once in a Lifetime   Talking Heads  1981".to_string()
            ]
        );
        table.set_alignment(Align::Center);
        assert_eq!(
            table.render().unwrap(),
            vec![
                "      Title             Artist      Year".to_string(),
                "Once in a Lifetime   Talking Heads  1981".to_string()
            ]
        );
        // Test surround and non-surround tables
        table.set_surround(true);
        table.set_alignment(Align::Left);
        assert_eq!(
            table.render().unwrap(),
            vec![
                "  Title              Artist        Year ".to_string(),
                "  Once in a Lifetime Talking Heads 1981 ".to_string(),
            ]
        );
        table.set_alignment(Align::Right);
        assert_eq!(
            table.render().unwrap(),
            vec![
                "               Title        Artist Year ".to_string(),
                "  Once in a Lifetime Talking Heads 1981 ".to_string()
            ]
        );
        table.set_alignment(Align::Right);
        assert_eq!(
            table.render().unwrap(),
            vec![
                "               Title        Artist Year ".to_string(),
                "  Once in a Lifetime Talking Heads 1981 ".to_string(),
            ]
        );
        // Test not enough space
        table.set_space(10);
        assert_eq!(
            table.render(),
            Some(vec!["          ".to_string(), "          ".to_string()]),
        );
        table.set_surround(false);
        assert_eq!(
            table.render(),
            Some(vec!["          ".to_string(), "          ".to_string()]),
        );
        // Test column collapsing
        table.set_surround(true);
        table.set_priorities(&[2, 0, 1]);
        table.set_space(25);
        assert_eq!(
            table.render().unwrap(),
            vec![
                "              Title Year ".to_string(),
                " Once in a Lifetime 1981 ".to_string(),
            ]
        );
        table.set_space(24);
        assert_eq!(
            table.render().unwrap(),
            vec![
                "                Title   ".to_string(),
                "   Once in a Lifetime   ".to_string(),
            ]
        );
        // Play with empty tables
        let table = Table::new::<String>(vec![], 0);
        assert_eq!(table.render(), Some(vec![]));
        let table = Table::new::<String>(vec![], 10);
        assert_eq!(table.render(), Some(vec![]));
        let table = Table::new::<String>(vec![vec![], vec![]], 0);
        assert_eq!(table.render(), Some(vec!["".to_string(), "".to_string()]));
        let table = Table::new::<String>(vec![vec![], vec![]], 10);
        assert_eq!(
            table.render(),
            Some(vec!["          ".to_string(), "          ".to_string()])
        );
        let table = Table::new::<String>(vec![vec![]], 0);
        assert_eq!(table.render(), Some(vec!["".to_string()]));
        let table = Table::new::<String>(vec![vec![]], 1);
        assert_eq!(table.render(), Some(vec![" ".to_string()]));
        let table = Table::new(vec![vec!["".to_string()], vec![]], 100);
        assert_eq!(table.render(), None);
    }

    #[test]
    fn test_partial_table() {
        let table = Table::new(
            vec![
                vec![
                    "Title".to_string(),
                    "Artist".to_string(),
                    "Year".to_string(),
                ],
                vec![
                    "Once in a Lifetime".to_string(),
                    "Talking Heads".to_string(),
                    "1981".to_string(),
                ],
            ],
            40,
        );
        assert_eq!(
            table.render_partial(0).unwrap(),
            vec![
                "Title                Artist         Year".to_string(),
                "Once in a Lifetime   Talking Heads  1981".to_string()
            ]
        );
        assert_eq!(
            table.render_partial(1).unwrap(),
            vec!["Once in a Lifetime   Talking Heads  1981".to_string()]
        );
        assert_eq!(table.render_partial(2), Some(vec![]));
        assert_eq!(table.render_partial(3), Some(vec![]));
        // Test context awareness
        let table = Table::new(
            vec![
                vec![
                    "Once in a Lifetime".to_string(),
                    "Talking Heads".to_string(),
                    "1981".to_string(),
                ],
                vec![
                    "Title".to_string(),
                    "Artist".to_string(),
                    "Year".to_string(),
                ],
            ],
            40,
        );
        assert_eq!(
            table.render_partial(1).unwrap(),
            vec!["Title                Artist         Year".to_string(),]
        );
    }
}
