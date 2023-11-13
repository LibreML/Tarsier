pub mod read;

// Defining the Column struct
// For simplicity, we're assuming the data in each column is of type i32.
// You can later modify this to use generics or enums for different data types.
struct Column {
    name: String,
    data: Vec<i32>,
}

// Defining the DataFrame struct
struct DataFrame {
    columns: Vec<Column>,
}

impl DataFrame {
    // Constructor for a new DataFrame
    fn new() -> DataFrame {
        DataFrame { columns: Vec::new() }
    }

    // Add a new column to the DataFrame
    // This function takes ownership of the column.
    fn add_column(&mut self, column: Column) {
        // You might want to add logic here to check for unique column names
        self.columns.push(column);
    }

    // Access column by name
    // Returns an Option, as the column may not exist
    fn get_column_by_name(&self, name: &str) -> Option<&Column> {
        self.columns.iter().find(|col| col.name == name)
    }

    // Access column by position
    // Returns None if the index is out of bounds
    fn get_column_by_index(&self, index: usize) -> Option<&Column> {
        self.columns.get(index)
    }
}
