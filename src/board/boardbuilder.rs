use std::io;
use super::Board;
use super::Field;

pub struct BoardBuilder<S: BoardBuilderState> {
    width: u32,
    height: u32,
    my_matter: u32,
    opponent_matter: u32,
    fields: Vec<Field>,
    _marker: std::marker::PhantomData<S>,
}

impl BoardBuilder<SizeKnown> {
    pub fn new(width: u32, height: u32) -> Self {
        BoardBuilder {
            width,
            height,
            my_matter: 0,
            opponent_matter: 0,
            fields: Vec::new(),
            _marker: Default::default(),
        }
    }

    pub fn fields_from_stdin(mut self) -> BoardBuilder<Complete> {

        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();

        let my_matter = parse_input!(inputs[0], u32);
        let opponent_matter = parse_input!(inputs[1], u32);

        for i in 0..self.height as usize {
            for j in 0..self.width as usize {
                let mut input_line = String::new();
                io::stdin().read_line(&mut input_line).unwrap();
                let inputs = input_line.trim().split([' ', '\n']).collect::<Vec<_>>();

                self.fields.push(Field::from_input_line(&inputs, j as u32, i as u32));
            }
        }

        BoardBuilder {
            width: self.width,
            height: self.height,
            my_matter,
            opponent_matter,
            fields:  self.fields,
            _marker: Default::default(),
        }
    }
}

impl BoardBuilder<Complete> {
    pub fn build(self) -> Board {
        Board {
            width: self.width,
            height: self.height,
            my_matter: self.my_matter,
            opponent_matter: self.opponent_matter,
            fields: self.fields,
        }
    }
}


pub enum SizeKnown {}
pub enum Complete {}
pub trait BoardBuilderState {}

impl BoardBuilderState for SizeKnown {}
impl BoardBuilderState for Complete {}
