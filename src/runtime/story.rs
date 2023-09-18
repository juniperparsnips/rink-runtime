// use std::fmt::{self, Write};

// use crate::runtime_graph::RuntimeGraph;

// use super::{value::Value, RuntimeObject, glue::Glue};

// pub struct Story<Output> {
//     graph: RuntimeGraph,
//     output_text: Output,
//     glue: bool,
// }

// impl<Output> Story<Output> {
//     fn execute(&mut self, object: RuntimeObject)
//     where
//         Output: Write,
//     {
//         match object {
//             RuntimeObject::Choice(_) => todo!(),
//             RuntimeObject::Container(_) => todo!(),
//             RuntimeObject::ControlCommand(_) => todo!(),
//             RuntimeObject::Divert(_) => todo!(),
//             RuntimeObject::Glue(a) => match a {
//                 Glue::Bidirectional => todo!(),
//                 Glue::Left => todo!(),
//                 Glue::Right => todo!(),
//             },
//             RuntimeObject::NativeFunctionCall(_) => todo!(),
//             RuntimeObject::Tag(_) => todo!(),
//             RuntimeObject::Value(value) => match value {
//                 Value::Int(_) => todo!(),
//                 Value::Float(_) => todo!(),
//                 Value::String(string) => self.output(string),
//                 Value::DivertTarget { target_path } => todo!(),
//                 Value::VariablePointer {
//                     name,
//                     context_index,
//                 } => todo!(),
//             },
//             RuntimeObject::VariableAssignment(_) => todo!(),
//             RuntimeObject::VariableReference(_) => todo!(),
//             RuntimeObject::ReadCount(_) => todo!(),
//             RuntimeObject::Void => {}
//         }
//     }

//     fn output<Object>(&mut self, object: Object)
//     where
//         Object: fmt::Display,
//         Output: Write,
//     {
//         if self.glue {

//         }
//         self.output_text
//             .write_str(&format!("{}", object))
//             .expect("Error writing Ink output to output stream");
//     }
// }
