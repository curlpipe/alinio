use alinio::table::{Align, Table};

fn main() {
    // Get terminal width
    //let width = crossterm::terminal::size().unwrap().0;
    // Set up table with data
    let mut table = Table::new(
        vec![
            // Row 1
            vec![
                "Title".to_string(), 
                "Album".to_string(),
                "Artist".to_string(),
                "Genre".to_string(),
                "Year".to_string(),
            ],
            // Row 2
            vec![
                "Once in a Lifetime".to_string(),
                "Remain in Light".to_string(),
                "Talking Heads".to_string(),
                "Rock".to_string(),
                "1981".to_string(),
            ],
        ],
        // Make table fit full width of terminal
        0,
    );
    // Set priorities of the columns
    table.set_priorities(&[4, 1, 3, 0, 2]);
    // Add space to the sides of the table
    table.set_surround(true);
    // Align each cell center (Can be Align::Center, Align::Left, Align::Right)
    table.set_alignment(Align::Left);
    // Print the table
    for i in (10..100).rev() {
        table.set_space(i);
        println!("{}\n---", table.render().unwrap().join("\n"))
    }
    // Test
    let data = vec![
        vec!["First name", "Surname", "Telephone"],
        vec!["John",       "Smith",   "04529834125"],
    ];
    let mut table = Table::new(data, 24);
    // First name is the most important, surname is least important, telephone is second most
    // important.
    // The higher the number, the more important the column
    table.set_priorities(&[2, 0, 1]);
    // We can't fit the whole table in 24 spaces, so it looks to see what it can remove.
    // In this case, we can remove the surname column in order to make it fit, as the surname column
    // has the lowest priority in the table.
    println!("{}\n---", table.render().unwrap().join("\n"))
}

