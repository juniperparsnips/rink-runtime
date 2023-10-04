use std::{
    borrow::Borrow,
    fmt::{self, Write},
    rc::Rc,
};

use crate::{
    runtime::{
        container::Container,
        control_command::ControlCommand,
        value::Value,
        variable::{ReadCount, VariableReference},
        RuntimeObject,
    },
    runtime_graph::RuntimeGraph,
};

#[derive(Debug)]
pub struct Story<Output> {
    output_text: Output,
    glue: bool,
    cursors: Vec<(Rc<Container>, usize)>,
}

impl<Output> Story<Output>
where
    Output: Write,
{
    fn new<Graph>(graph: Graph, output_text: Output) -> Self
    where
        Graph: Borrow<RuntimeGraph>,
    {
        let cursors = vec![(graph.borrow().root_container.clone(), 0)];
        Self {
            output_text,
            glue: false,
            cursors,
        }
    }

    pub fn new_from_json(ink: &str, output: Output) -> Option<Self> {
        let graph: RuntimeGraph = serde_json::from_str(ink).unwrap();
        Some(Self::new(graph, output))
    }

    pub fn step(&mut self) -> Result<(), ()> {
        let object = self.peek_cursor().ok_or(())?;
        if self.execute(object.clone()) {
            self.advance_cursor()?;
        }
        Ok(())
    }

    fn execute(&mut self, object: RuntimeObject) -> bool {
        match object {
            RuntimeObject::Choice(_choice) => todo!(),
            RuntimeObject::Container(container) => {
                // push first index of container to cursor stack
                self.cursors.push((container.clone(), 0));
                // shouldn't advance cursor
                return false;
            }
            RuntimeObject::ControlCommand(command) => self.command(command),
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
        // should advance cursor
        true
    }

    fn output<Object>(&mut self, object: Object)
    where
        Object: fmt::Display,
    {
        if self.glue {
            todo!()
        }
        self.output_text
            .write_str(&format!("{}", object))
            .expect("Error writing Ink output to output stream");
    }

    fn command(&mut self, command: ControlCommand) {
        match command {
            ControlCommand::Done => {
                // TODO close current thread
            }
            _ => todo!(),
        }
    }

    fn peek_cursor(&self) -> Option<&RuntimeObject> {
        let (container, index) = self.cursors.last()?;
        container.content.get(*index)
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
    use crate::runtime::{value::Value, RuntimeObject, story::Story};

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
        let value = serde_json::from_str(r##""^string""##).unwrap();
        let mut output = String::new();
        let mut story = Story::new(&graph, &mut output);
        assert!(story.execute(value));
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
    fn step_through() {
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

    #[test]
    fn step_through_nested() {
        let graph =
            serde_json::from_str(r##"{"root": ["^1", ["^2", "^3", null], "^4", null], "inkVersion": 21}"##)
                .unwrap();
        let mut output = String::new();
        let mut story = Story::new(&graph, &mut output);
        story.step().unwrap();
        story.step().unwrap();
        story.step().unwrap();
        story.step().unwrap();
        assert!(story.step().is_err());
        assert_eq!(output, "1234");
    }
}
