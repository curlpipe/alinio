/// Table.rs contains a high level way to render and display tables in crossterm.
/// This of course is all compatible with unicode characters.
use crate::align;
use unicode_width::UnicodeWidthStr;

/// Represents the data in a table
pub type Data = Vec<Vec<String>>;

/// Find the longest string in a list of strings
pub fn find_longest(column: &[&String]) -> usize {
    column.iter().map(|i| i.width()).max().unwrap_or(0)
}

/// For setting the alignment of cells within the table
pub enum Align {
    Left,
    Center,
    Right,
}

impl Default for Align {
    fn default() -> Self {
        Self::Left
    }
}

/// A really powerful table formatter for text user interfaces.
///
/// Example:
/// ```
/// use alinio::table::Table;
/// let data = vec![
///     vec!["First name", "Surname", "Telephone"],
///     vec!["John",       "Smith",   "04529834125"],
/// ];
/// let mut table = Table::new(data, 40);
/// println!("{}\n---", table.render().unwrap().join("\n"))
/// ```
/// This will print a table
#[derive(Default)]
pub struct Table {
    /// Stores the data within this table
    data: Data,
    /// Stores the priority of each column
    priorities: Vec<usize>,
    /// How to align each cell
    align: Align,
    /// The space that this table has
    space: usize,
    /// Surround with padding?
    surround: bool,
}

impl Table {
    /// Create new table with data and space.
    ///
    /// `data` is organized into rows, and then within those rows, there are columns.
    ///
    /// If you are creating a blank table, you may need to specify the type that you wish to give
    /// to the table in future. This is usually a `String`.
    /// Example:
    /// ```
    /// use alinio::table::Table;
    /// let table = Table::new::<String>(vec![], 10);
    /// ```
    pub fn new<T: Into<String>>(data: Vec<Vec<T>>, space: usize) -> Self {
        let mut table = Table::default();
        let mut converted_data = vec![];
        for row in data {
            converted_data.push(row.into_iter().map(|x| x.into()).collect())
        }
        table.data = converted_data;
        table.space = space;
        table
    }

    /// Set the priorities for the columns.
    /// This allows you to control which columns to remove when space is limited.
    /// The higher the number of the column, the more important it is.
    /// For example:
    /// ```
    /// use alinio::table::Table;
    /// let data = vec![
    ///     vec!["First name", "Surname", "Telephone"],
    ///     vec!["John",       "Smith",   "04529834125"],
    /// ];
    /// let mut table = Table::new(data, 24);
    /// // First name is the most important, surname is least important, telephone is second most
    /// // important.
    /// // The higher the number, the more important the column
    /// table.set_priorities(&[2, 0, 1]);
    /// // We can't fit the whole table in 24 spaces, so it looks to see what it can remove.
    /// // In this case, we can remove the surname column in order to make it fit, as the surname column
    /// // has the lowest priority in the table.
    /// println!("{}\n---", table.render().unwrap().join("\n"))
    /// ```
    pub fn set_priorities(&mut self, priorities: &[usize]) {
        self.priorities = priorities.to_vec();
    }

    /// Set the alignment of each cell.
    pub fn set_alignment(&mut self, align: Align) {
        self.align = align;
    }

    /// When `surround` is true, padding will be applied to the sides of the table.
    /// When `surround` is false, the table will take the full width.
    pub fn set_surround(&mut self, surround: bool) {
        self.surround = surround;
    }

    /// Set the space between each cell.
    /// Use this if your terminal size updates.
    pub fn set_space(&mut self, space: usize) {
        self.space = space;
    }

    /// Render this table to rows of strings.
    ///
    /// This will return `None` if there is not enough space to fit the table.
    pub fn render(&self) -> Option<Vec<String>> {
        self.render_partial(0)
    }

    /// Only renders rows after `offset` row. This is particularly useful if you have a table
    /// that you wish to fit into a terminal with a height shorter than the table.
    ///
    /// This will return `None` if there is not enough space to fit the table, or if the offset is
    /// out of bounds
    pub fn render_partial(&self, offset: usize) -> Option<Vec<String>> {
        // Return nothing if there is no data
        if self.data.len().saturating_sub(offset) == 0 {
            return Some(vec![]);
        }
        // Create copy of data
        let mut data: Data = self.data.clone().into_iter().skip(offset).collect();
        // Reform into columns
        let mut columns = vec![];
        for column in 0..data[0].len() {
            let mut this = vec![];
            for row in &data {
                // Returns None if table is in an invalid format
                this.push(row.get(column)?)
            }
            columns.push(this);
        }
        // For each column in this table, work out the maximum space required
        let mut limits = vec![];
        for column in &columns {
            limits.push(find_longest(column));
        }
        // Strip columns until it fits
        let mut pri = self.priorities.clone();
        let mut pad_places = if self.surround {
            columns.len() + 1
        } else {
            columns.len().saturating_sub(1)
        };
        let mut column_count = columns.len().saturating_sub(1);
        while limits.iter().sum::<usize>() + pad_places > self.space {
            // Work out which column to remove
            let rm = pri.iter().min().unwrap_or(&0);
            let rm = pri.iter().position(|x| x == rm).unwrap_or(column_count);
            // Remove from data
            for row in &mut data {
                row.remove(rm);
            }
            // Remove from limits
            limits.remove(rm);
            // Remove from priority
            if !pri.is_empty() {
                pri.remove(rm);
            }
            // Decrement counters
            pad_places = pad_places.saturating_sub(1);
            column_count = column_count.saturating_sub(1);
        }
        // Correctly align each item within said columns and format them
        let mut result = vec![];
        for row in data.iter() {
            let mut this = vec![];
            for (column, limit) in row.iter().zip(&limits) {
                // Align cell
                let cell = match self.align {
                    Align::Left => align::left(column, *limit),
                    Align::Right => align::right(column, *limit),
                    Align::Center => align::center(column, *limit),
                };
                this.push(cell.unwrap());
            }
            // Get parts as a vector of &str (for use in align functions)
            let parts = this.iter().map(|x| x.as_str()).collect::<Vec<_>>();
            // Do alignment
            result.push(if self.surround {
                align::around(parts.as_slice(), self.space)
            } else {
                align::between(parts.as_slice(), self.space)
            }?);
        }
        Some(result)
    }
}
