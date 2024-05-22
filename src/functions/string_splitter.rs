
#[derive(Debug)]
pub struct Split<'a, 'b> {
    input: &'a str,
    delimiter: &'b str,
    current_position: usize,
}

impl<'a, 'b> Split<'a, 'b> {
    pub fn new(input: &'a str, delimiter: &'b str) -> Self {
        Self {
            input,
            delimiter,
            current_position: 0,
        }
    }
}

impl<'a, 'b> Iterator for Split<'a, 'b> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_position >= self.input.len() {
            return None;
        }

        let start = self.current_position;
        let remainder = &self.input[start..];

        match remainder.find(self.delimiter) {
            Some(end) => {
                self.current_position += end + self.delimiter.len();
                Some(&self.input[start..start + end])
            },
            None => {
                self.current_position = self.input.len();
                Some(remainder)
            }
        }
    }
}

pub fn split_string(input: String, delimiter: String) {
    let mut splitter = Split::new(&input, &delimiter);

    loop {
        let string = splitter.next();
        if string == None {
            break;
        }
        println!("{:?}", string.unwrap());
    }
}
