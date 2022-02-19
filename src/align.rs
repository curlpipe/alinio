/// Align.rs contains everything you'll need to align and display data on the terminal.
/// All functions in this file are compatible with unicode characters.
use unicode_width::UnicodeWidthStr;

/// Aligns the text to the center
///
/// Will return `None` if there is not enough space to fit the text (length of txt > space)
pub fn center(txt: &str, space: usize) -> Option<String> {
    // Determine the width of the characters when displayed
    let len = txt.width();
    // Return None if there is not enough space to fit the string
    if len > space {
        return None;
    }
    // Work out total space needed on each side
    let left_over = space - len;
    let each = left_over / 2;
    // Pad the left hand side
    let left_pad = " ".repeat(each);
    // Pad the right hand side
    let right_pad = " ".repeat(left_over - each);
    // Format and return
    Some(format!("{}{}{}", left_pad, txt, right_pad))
}

/// Aligns the text to the left
///
/// This is particularly useful if you want to align columns in a table
///
/// Will return `None` if there is not enough space to fit the text (length of txt > space)
pub fn left(txt: &str, space: usize) -> Option<String> {
    // Determine the width of the characters when displayed
    let len = txt.width();
    // Return None if there is not enough space to fit the string
    if len > space {
        return None;
    }
    // Work out total space needed on the right
    let left_over = space - len;
    // Pad the right hand side
    let right_pad = " ".repeat(left_over);
    // Format and return
    Some(format!("{}{}", txt, right_pad))
}

/// Aligns the text to the right
///
/// Will return `None` if there is not enough space to fit the text (length of txt > space)
pub fn right(txt: &str, space: usize) -> Option<String> {
    // Determine the width of the characters when displayed
    let len = txt.width();
    // Return None if there is not enough space to fit the string
    if len > space {
        return None;
    }
    // Work out total space needed on the left
    let left_over = space - len;
    // Pad the right hand side
    let left_pad = " ".repeat(left_over);
    // Format and return
    Some(format!("{}{}", left_pad, txt))
}

/// Adds space between the specified strings in the `txt` slice
///
/// Great for rendering a status line, or some kind of simple column set up
///
/// Will return `None` if there is not enough space to fit the text (length of txt > space)
///
/// Example:
/// ```
/// use alinio::align;
/// let result = align::between(&["Title", "Artist", "Album"], 20); // Format 3 columns into a space of 20
/// println!("{}", result.unwrap()); // -> "Title  Artist  Album"
/// ```
pub fn between(txt: &[&str], space: usize) -> Option<String> {
    // Determine the width of the characters when displayed
    let len: usize = txt.iter().map(|x| x.width()).sum();
    // Return None if there is not enough space to fit the string
    if len > space {
        return None;
    }
    // Handle the case of there being < 2 columns provided
    if txt.is_empty() {
        return Some(" ".repeat(space));
    } else if txt.len() == 1 {
        return left(txt[0], space);
    }
    // Work out total space needed between the columns
    let left_over = space - len;
    let pad_places = txt.len().saturating_sub(1); // Number of places where padding is required
    let each = left_over / pad_places;
    let mut remainder = left_over - each * pad_places; // Remainder padding when space doesn't divide equally
                                                       // Begin formatting
    let mut result = "".to_string();
    for t in txt.iter().take(pad_places) {
        // Push text and padding
        result.push_str(t);
        result.push_str(&" ".repeat(each));
        // Redistribute any remainder padding
        if remainder > 0 {
            result.push(' ');
            remainder -= 1;
        }
    }
    // Push on final element
    result.push_str(txt.last().unwrap_or(&""));
    Some(result)
}

/// Adds space between the specified strings in the `txt` slice, and includes spaces on the outside
///
/// Great for setting up a columns with padding on each side
///
/// Will return `None` if there is not enough space to fit the text (length of txt > space)
///
/// Example:
/// ```
/// use alinio::align;
/// let result = align::around(&["Title", "Artist", "Album"], 24); // Format 3 columns into a space of 24
/// println!("{}", result.unwrap()); // -> "  Title  Artist  Album  "
/// ```
pub fn around(txt: &[&str], space: usize) -> Option<String> {
    // Determine the width of the characters when displayed
    let len: usize = txt.iter().map(|x| x.width()).sum();
    // Return None if there is not enough space to fit the string
    if len > space {
        return None;
    }
    // Handle the case of there being < 2 columns provided
    if txt.is_empty() {
        return Some(" ".repeat(space));
    } else if txt.len() == 1 {
        return center(txt[0], space);
    }
    // Work out total space needed between the columns
    let left_over = space - len;
    let pad_places = txt.len() + 1; // Number of places where padding is required
    let each = left_over / pad_places;
    let mut remainder = left_over - each * pad_places; // Remainder padding when space doesn't divide equally
                                                       // Begin formatting
    let mut result = "".to_string();
    for t in 0..pad_places {
        // Push padding
        result.push_str(&" ".repeat(each));
        // Redistribute any remainder padding
        if remainder > 0 {
            result.push(' ');
            remainder -= 1;
        }
        // Push text (if there is text to be pushed)
        if let Some(col) = txt.get(t) {
            result.push_str(col);
        }
    }
    Some(result)
}
