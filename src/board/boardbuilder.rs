use std::io;
use crate::board::Board;
use super::Field;

struct BoardBuilder<S: BoardBuilderState> {
    width: u32,
    height: u32,
    fields: Vec<Field>,
    _marker: std::marker::PhantomData<S>,
}

impl BoardBuilder<SizeKnown> {
    fn new(width: u32, height: u32) -> Self {
        BoardBuilder {
            width,
            height,
            fields: Vec::new(),
            _marker: Default::default(),
        }
    }

    fn fields_from_stdin(mut self) -> BoardBuilder<Complete> {
        let mut input_line = String::new();
        for i in 0..self.height as usize {
            for j in 0..self.width as usize {
                io::stdin().read_line(&mut input_line).unwrap();
                let inputs = input_line.split(" ").collect::<Vec<_>>();

                self.fields.push(Field::from_input_line(&inputs));
            }
        }

        BoardBuilder {
            width: self.width,
            height: self.height,
            fields:  self.fields,
            _marker: Default::default(),
        }
    }
}

impl BoardBuilder<Complete> {
    fn build(self) -> Board {
        Board {
            width: self.width,
            height: self.height,
            fields: self.fields,
        }
    }
}


enum SizeKnown {}
enum Complete {}
trait BoardBuilderState {}

impl BoardBuilderState for SizeKnown {}
impl BoardBuilderState for Complete {}
