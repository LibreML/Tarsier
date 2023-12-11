pub mod read;

use std::any::Any;

struct Row<T> {
    data: T,
}

struct Column<T> {
    name: String,
    rows: Vec<Row<T>>,
}

struct DataFrame {
    columns: Vec<Box<dyn Any>>, // Using trait object to handle different types
}

impl DataFrame {
    fn new() -> DataFrame {
        DataFrame { columns: Vec::new() }
    }

    // Add a new column
    fn add_column<T: 'static>(&mut self, name: String) {
        let column = Column::<T> {
            name,
            rows: Vec::new(),
        };
        self.columns.push(Box::new(column));
    }

    // Add a new row to a specific column
    fn add_row<T: 'static>(&mut self, column_name: &str, data: T) {
        for column in &mut self.columns {
            if let Some(col) = column.downcast_mut::<Column<T>>() {
                if col.name == column_name {
                    col.rows.push(Row { data });
                    return;
                }
            }
        }
        println!("Column not found or type mismatch");
    }

    // Get column by name
    fn get_column_by_name<T: 'static>(&self, name: &str) -> Option<&Column<T>> {
        for column in &self.columns {
            if let Some(col) = column.downcast_ref::<Column<T>>() {
                if col.name == name {
                    return Some(col);
                }
            }
        }
        None
    }

    // Get column by index
    fn get_column_by_index<T: 'static>(&self, index: usize) -> Option<&Column<T>> {
        self.columns.get(index)?.downcast_ref::<Column<T>>()
    }

    // Get a row from a specific column by row index
    fn get_row<T: 'static>(&self, column_name: &str, row_index: usize) -> Option<&T> {
        self.get_column_by_name::<T>(column_name)?.rows.get(row_index).map(|row| &row.data)
    }
}

// Implement Column and Row methods as needed
