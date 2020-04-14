#[derive(PartialEq)]
pub enum ColumnAlignment {
    Left,
    Right,
}

pub struct Column<'a, 'b, T> {
    pub name: &'a str,
    pub extractor: Box<dyn Fn(&T) -> String + 'b>,
    pub alignment: ColumnAlignment,
}

pub struct Table<'a, 'b, T> {
    pub columns: Vec<Column<'a, 'b, T>>,
}

// "If you just give it enough lifetimes, it'll all work out, right? ...right?"
// "If you have more than N levels of indentation, you're screwed anyway, and
// you should fix your program."
impl<'a, 'b, 'c, T: 'a> Table<'b, 'c, T> {
    pub fn render<I>(&self, values: I) -> String
    where
        I: Iterator<Item = &'a T>,
    {
        let mut table: Vec<Vec<String>> = Vec::new();
        let mut header_row = Vec::new();
        for column in &self.columns {
            header_row.push(column.name.to_string());
        }
        table.push(header_row);
        for value in values {
            let mut row = Vec::new();
            for column in &self.columns {
                row.push((column.extractor)(value));
            }
            table.push(row);
        }
        let mut max_length: Vec<usize> = Vec::new();
        for _ in &self.columns {
            max_length.push(0);
        }
        for row in table.iter() {
            for (i, cell) in row.iter().enumerate() {
                if max_length[i] < cell.len() {
                    max_length[i] = cell.len();
                }
            }
        }
        let mut table_string = String::new();
        for row in table {
            table_string.push_str("|");
            for (i, cell) in row.iter().enumerate() {
                table_string.push_str(" ");
                if self.columns[i].alignment == ColumnAlignment::Right {
                    for _ in 0..(max_length[i] - cell.len()) {
                        table_string.push_str(" ");
                    }
                }
                table_string.push_str(cell);
                if self.columns[i].alignment == ColumnAlignment::Left {
                    for _ in 0..(max_length[i] - cell.len()) {
                        table_string.push_str(" ");
                    }
                }
                table_string.push_str(" |");
            }
            table_string.push_str("\n");
        }
        table_string
    }
}

