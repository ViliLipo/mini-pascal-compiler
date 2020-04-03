use std::fs;

pub struct Source {
    column: usize,
    row: usize,
    lines: Vec<Vec<char>>,
}

impl Source {
    fn move_index_over_line(&mut self) {
        if self.column >= self.lines[self.row].len() {
            self.row = self.row + 1;
            self.column = 0;
        }
    }

    pub fn end_of_file(&self) -> bool {
        self.row >= self.lines.len()
            || (self.row + 1 == self.lines.len() && self.column == self.lines[self.row].len())
    }

    pub fn get_next_char(&mut self) -> char {
        if self.end_of_file() {
            '\0'
        } else {
            let value = self.lines[self.row][self.column];
            self.column = self.column + 1;
            self.move_index_over_line();
            value
        }
    }

    pub fn peek(&self) -> char {
        if self.end_of_file() {
            '\0'
        } else {
            if self.column < self.lines[self.row].len() {
                self.lines[self.row][self.column]
            } else if self.row + 1 < self.lines.len() {
                self.lines[self.row + 1][0]
            } else {
                '\0'
            }
        }
    }

    fn reverse_over_line(&mut self) {
        self.row = self.row - 1;
        self.column = self.lines[self.row].len() - 1;
    }

    pub fn reverse(&mut self) {
        if self.column == 0 && self.row == 0 {
            return;
        } else if self.column == 0 {
            self.reverse_over_line()
        } else {
            self.column = self.column - 1;
        }
    }

    pub fn get_row(&self) -> usize {
        self.row
    }

    pub fn get_column(&self) -> usize {
        self.column
    }
}

pub fn create_source(text: String) -> Source {
    let mut v: Vec<Vec<char>> = Vec::new();
    for line in text.lines() {
        let mut lv: Vec<char> = Vec::new();
        for c in line.chars() {
            lv.push(c);
        }
        lv.push('\n');
        v.push(lv);
    }
    Source {
        column: 0,
        row: 0,
        lines: v,
    }
}

pub fn read_file(filename: &String) -> Source {
    let contents = fs::read_to_string(filename).expect("Something went wrog reading the file");
    create_source(contents)
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_next_char() {
        let t = "\
        var i: int := 3;\n
        var b: int := i + 5;\n";
        let text = String::from(t);
        let mut source = create_source(text);
        assert_eq!(source.get_next_char(), 'v');
        assert_eq!(source.get_next_char(), 'a');
        assert_eq!(source.get_next_char(), 'r');
        let mut c = '\0';
        while !source.end_of_file() {
            c = source.get_next_char();
        }
        assert_eq!(c, '\n');
    }
}
