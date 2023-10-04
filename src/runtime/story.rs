use std::{
    fmt::{self, Write},
    rc::Rc,
};

use crate::runtime_graph::RuntimeGraph;

use super::{
    container::Container,
    value::Value,
    variable::{ReadCount, VariableReference},
    RuntimeObject,
};

#[derive(Debug)]
pub struct Story<'graph, Output> {
    graph: &'graph RuntimeGraph,
    output_text: Output,
    glue: bool,
    cursors: Vec<(Rc<Container>, usize)>,
}

impl<'graph, Output> Story<'graph, Output> {
    fn new(graph: &'graph RuntimeGraph, output_text: Output) -> Self {
        let cursors = vec![(graph.root_container.clone(), 0)];
        Self {
            graph,
            output_text,
            glue: false,
            cursors,
        }
    }

    fn step(&mut self) -> Result<(), ()>
    where
        Output: Write,
    {
        let object = self.peek_cursor().ok_or(())?;
        self.execute(object.clone());
        self.advance_cursor()?;
        Ok(())
    }

    fn execute(&mut self, object: RuntimeObject)
    where
        Output: Write,
    {
        match object {
            RuntimeObject::Choice(_choice) => todo!(),
            RuntimeObject::Container(container) => {
                // push first index of container to cursor stack
                self.cursors.push((container.clone(), 0))
            }
            RuntimeObject::ControlCommand(_command) => todo!(),
            RuntimeObject::Divert(_divert) => todo!(),
            RuntimeObject::Glue => self.glue = true,
            RuntimeObject::NativeFunctionCall(_call) => todo!(),
            RuntimeObject::Tag(_tag) => todo!(),
            RuntimeObject::Value(value) => match value {
                Value::Int(_) => todo!(),
                Value::Float(_) => todo!(),
                Value::String(string) => self.output(string),
                Value::DivertTarget { target_path: _ } => todo!(),
                Value::VariablePointer {
                    name: _,
                    context_index: _,
                } => todo!(),
            },
            RuntimeObject::VariableAssignment(_assignment) => todo!(),
            RuntimeObject::VariableReference(VariableReference { name: _ }) => todo!(),
            RuntimeObject::ReadCount(ReadCount { target: _ }) => todo!(),
            RuntimeObject::Void => {}
        }
    }

    fn output<Object>(&mut self, object: Object)
    where
        Object: fmt::Display,
        Output: Write,
    {
        if self.glue {
            todo!()
        }
        self.output_text
            .write_str(&format!("{}", object))
            .expect("Error writing Ink output to output stream");
    }

    fn peek_cursor(&self) -> Option<&RuntimeObject> {
        let (container, index) = self.cursors.last()?;
        let object = container.content.get(*index)?;
        Some(object)
    }

    fn advance_cursor(&mut self) -> Result<(), ()> {
        let (mut container, mut index) = self.cursors.pop().ok_or(())?;
        while container.content.len() <= index + 1 {
            // pop all finished containers from stack
            (container, index) = self.cursors.pop().ok_or(())?
        }
        // place index back on cursor stack, incremented
        self.cursors.push((container, index + 1));
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::runtime::{value::Value, RuntimeObject};

    use super::Story;

    #[test]
    fn output_string() {
        let graph = serde_json::from_str(r##"{"root": [null],"inkVersion":21}"##).unwrap();
        let mut output = String::new();
        let mut story = Story::new(&graph, &mut output);
        story.output("string");
        assert_eq!(output, "string");
    }

    #[test]
    fn execute_string() {
        let graph = serde_json::from_str(r##"{"root": [null], "inkVersion": 21}"##).unwrap();
        let mut output = String::new();
        let mut story = Story::new(&graph, &mut output);
        story.execute(serde_json::from_str(r##""^string""##).unwrap());
        assert_eq!(output, "string");
    }

    #[test]
    fn cursor() {
        let graph =
            serde_json::from_str(r##"{"root": ["^1", "^2", "^3", null], "inkVersion": 21}"##)
                .unwrap();
        let mut output = String::new();
        let mut story = Story::new(&graph, &mut output);
        assert_eq!(
            story.peek_cursor(),
            Some(&RuntimeObject::Value(Value::String("1".into())))
        );
        story.advance_cursor().unwrap();
        assert_eq!(
            story.peek_cursor(),
            Some(&RuntimeObject::Value(Value::String("2".into())))
        );
        story.advance_cursor().unwrap();
        assert_eq!(
            story.peek_cursor(),
            Some(&RuntimeObject::Value(Value::String("3".into())))
        );
        assert!(story.advance_cursor().is_err());
    }

    #[test]
    fn stepping() {
        let graph =
            serde_json::from_str(r##"{"root": ["^1", "^2", "^3", null], "inkVersion": 21}"##)
                .unwrap();
        let mut output = String::new();
        let mut story = Story::new(&graph, &mut output);
        story.step().unwrap();
        story.step().unwrap();
        assert!(story.step().is_err());
        assert_eq!(output, "123");
    }
}
