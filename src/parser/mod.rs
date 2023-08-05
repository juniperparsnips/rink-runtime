use std::{error::Error, fmt, io::Read, rc::Rc};

use serde::de::{Deserialize, Deserializer, Error as SerdeError, MapAccess, SeqAccess, Visitor};
use serde_json;

use crate::{
    error::InkError,
    path::Path,
    runtime::{
        choice_point::ChoicePoint,
        container::Container,
        control_command::ControlCommand,
        divert::{Divert, TargetType},
        glue::Glue,
        tag::Tag,
        value::Value,
        variable::{ReadCount, VariableAssignment, VariableReference},
        RuntimeObject,
    },
    runtime_graph::RuntimeGraph,
};

#[cfg(test)]
mod tests;

struct RuntimeGraphVisitor {}

impl RuntimeGraphVisitor {
    fn new() -> Self {
        RuntimeGraphVisitor {}
    }
}

impl<'de> Visitor<'de> for RuntimeGraphVisitor {
    // Our Visitor is going to produce a RuntimeGraph.
    type Value = RuntimeGraph;

    // Format a message stating what data this Visitor expects to receive.
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Runtime graph")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let ink_version = match map.next_entry()? as Option<(&str, u32)> {
            Some(("inkVersion", value)) => Some(value),
            _ => None,
        }
        .ok_or(SerdeError::custom(
            "Invalid runtime graph format, expected inkVersion",
        ))?;

        let container = match map.next_entry()? as Option<(&str, RuntimeObject)> {
            Some(("root", value)) => match value {
                RuntimeObject::Container(container) => Some(container),
                _ => None,
            },
            _ => None,
        }
        .ok_or(SerdeError::custom(
            "Invalid runtime graph format, expected root",
        ))?;

        let _list_defs = match map.next_entry()? as Option<(&str, ListDefinitions)> {
            Some(("listDefs", value)) => Some(value),
            _ => None,
        }
        .ok_or(SerdeError::custom(
            "Invalid runtime graph format, expected listDefs",
        ))?;

        Ok(RuntimeGraph::new(ink_version, container))
    }
}

impl<'de> Deserialize<'de> for RuntimeGraph {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // Instantiate our Visitor and ask the Deserializer to drive
        // it over the input data, resulting in an instance of RuntimeGraph.
        deserializer.deserialize_map(RuntimeGraphVisitor::new())
    }
}

struct RuntimeObjectVisitor {}

impl RuntimeObjectVisitor {
    fn new() -> Self {
        RuntimeObjectVisitor {}
    }
}

impl<'de> Visitor<'de> for RuntimeObjectVisitor {
    // Our Visitor is going to produce a RuntimeObject.
    type Value = RuntimeObject;

    // Format a message stating what data this Visitor expects to receive.
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Runtime object")
    }

    fn visit_i8<E>(self, v: i8) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(RuntimeObject::Value(Value::Int(v as i32)))
    }
    fn visit_i16<E>(self, v: i16) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(RuntimeObject::Value(Value::Int(v as i32)))
    }

    fn visit_i32<E>(self, v: i32) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(RuntimeObject::Value(Value::Int(v)))
    }

    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(RuntimeObject::Value(Value::Int(v as i32)))
    }

    fn visit_u8<E>(self, v: u8) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(RuntimeObject::Value(Value::Int(v as i32)))
    }

    fn visit_u16<E>(self, v: u16) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(RuntimeObject::Value(Value::Int(v as i32)))
    }

    fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(RuntimeObject::Value(Value::Int(v as i32)))
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(RuntimeObject::Value(Value::Int(v as i32)))
    }

    fn visit_f32<E>(self, v: f32) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(RuntimeObject::Value(Value::Float(v)))
    }

    fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(RuntimeObject::Value(Value::Float(v as f32)))
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: SerdeError,
    {
        if v.starts_with("^") {
            return Ok(RuntimeObject::Value(Value::String(
                v.chars().skip(1).collect(),
            )));
        }

        match v {
            "\n" => Ok(RuntimeObject::Value(Value::String("\n".to_string()))),

            // Glue
            "<>" => Ok(RuntimeObject::Glue(Glue::Bidirectional)),
            "G<" => Ok(RuntimeObject::Glue(Glue::Left)),
            "G>" => Ok(RuntimeObject::Glue(Glue::Right)),

            // Control Commands
            "ev" => Ok(RuntimeObject::ControlCommand(ControlCommand::EvalStart)),
            "out" => Ok(RuntimeObject::ControlCommand(ControlCommand::EvalOutput)),
            "/ev" => Ok(RuntimeObject::ControlCommand(ControlCommand::EvalEnd)),
            "du" => Ok(RuntimeObject::ControlCommand(ControlCommand::Duplicate)),
            "pop" => Ok(RuntimeObject::ControlCommand(
                ControlCommand::PopEvaluatedValue,
            )),
            "~ret" => Ok(RuntimeObject::ControlCommand(ControlCommand::PopFunction)),
            "->->" => Ok(RuntimeObject::ControlCommand(ControlCommand::PopTunnel)),
            "str" => Ok(RuntimeObject::ControlCommand(ControlCommand::BeginString)),
            "/str" => Ok(RuntimeObject::ControlCommand(ControlCommand::EndString)),
            "nop" => Ok(RuntimeObject::ControlCommand(ControlCommand::NoOp)),
            "choiceCnt" => Ok(RuntimeObject::ControlCommand(ControlCommand::ChoiceCount)),
            "turns" => Ok(RuntimeObject::ControlCommand(ControlCommand::TurnsSince)),
            "readc" => Ok(RuntimeObject::ControlCommand(ControlCommand::ReadCount)),
            "rnd" => Ok(RuntimeObject::ControlCommand(ControlCommand::Random)),
            "srnd" => Ok(RuntimeObject::ControlCommand(ControlCommand::SeedRandom)),
            "visit" => Ok(RuntimeObject::ControlCommand(ControlCommand::VisitIndex)),
            "seq" => Ok(RuntimeObject::ControlCommand(
                ControlCommand::SequenceShuffleIndex,
            )),
            "thread" => Ok(RuntimeObject::ControlCommand(ControlCommand::StartThread)),
            "done" => Ok(RuntimeObject::ControlCommand(ControlCommand::Done)),
            "end" => Ok(RuntimeObject::ControlCommand(ControlCommand::End)),
            "listInt" => Ok(RuntimeObject::ControlCommand(ControlCommand::ListFromInt)),
            "range" => Ok(RuntimeObject::ControlCommand(ControlCommand::ListRange)),

            // Native functions
            //Some("L^") => {},

            // Void
            "void" => Ok(RuntimeObject::Void),

            _ => Err(SerdeError::custom("Invalid String")),
        }
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let mut opt_key: Option<&str> = map.next_key()?;
        if let &Some(key) = &opt_key {
            match key {
                // Divert target value to path
                "^->" => {
                    let value: Option<&str> = map.next_value()?;
                    match value {
                        Some(target) => match Path::from_str(target) {
                            Some(path) => {
                                return Ok(RuntimeObject::Value(Value::DivertTarget(path)))
                            }
                            _ => return Err(SerdeError::custom("Cannot parse target path")),
                        },
                        _ => return Err(SerdeError::custom("Unexpected divert target value type")),
                    }
                }

                // VariablePointerValue
                "^var" => {
                    let value: Option<&str> = map.next_value()?;
                    match value {
                        Some(name) => {
                            let mut context_index = -1;
                            if let Some(("ci", value)) = map.next_entry()? as Option<(&str, i32)> {
                                context_index = value;
                            }
                            return Ok(RuntimeObject::Value(Value::VariablePointer(
                                name.to_owned(),
                                context_index,
                            )));
                        }
                        _ => {
                            return Err(SerdeError::custom(
                                "Unexpected variable pointer value type",
                            ))
                        }
                    }
                }

                // Divert
                "->" => {
                    let value: Option<&str> = map.next_value()?;
                    match value {
                        Some(target) => {
                            let mut divert = Divert::new();

                            let entry: Option<(&str, bool)> = map.next_entry()?;
                            match entry {
                                // Case {"->": "variableTarget", "var": true}
                                Some(("var", true)) => {
                                    divert.set_target(TargetType::Name(target.to_owned()));

                                    // Case {"->": "variableTarget", "var": true, "c": true}
                                    if let Some(("c", true)) = map.next_entry()? {
                                        divert.set_is_conditional(true);
                                    }
                                }
                                _ => {
                                    match Path::from_str(target) {
                                        Some(path) => divert.set_target(TargetType::Path(path)),
                                        _ => {
                                            return Err(SerdeError::custom(
                                                "Cannot parse divert target path",
                                            ))
                                        }
                                    }

                                    // Case {"->": "variableTarget", "c": true}
                                    if let Some(("c", true)) = entry {
                                        divert.set_is_conditional(true);
                                    }
                                }
                            }
                            return Ok(RuntimeObject::Divert(divert));
                        }
                        _ => return Err(SerdeError::custom("Unexpected divert type")),
                    }
                }

                // Function Call
                "f()" => {
                    let value: Option<&str> = map.next_value()?;
                    match value {
                        Some(target) => {
                            let mut divert = Divert::new_function();

                            match Path::from_str(target) {
                                Some(path) => divert.set_target(TargetType::Path(path)),
                                _ => return Err(SerdeError::custom("Cannot parse target path")),
                            }

                            // Case {"f()": "path.to.func", "c": true}
                            if let Some(("c", true)) = map.next_entry()? {
                                divert.set_is_conditional(true);
                            }

                            return Ok(RuntimeObject::Divert(divert));
                        }
                        _ => return Err(SerdeError::custom("Unexpected function call type")),
                    }
                }

                // Tunnel
                "->t->" => {
                    let value: Option<&str> = map.next_value()?;
                    match value {
                        Some(target) => {
                            let mut divert = Divert::new_tunnel();

                            match Path::from_str(target) {
                                Some(path) => divert.set_target(TargetType::Path(path)),
                                _ => return Err(SerdeError::custom("Cannot parse target path")),
                            }

                            // Case {"->t->": "path.tunnel", "c": true}
                            if let Some(("c", true)) = map.next_entry()? {
                                divert.set_is_conditional(true);
                            }

                            return Ok(RuntimeObject::Divert(divert));
                        }
                        _ => return Err(SerdeError::custom("Unexpected tunnel type")),
                    }
                }

                // External function
                "x()" => {
                    let value: Option<&str> = map.next_value()?;
                    match value {
                        Some(target) => {
                            let mut divert = Divert::new_external_function();

                            match Path::from_str(target) {
                                Some(path) => divert.set_target(TargetType::Path(path)),
                                _ => return Err(SerdeError::custom("Cannot parse target path")),
                            }

                            // Case {"x()": "externalFuncName", "exArgs": 5}
                            if let Some(("exArgs", external_args)) = map.next_entry()? {
                                divert.set_external_args(external_args);
                            }

                            // Case {"x()": "externalFuncName", "exArgs": 5, "c": true}
                            if let Some(("c", true)) = map.next_entry()? {
                                divert.set_is_conditional(true);
                            }

                            return Ok(RuntimeObject::Divert(divert));
                        }
                        _ => return Err(SerdeError::custom("Unexpected external function type")),
                    }
                }

                // Choice
                "*" => {
                    let value: Option<&str> = map.next_value()?;
                    match value {
                        Some(target) => {
                            let mut choice = ChoicePoint::new();

                            match Path::from_str(target) {
                                Some(path) => choice.set_path_on_choice(path),
                                _ => return Err(SerdeError::custom("Cannot parse choice path")),
                            }

                            if let Some(("flg", flags)) = map.next_entry()? {
                                choice.set_flags(flags);
                            }

                            return Ok(RuntimeObject::Choice(choice));
                        }
                        _ => return Err(SerdeError::custom("Unexpected choice type")),
                    }
                }

                // Variable reference
                "VAR?" => {
                    let value: Option<&str> = map.next_value()?;
                    match value {
                        Some(name) => {
                            return Ok(RuntimeObject::VariableReference(VariableReference::new(
                                name.to_owned(),
                            )))
                        }
                        _ => return Err(SerdeError::custom("Unexpected variable reference type")),
                    }
                }

                // Read Count
                "CNT?" => {
                    let value: Option<&str> = map.next_value()?;
                    match value {
                        Some(target) => match Path::from_str(target) {
                            Some(path) => {
                                return Ok(RuntimeObject::ReadCount(ReadCount::new(path)))
                            }
                            _ => return Err(SerdeError::custom("Cannot parse read count target")),
                        },
                        _ => return Err(SerdeError::custom("Unexpected read count type")),
                    }
                }

                // Variable assignment
                "VAR=" => {
                    let value: Option<&str> = map.next_value()?;
                    match value {
                        Some(name) => {
                            if let Some(("re", re)) = map.next_entry()? as Option<(&str, bool)> {
                                return Ok(RuntimeObject::VariableAssignment(
                                    VariableAssignment::new(name.to_owned(), !re, true),
                                ));
                            }

                            return Ok(RuntimeObject::VariableAssignment(VariableAssignment::new(
                                name.to_owned(),
                                true,
                                true,
                            )));
                        }
                        _ => return Err(SerdeError::custom("Unexpected variable assignment type")),
                    }
                }

                // Temporary variable
                "temp=" => {
                    let value: Option<&str> = map.next_value()?;
                    match value {
                        Some(name) => {
                            return Ok(RuntimeObject::VariableAssignment(VariableAssignment::new(
                                name.to_owned(),
                                true,
                                false,
                            )))
                        }
                        _ => return Err(SerdeError::custom("Unexpected temporary variable type")),
                    }
                }

                // Tag
                "#" => {
                    let value: Option<&str> = map.next_value()?;
                    match value {
                        Some(tag) => return Ok(RuntimeObject::Tag(Tag::new(tag.to_owned()))),
                        _ => return Err(SerdeError::custom("Unexpected temp var name type")),
                    }
                }

                // List
                "list" => return Err(SerdeError::custom("TODO")),

                _ => {}
            }
        }

        let mut opt_container: Option<Container> = None;

        while let Some(key) = opt_key {
            match key {
                // Container name
                "#n" => {
                    let value: Option<&str> = map.next_value()?;
                    match value {
                        Some(name) => {
                            if opt_container.is_none() {
                                opt_container = Some(Container::new());
                            }

                            if let Some(ref mut container_ref) = opt_container.as_mut() {
                                container_ref.set_name(name.to_owned());
                            }
                        }
                        _ => return Err(SerdeError::custom("Unexpected container name type")),
                    }
                }

                // Container flags
                "#f" => {
                    let value: Option<u8> = map.next_value()?;
                    match value {
                        Some(flags) => {
                            if opt_container.is_none() {
                                opt_container = Some(Container::new());
                            }

                            if let Some(ref mut container_ref) = opt_container.as_mut() {
                                container_ref.set_count_flags(flags);
                            }
                        }
                        _ => return Err(SerdeError::custom("Unexpected container flags type")),
                    }
                }

                // Sub-container
                _ => {
                    let value: Option<RuntimeObject> = map.next_value()?;
                    match value {
                        Some(obj) => {
                            if let RuntimeObject::Container(mut sub_container_rc) = obj {
                                if opt_container.is_none() {
                                    opt_container = Some(Container::new());
                                }

                                match Rc::get_mut(&mut sub_container_rc) {
                                    Some(sub_container) => sub_container.set_name(key.to_owned()),
                                    _ => {
                                        return Err(SerdeError::custom(
                                            "Fail to get mutable sub-container",
                                        ))
                                    }
                                }

                                if let Some(ref mut container_ref) = opt_container.as_mut() {
                                    container_ref
                                        .add_child(RuntimeObject::Container(sub_container_rc));
                                }
                            }
                        }
                        _ => return Err(SerdeError::custom("Unexpected sub-container type")),
                    }
                }
            }

            opt_key = map.next_key()?;
        }

        if let Some(container) = opt_container {
            return Ok(RuntimeObject::Container(Rc::new(container)));
        }

        Err(SerdeError::custom(
            "Runtime Object dictionary match not found",
        ))
    }

    fn visit_seq<V>(self, mut seq: V) -> Result<Self::Value, V::Error>
    where
        V: SeqAccess<'de>,
    {
        let mut runtime_objects: Vec<RuntimeObject> = Vec::new();

        let mut opt_child: Option<RuntimeObject> = seq.next_element()?;
        while let Some(child) = opt_child {
            opt_child = seq.next_element()?;

            if opt_child.is_some() {
                runtime_objects.push(child);
            } else {
                if let RuntimeObject::Container(mut container_rc) = child {
                    match Rc::get_mut(&mut container_rc) {
                        Some(container) => container.prepend(runtime_objects),
                        _ => return Err(SerdeError::custom("Fail to get mutable container")),
                    }

                    return Ok(RuntimeObject::Container(container_rc));
                }
            }
        }

        Ok(RuntimeObject::Container(Rc::new(
            Container::from_runtime_object_vec(runtime_objects),
        )))
    }

    fn visit_unit<E>(self) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(RuntimeObject::Null)
    }
}

impl<'de> Deserialize<'de> for RuntimeObject {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // Instantiate our Visitor and ask the Deserializer to drive
        // it over the input data, resulting in an instance of RuntimeObject.
        deserializer.deserialize_map(RuntimeObjectVisitor::new())
    }
}

// TODO
struct ListDefinitions {}

struct ListDefinitionsVisitor {}

impl ListDefinitionsVisitor {
    fn new() -> Self {
        ListDefinitionsVisitor {}
    }
}

impl<'de> Visitor<'de> for ListDefinitionsVisitor {
    // Our Visitor is going to produce a RuntimeGraph.
    type Value = ListDefinitions;

    // Format a message stating what data this Visitor expects to receive.
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("List definitions")
    }

    fn visit_map<A>(self, _map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        Ok(ListDefinitions {})
    }
}

impl<'de> Deserialize<'de> for ListDefinitions {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // Instantiate our Visitor and ask the Deserializer to drive
        // it over the input data, resulting in an instance of ListDefinitions.
        deserializer.deserialize_map(ListDefinitionsVisitor::new())
    }
}

pub struct RuntimeGraphBuilder {}

impl RuntimeGraphBuilder {
    pub fn from_str(s: &str) -> Result<RuntimeGraph, InkError> {
        serde_json::from_str(s).map_err(|e| InkError::from(e))
    }

    pub fn from_slice(v: &[u8]) -> Result<RuntimeGraph, InkError> {
        serde_json::from_slice(v).map_err(|e| InkError::from(e))
    }

    pub fn from_reader<R>(rdr: R) -> Result<RuntimeGraph, InkError>
    where
        R: Read,
    {
        serde_json::from_reader(rdr).map_err(|e| InkError::from(e))
    }
}
