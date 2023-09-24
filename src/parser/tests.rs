use crate::runtime::divert::PushPopType;

use super::*;

#[cfg(test)]
mod tests {
    use crate::{
        runtime::{
            container::Container,
            control_command::ControlCommand,
            divert::{Divert, PushPopType, TargetType},
            native_function_call::NativeFunctionCall,
            value::Value,
            RuntimeObject,
        },
        runtime_graph::RuntimeGraph,
    };

    #[test]
    fn value_int_test() {
        let json = "[42]";
        let runtime_objects: Vec<RuntimeObject> = serde_json::from_str(json).unwrap();
        match runtime_objects.get(0).unwrap() {
            &RuntimeObject::Value(ref value) => match value {
                &Value::Int(int_value) => assert_eq!(int_value, 42),
                _ => assert!(false),
            },
            _ => assert!(false),
        }
    }

    #[test]
    fn value_float_test() {
        let json = "[3.14159265359]";
        let runtime_objects: Vec<RuntimeObject> = serde_json::from_str(json).unwrap();
        match runtime_objects.get(0).unwrap() {
            &RuntimeObject::Value(ref value) => match value {
                &Value::Float(float_value) => assert_eq!(float_value, 3.14159265359),
                _ => assert!(false),
            },
            _ => assert!(false),
        }
    }

    #[test]
    fn value_string_test() {
        let json = "[\"^I looked at Monsieur Fogg\"]";
        let runtime_objects: Vec<RuntimeObject> = serde_json::from_str(json).unwrap();
        match runtime_objects.get(0).unwrap() {
            &RuntimeObject::Value(ref value) => match value {
                &Value::String(ref string_value) => {
                    assert_eq!(string_value, "I looked at Monsieur Fogg")
                }
                _ => assert!(false),
            },
            _ => assert!(false),
        }
    }

    #[test]
    fn value_divert_target_test() {
        let json = "{\"^->\":\"0.g-0.2.$r1\"}";
        let runtime_object: RuntimeObject = serde_json::from_str(json).unwrap();
        match runtime_object {
            RuntimeObject::Value(value) => match value {
                Value::DivertTarget { target_path } => {
                    assert_eq!(target_path.to_string(), "0.g-0.2.$r1")
                }
                _ => assert!(false),
            },
            _ => assert!(false),
        }
    }

    #[test]
    fn value_variable_pointer_test() {
        let json = "{\"^var\": \"varname\", \"ci\": 0}";
        let runtime_object: RuntimeObject = serde_json::from_str(json).unwrap();
        match runtime_object {
            RuntimeObject::Value(value) => match value {
                Value::VariablePointer {
                    name,
                    context_index,
                } => {
                    assert_eq!(name, "varname");
                    assert_eq!(context_index, 0);
                }
                _ => assert!(false),
            },
            _ => assert!(false),
        }
    }

    #[test]
    fn newline_test() {
        let json = r##"["\n"]"##;
        let runtime_objects: Vec<RuntimeObject> = serde_json::from_str(json).unwrap();
        match runtime_objects.get(0).unwrap() {
            RuntimeObject::Value(value) => match value {
                Value::String(string_value) => assert_eq!(string_value, "\n"),
                _ => assert!(false),
            },
            _ => assert!(false),
        }
    }

    #[test]
    fn glue_test() {
        let json = "\"<>\"";

        let runtime_object: RuntimeObject = serde_json::from_str(json).unwrap();

        match runtime_object {
            RuntimeObject::Glue => {}
            _ => assert!(false),
        }
    }

    #[test]
    fn native_functions_test() {
        let json = r#"[
            "+",
            "-",
            "/",
            "*",
            "%",
            "_",
            "==",
            ">",
            "<",
            ">=",
            "<=",
            "!=",
            "!",
            "&&",
            "||",
            "MIN",
            "MAX"
]"#;
        let native_commands = vec![
            NativeFunctionCall::Plus,
            NativeFunctionCall::Minus,
            NativeFunctionCall::Divide,
            NativeFunctionCall::Multiply,
            NativeFunctionCall::Modulo,
            NativeFunctionCall::UnaryMinus,
            NativeFunctionCall::Eq,
            NativeFunctionCall::GT,
            NativeFunctionCall::LT,
            NativeFunctionCall::GEq,
            NativeFunctionCall::LEq,
            NativeFunctionCall::NEq,
            NativeFunctionCall::UnaryNot,
            NativeFunctionCall::And,
            NativeFunctionCall::Or,
            NativeFunctionCall::Min,
            NativeFunctionCall::Max,
        ];
        let runtime_objects: Vec<RuntimeObject> = serde_json::from_str(json).unwrap();
        assert_eq!(native_commands.len(), runtime_objects.len());

        for (i, runtime_object) in runtime_objects.iter().enumerate() {
            let control_command = native_commands.get(i).unwrap();

            match runtime_object {
                RuntimeObject::NativeFunctionCall(value) => assert_eq!(value, control_command),
                _ => assert!(false),
            }
        }
    }

    #[test]
    fn control_command_test() {
        let json = r#"[
            "ev",
            "out",
            "/ev",
            "du",
            "pop",
            "~ret",
            "->->",
            "str",
            "/str",
            "nop",
            "choiceCnt",
            "turns",
            "readc",
            "rnd",
            "srnd",
            "visit",
            "seq",
            "thread",
            "done",
            "end",
            "listInt",
            "range"
]"#;
        let control_commands: Vec<ControlCommand> = vec![
            ControlCommand::EvalStart,
            ControlCommand::EvalOutput,
            ControlCommand::EvalEnd,
            ControlCommand::Duplicate,
            ControlCommand::PopEvaluatedValue,
            ControlCommand::PopFunction,
            ControlCommand::PopTunnel,
            ControlCommand::BeginString,
            ControlCommand::EndString,
            ControlCommand::NoOp,
            ControlCommand::ChoiceCount,
            ControlCommand::TurnsSince,
            ControlCommand::ReadCount,
            ControlCommand::Random,
            ControlCommand::SeedRandom,
            ControlCommand::VisitIndex,
            ControlCommand::SequenceShuffleIndex,
            ControlCommand::StartThread,
            ControlCommand::Done,
            ControlCommand::End,
            ControlCommand::ListFromInt,
            ControlCommand::ListRange,
        ];

        let runtime_objects: Vec<RuntimeObject> = serde_json::from_str(json).unwrap();
        assert_eq!(control_commands.len(), runtime_objects.len());

        for (i, runtime_object) in runtime_objects.iter().enumerate() {
            let control_command = control_commands.get(i).unwrap();

            match runtime_object {
                RuntimeObject::ControlCommand(value) => assert_eq!(value, control_command),
                _ => assert!(false),
            }
        }
    }

    #[test]
    fn void_test() {
        let json = "[\"void\"]";
        let runtime_objects: Vec<RuntimeObject> = serde_json::from_str(json).unwrap();
        assert_eq!(*runtime_objects.get(0).unwrap(), RuntimeObject::Void);
    }

    #[test]
    fn divert_test() {
        let json = "{\"->\": \".^.s\"}";
        let runtime_object: RuntimeObject = serde_json::from_str(json).unwrap();
        match runtime_object {
            RuntimeObject::Divert(divert) => {
                match divert.target {
                    TargetType::Path(path) => {
                        assert_eq!(path.to_string(), ".^.s");
                    }
                    _ => assert!(false),
                }

                assert_eq!(divert.stack_push_type, PushPopType::None);
                assert_eq!(divert.pushes_to_stack, false);
                assert_eq!(divert.is_conditional, false);
            }
            _ => assert!(false),
        }
    }

    #[test]
    fn divert_conditional_test() {
        let json = "{\"->\": \".^.s\", \"c\": true}";
        let runtime_object: RuntimeObject = serde_json::from_str(json).unwrap();
        match runtime_object {
            RuntimeObject::Divert(divert) => {
                assert_eq!(divert.is_conditional, true);
            }
            _ => assert!(false),
        }
    }

    #[test]
    fn divert_with_var_test() {
        let json = "{\"->\":\"$r\",\"var\":true}";
        let runtime_object: RuntimeObject = serde_json::from_str(json).unwrap();
        match runtime_object {
            RuntimeObject::Divert(divert) => {
                match divert.target {
                    TargetType::VarName(target_name) => {
                        assert_eq!(target_name, "$r");
                    }
                    _ => assert!(false),
                }

                assert_eq!(divert.stack_push_type, PushPopType::None);
                assert_eq!(divert.pushes_to_stack, false);
                assert_eq!(divert.is_conditional, false);
            }
            _ => assert!(false),
        }
    }

    #[test]
    fn function_call_test() {
        let json = "{\"f()\": \"0.g-0.2.c.12.0.c.11.g-0.2.c.$r2\"}";
        let runtime_object: RuntimeObject = serde_json::from_str(json).unwrap();
        match runtime_object {
            RuntimeObject::Divert(divert) => {
                match divert.target {
                    TargetType::Path(ref path) => {
                        assert_eq!(path.to_string(), "0.g-0.2.c.12.0.c.11.g-0.2.c.$r2");
                    }
                    _ => assert!(false),
                }

                assert_eq!(divert.stack_push_type, PushPopType::Function);
                assert_eq!(divert.pushes_to_stack, true);
                assert_eq!(divert.is_conditional, false);
            }
            _ => assert!(false),
        }
    }

    #[test]
    fn function_call_conditional_test() {
        let json = "{\"f()\": \".^.s\", \"c\": true}";
        let runtime_object: RuntimeObject = serde_json::from_str(json).unwrap();
        match runtime_object {
            RuntimeObject::Divert(divert) => {
                assert_eq!(divert.is_conditional, true);
            }
            _ => assert!(false),
        }
    }

    #[test]
    fn tunnel_test() {
        let json = "{\"->t->\": \"0.g-0.2.c.12.0.c.11.g-0.2.$r1\"}";
        let runtime_object: RuntimeObject = serde_json::from_str(json).unwrap();
        match runtime_object {
            RuntimeObject::Divert(divert) => {
                match divert.target {
                    TargetType::Path(ref path) => {
                        assert_eq!(path.to_string(), "0.g-0.2.c.12.0.c.11.g-0.2.$r1");
                    }
                    _ => assert!(false),
                }

                assert_eq!(divert.stack_push_type, PushPopType::Tunnel);
                assert_eq!(divert.pushes_to_stack, true);
                assert_eq!(divert.is_conditional, false);
            }
            _ => assert!(false),
        }
    }

    #[test]
    fn tunnel_conditional_test() {
        let json = "{\"->t->\": \".^.s\", \"c\": true}";
        let runtime_object: RuntimeObject = serde_json::from_str(json).unwrap();
        match runtime_object {
            RuntimeObject::Divert(divert) => {
                assert_eq!(divert.is_conditional, true);
            }
            _ => assert!(false),
        }
    }

    #[test]
    fn external_function_test() {
        let json = "{\"x()\": \"0.g-0.3.$r1\"}";
        let runtime_object: RuntimeObject = serde_json::from_str(json).unwrap();
        match runtime_object {
            RuntimeObject::Divert(divert) => {
                match divert.target {
                    TargetType::ExternalName(string, external_args) => {
                        assert_eq!(string, "0.g-0.3.$r1");
                    }
                    _ => assert!(false),
                }

                assert_eq!(divert.stack_push_type, PushPopType::Function);
                assert_eq!(divert.pushes_to_stack, false);
                assert_eq!(divert.is_conditional, false);
            }
            _ => assert!(false),
        }
    }

    #[test]
    fn external_function_with_args_test() {
        let json = "{\"x()\": \"0.g-0.3.$r1\", \"exArgs\": 5}";
        let runtime_object: RuntimeObject = serde_json::from_str(json).unwrap();
        match runtime_object {
            RuntimeObject::Divert(Divert {
                target: TargetType::ExternalName(_, external_args),
                ..
            }) => {
                assert_eq!(external_args, 5);
            }
            _ => assert!(false),
        }
    }

    #[test]
    fn external_function_with_conditional_test() {
        let json = "{\"x()\": \"0.g-0.3.$r1\", \"exArgs\": 5, \"c\": true}";
        let runtime_object: RuntimeObject = serde_json::from_str(json).unwrap();
        match runtime_object {
            RuntimeObject::Divert(divert) => {
                assert_eq!(divert.is_conditional, true);
            }
            _ => assert!(false),
        }
    }

    #[test]
    fn choice_test() {
        let json = "{\"*\":\".^.c\",\"flg\":18}";
        let runtime_object: RuntimeObject = serde_json::from_str(json).unwrap();
        match runtime_object {
            RuntimeObject::Choice(choice) => {
                assert_eq!(choice.choice_target_path.to_string(), ".^.c");
                assert_eq!(choice.has_condition, false);
                assert_eq!(choice.has_start_content, true);
                assert_eq!(choice.has_choice_only_content, false);
                assert_eq!(choice.is_invisible_default, false);
                assert_eq!(choice.once_only, true);
            }
            _ => assert!(false),
        }
    }

    #[test]
    fn variable_reference_test() {
        let json = "{\"VAR?\": \"danger\"}";
        let runtime_object: RuntimeObject = serde_json::from_str(json).unwrap();
        match runtime_object {
            RuntimeObject::VariableReference(variable) => {
                assert_eq!(variable.name, "danger");
            }
            _ => assert!(false),
        }
    }

    #[test]
    fn read_count_test() {
        let json = "{\"CNT?\": \"the_hall.light_switch\"}";
        let runtime_object: RuntimeObject = serde_json::from_str(json).unwrap();
        match runtime_object {
            RuntimeObject::ReadCount(variable) => {
                assert_eq!(variable.target.to_string(), "the_hall.light_switch");
            }
            _ => assert!(false),
        }
    }

    #[test]
    fn variable_assignment_test() {
        let json = "{\"VAR=\": \"money\"}";
        let runtime_object: RuntimeObject = serde_json::from_str(json).unwrap();
        match runtime_object {
            RuntimeObject::VariableAssignment(variable) => {
                assert_eq!(variable.name, "money");
                assert_eq!(variable.is_new_declaration, true);
                assert_eq!(variable.is_global, true);
            }
            _ => assert!(false),
        }
    }

    #[test]
    fn variable_assignment_redeclared_test() {
        let json = "{\"VAR=\": \"money\", \"re\": true}";
        let runtime_object: RuntimeObject = serde_json::from_str(json).unwrap();
        match runtime_object {
            RuntimeObject::VariableAssignment(variable) => {
                assert_eq!(variable.name, "money");
                assert_eq!(variable.is_new_declaration, false);
                assert_eq!(variable.is_global, true);
            }
            _ => assert!(false),
        }
    }

    #[test]
    fn temporary_variable_assignment_test() {
        let json = "{\"temp=\": \"x\"}";
        let runtime_object: RuntimeObject = serde_json::from_str(json).unwrap();
        match runtime_object {
            RuntimeObject::VariableAssignment(variable) => {
                assert_eq!(variable.name, "x");
                assert_eq!(variable.is_new_declaration, true);
                assert_eq!(variable.is_global, false);
            }
            _ => assert!(false),
        }
    }

    #[test]
    fn tag_test() {
        let json = "{\"#\": \"This is a tag\"}";
        let runtime_object: RuntimeObject = serde_json::from_str(json).unwrap();
        match runtime_object {
            RuntimeObject::Tag(tag) => {
                assert_eq!(tag.text, "This is a tag");
            }
            _ => assert!(false),
        }
    }

    #[test]
    fn container_test() {
        let json = r#"["^'Ah",{"->":"$r","var":true}, null]"#;
        let runtime_object: RuntimeObject = serde_json::from_str(json).unwrap();
        match runtime_object {
            RuntimeObject::Container(container) => {
                let container = &container.content;
                assert_eq!(container.len(), 2);

                match container.get(0).unwrap() {
                    &RuntimeObject::Value(ref value) => match value {
                        &Value::String(ref str) => assert_eq!(str, "'Ah"),
                        _ => assert!(false),
                    },
                    _ => assert!(false),
                }

                match container.get(1).unwrap() {
                    &RuntimeObject::Divert(ref divert) => {
                        match divert.target {
                            TargetType::VarName(ref target_name) => {
                                assert_eq!(target_name, "$r");
                            }
                            _ => assert!(false),
                        }

                        assert_eq!(divert.stack_push_type, PushPopType::None);
                        assert_eq!(divert.pushes_to_stack, false);
                        assert_eq!(divert.is_conditional, false);
                    }
                    _ => assert!(false),
                }
            }
            _ => assert!(false),
        }
    }

    #[test]
    fn nested_container_test() {
        let json = r###"["^test",{"subContainer":[5,6,null],"#f":3,"#n":"container"}]"###;
        let runtime_object: RuntimeObject = serde_json::from_str(json).unwrap();
        match runtime_object {
            RuntimeObject::Container(container) => {
                let content = &container.content;
                assert_eq!(content.len(), 1);
                assert_eq!(container.name.as_ref().unwrap(), "container");

                match content.get(0).unwrap() {
                    &RuntimeObject::Value(ref value) => match value {
                        &Value::String(ref str) => assert_eq!(str, "test"),
                        _ => assert!(false),
                    },
                    _ => assert!(false),
                }

                let sub_object = &container.named_subelements["subContainer"];

                let RuntimeObject::Container(sub_container) = sub_object else {
                    panic!()
                };

                let sub_content = &sub_container.content;
                assert_eq!(sub_content.len(), 2);
                // this assert fails until naming of nested containers is supported
                assert_eq!(sub_container.name.as_ref().unwrap(), "subContainer");

                match sub_content.get(0).unwrap() {
                    RuntimeObject::Value(value) => match value {
                        Value::Int(int_value) => assert_eq!(*int_value, 5),
                        _ => assert!(false),
                    },
                    _ => assert!(false),
                }

                match sub_content.get(1).unwrap() {
                    &RuntimeObject::Value(ref value) => match value {
                        &Value::Int(int_value) => assert_eq!(int_value, 6),
                        _ => assert!(false),
                    },
                    _ => assert!(false),
                }
            }
            _ => assert!(false),
        }
    }

    #[test]
    fn runtime_graph_test() {
        let json = r###"{"inkVersion":17,"root":[[["^I looked at Monsieur Fogg","\n",["ev",{"^->":"0.g-0.2.$r1"},{"temp=":"$r"},"str",{"->":".^.s"},[{"#n":"$r1"}],"/str","/ev",{"*":".^.c","flg":18},{"s":["^... and I could contain myself no longer.",{"->":"$r","var":true},null],"c":["ev",{"^->":"0.g-0.2.c.$r2"},"/ev",{"temp=":"$r"},{"->":".^.^.s"},[{"#n":"$r2"}],"\n","\n","^'What is the purpose of our journey, Monsieur?'","\n","^'A wager,' he replied.","\n",[["ev",{"^->":"0.g-0.2.c.12.0.$r1"},{"temp=":"$r"},"str",{"->":".^.s"},[{"#n":"$r1"}],"/str","/ev",{"*":".^.c","flg":18},{"s":["^'A wager!'",{"->":"$r","var":true},null],"c":["ev",{"^->":"0.g-0.2.c.12.0.c.$r2"},"/ev",{"temp=":"$r"},{"->":".^.^.s"},[{"#n":"$r2"}],"^ I returned.","\n","\n","^He nodded.","\n",[["ev",{"^->":"0.g-0.2.c.12.0.c.11.0.$r1"},{"temp=":"$r"},"str",{"->":".^.s"},[{"#n":"$r1"}],"/str","/ev",{"*":".^.c","flg":18},{"s":["^'But surely that is foolishness!'",{"->":"$r","var":true},null],"c":["ev",{"^->":"0.g-0.2.c.12.0.c.11.0.c.$r2"},"/ev",{"temp=":"$r"},{"->":".^.^.s"},[{"#n":"$r2"}],"\n","\n",{"->":".^.^.^.g-0"},{"#f":5}]}],["ev",{"^->":"0.g-0.2.c.12.0.c.11.1.$r1"},{"temp=":"$r"},"str",{"->":".^.s"},[{"#n":"$r1"}],"/str","/ev",{"*":".^.c","flg":18},{"s":["^'A most serious matter then!'",{"->":"$r","var":true},null],"c":["ev",{"^->":"0.g-0.2.c.12.0.c.11.1.c.$r2"},"/ev",{"temp=":"$r"},{"->":".^.^.s"},[{"#n":"$r2"}],"\n","\n",{"->":".^.^.^.g-0"},{"#f":5}]}],{"g-0":["^He nodded again.","\n",["ev",{"^->":"0.g-0.2.c.12.0.c.11.g-0.2.$r1"},{"temp=":"$r"},"str",{"->":".^.s"},[{"#n":"$r1"}],"/str","/ev",{"*":".^.c","flg":18},{"s":["^'But can we win?'",{"->":"$r","var":true},null],"c":["ev",{"^->":"0.g-0.2.c.12.0.c.11.g-0.2.c.$r2"},"/ev",{"temp=":"$r"},{"->":".^.^.s"},[{"#n":"$r2"}],"\n","\n","^'That is what we will endeavour to find out,' he answered.","\n",{"->":"0.g-0.2.c.12.g-0"},{"#f":5}]}],["ev",{"^->":"0.g-0.2.c.12.0.c.11.g-0.3.$r1"},{"temp=":"$r"},"str",{"->":".^.s"},[{"#n":"$r1"}],"/str","/ev",{"*":".^.c","flg":18},{"s":["^'A modest wager, I trust?'",{"->":"$r","var":true},null],"c":["ev",{"^->":"0.g-0.2.c.12.0.c.11.g-0.3.c.$r2"},"/ev",{"temp=":"$r"},{"->":".^.^.s"},[{"#n":"$r2"}],"\n","\n","^'Twenty thousand pounds,' he replied, quite flatly.","\n",{"->":"0.g-0.2.c.12.g-0"},{"#f":5}]}],["ev",{"^->":"0.g-0.2.c.12.0.c.11.g-0.4.$r1"},{"temp=":"$r"},"str",{"->":".^.s"},[{"#n":"$r1"}],"/str","str","^.","/str","/ev",{"*":".^.c","flg":22},{"s":["^I asked nothing further of him then",{"->":"$r","var":true},null],"c":["ev",{"^->":"0.g-0.2.c.12.0.c.11.g-0.4.c.$r2"},"/ev",{"temp=":"$r"},{"->":".^.^.s"},[{"#n":"$r2"}],"^, and after a final, polite cough, he offered nothing more to me. ","<>","\n","\n",{"->":"0.g-0.2.c.12.g-0"},{"#f":5}]}],null]}],{"#f":5}]}],["ev",{"^->":"0.g-0.2.c.12.1.$r1"},{"temp=":"$r"},"str",{"->":".^.s"},[{"#n":"$r1"}],"/str","str","^.'","/str","/ev",{"*":".^.c","flg":22},{"s":["^'Ah",{"->":"$r","var":true},null],"c":["ev",{"^->":"0.g-0.2.c.12.1.c.$r2"},"/ev",{"temp=":"$r"},{"->":".^.^.s"},[{"#n":"$r2"}],"^,' I replied, uncertain what I thought.","\n","\n",{"->":".^.^.^.g-0"},{"#f":5}]}],{"g-0":["^After that, ","<>","\n",{"->":"0.g-1"},null]}],{"#f":5}]}],["ev",{"^->":"0.g-0.3.$r1"},{"temp=":"$r"},"str",{"->":".^.s"},[{"#n":"$r1"}],"/str","/ev",{"*":".^.c","flg":18},{"s":["^... but I said nothing",{"->":"$r","var":true},null],"c":["ev",{"^->":"0.g-0.3.c.$r2"},"/ev",{"temp=":"$r"},{"->":".^.^.s"},[{"#n":"$r2"}],"^ and ","<>","\n","\n",{"->":"0.g-1"},{"#f":5}]}],{"#n":"g-0"}],{"g-1":["^we passed the day in silence.","\n",["end",{"#n":"g-2"}],null]}],"done",{"#f":3}],"listDefs":{}}"###;
        let runtime_graph: RuntimeGraph = serde_json::from_str(json).unwrap();
        assert_eq!(runtime_graph.ink_version, 17)
    }

    #[test]
    fn mini_runtime_graph_test() {
        let json = r###"{"inkVersion":21,"root":[["end",["done",{"#f":5,"#n":"g-0"}],null],"done",{"#f":1}],"listDefs":{}}"###;
        let runtime_graph: RuntimeGraph = serde_json::from_str(json).unwrap();
        assert_eq!(runtime_graph.ink_version, 21)
    }

    #[test]
    fn container_length_test() {
        let json = r###"[["end",["done",{"#f":5,"#n":"g-0"}],null],"done",{"#f":1}]"###;
        let container: Container = serde_json::from_str(json).unwrap();
        assert_eq!(container.content.len(), 2);
    }

    #[test]
    fn null_container_test() {
        let json = r###"[null]"###;
        let container: Container = serde_json::from_str(json).unwrap();
        assert_eq!(container.content.len(), 0);
    }

    // FIXME: For now serde MapAccess::next_value() for &str fail when deserializing from a reader
    // FIXME: https://github.com/serde-rs/serde/issues/1009
    /*#[test]
    fn ink_test_from_reader() {
        use std::io::BufReader;
        use std::fs::File;

        let json = r###"{"inkVersion":17,"root":[[["^I looked at Monsieur Fogg","\n",["ev",{"^->":"0.g-0.2.$r1"},{"temp=":"$r"},"str",{"->":".^.s"},[{"#n":"$r1"}],"/str","/ev",{"*":".^.c","flg":18},{"s":["^... and I could contain myself no longer.",{"->":"$r","var":true},null],"c":["ev",{"^->":"0.g-0.2.c.$r2"},"/ev",{"temp=":"$r"},{"->":".^.^.s"},[{"#n":"$r2"}],"\n","\n","^'What is the purpose of our journey, Monsieur?'","\n","^'A wager,' he replied.","\n",[["ev",{"^->":"0.g-0.2.c.12.0.$r1"},{"temp=":"$r"},"str",{"->":".^.s"},[{"#n":"$r1"}],"/str","/ev",{"*":".^.c","flg":18},{"s":["^'A wager!'",{"->":"$r","var":true},null],"c":["ev",{"^->":"0.g-0.2.c.12.0.c.$r2"},"/ev",{"temp=":"$r"},{"->":".^.^.s"},[{"#n":"$r2"}],"^ I returned.","\n","\n","^He nodded.","\n",[["ev",{"^->":"0.g-0.2.c.12.0.c.11.0.$r1"},{"temp=":"$r"},"str",{"->":".^.s"},[{"#n":"$r1"}],"/str","/ev",{"*":".^.c","flg":18},{"s":["^'But surely that is foolishness!'",{"->":"$r","var":true},null],"c":["ev",{"^->":"0.g-0.2.c.12.0.c.11.0.c.$r2"},"/ev",{"temp=":"$r"},{"->":".^.^.s"},[{"#n":"$r2"}],"\n","\n",{"->":".^.^.^.g-0"},{"#f":5}]}],["ev",{"^->":"0.g-0.2.c.12.0.c.11.1.$r1"},{"temp=":"$r"},"str",{"->":".^.s"},[{"#n":"$r1"}],"/str","/ev",{"*":".^.c","flg":18},{"s":["^'A most serious matter then!'",{"->":"$r","var":true},null],"c":["ev",{"^->":"0.g-0.2.c.12.0.c.11.1.c.$r2"},"/ev",{"temp=":"$r"},{"->":".^.^.s"},[{"#n":"$r2"}],"\n","\n",{"->":".^.^.^.g-0"},{"#f":5}]}],{"g-0":["^He nodded again.","\n",["ev",{"^->":"0.g-0.2.c.12.0.c.11.g-0.2.$r1"},{"temp=":"$r"},"str",{"->":".^.s"},[{"#n":"$r1"}],"/str","/ev",{"*":".^.c","flg":18},{"s":["^'But can we win?'",{"->":"$r","var":true},null],"c":["ev",{"^->":"0.g-0.2.c.12.0.c.11.g-0.2.c.$r2"},"/ev",{"temp=":"$r"},{"->":".^.^.s"},[{"#n":"$r2"}],"\n","\n","^'That is what we will endeavour to find out,' he answered.","\n",{"->":"0.g-0.2.c.12.g-0"},{"#f":5}]}],["ev",{"^->":"0.g-0.2.c.12.0.c.11.g-0.3.$r1"},{"temp=":"$r"},"str",{"->":".^.s"},[{"#n":"$r1"}],"/str","/ev",{"*":".^.c","flg":18},{"s":["^'A modest wager, I trust?'",{"->":"$r","var":true},null],"c":["ev",{"^->":"0.g-0.2.c.12.0.c.11.g-0.3.c.$r2"},"/ev",{"temp=":"$r"},{"->":".^.^.s"},[{"#n":"$r2"}],"\n","\n","^'Twenty thousand pounds,' he replied, quite flatly.","\n",{"->":"0.g-0.2.c.12.g-0"},{"#f":5}]}],["ev",{"^->":"0.g-0.2.c.12.0.c.11.g-0.4.$r1"},{"temp=":"$r"},"str",{"->":".^.s"},[{"#n":"$r1"}],"/str","str","^.","/str","/ev",{"*":".^.c","flg":22},{"s":["^I asked nothing further of him then",{"->":"$r","var":true},null],"c":["ev",{"^->":"0.g-0.2.c.12.0.c.11.g-0.4.c.$r2"},"/ev",{"temp=":"$r"},{"->":".^.^.s"},[{"#n":"$r2"}],"^, and after a final, polite cough, he offered nothing more to me. ","<>","\n","\n",{"->":"0.g-0.2.c.12.g-0"},{"#f":5}]}],null]}],{"#f":5}]}],["ev",{"^->":"0.g-0.2.c.12.1.$r1"},{"temp=":"$r"},"str",{"->":".^.s"},[{"#n":"$r1"}],"/str","str","^.'","/str","/ev",{"*":".^.c","flg":22},{"s":["^'Ah",{"->":"$r","var":true},null],"c":["ev",{"^->":"0.g-0.2.c.12.1.c.$r2"},"/ev",{"temp=":"$r"},{"->":".^.^.s"},[{"#n":"$r2"}],"^,' I replied, uncertain what I thought.","\n","\n",{"->":".^.^.^.g-0"},{"#f":5}]}],{"g-0":["^After that, ","<>","\n",{"->":"0.g-1"},null]}],{"#f":5}]}],["ev",{"^->":"0.g-0.3.$r1"},{"temp=":"$r"},"str",{"->":".^.s"},[{"#n":"$r1"}],"/str","/ev",{"*":".^.c","flg":18},{"s":["^... but I said nothing",{"->":"$r","var":true},null],"c":["ev",{"^->":"0.g-0.3.c.$r2"},"/ev",{"temp=":"$r"},{"->":".^.^.s"},[{"#n":"$r2"}],"^ and ","<>","\n","\n",{"->":"0.g-1"},{"#f":5}]}],{"#n":"g-0"}],{"g-1":["^we passed the day in silence.","\n",["end",{"#n":"g-2"}],null]}],"done",{"#f":3}],"listDefs":{}}"###;
        //let reader = BufReader::new(json.as_bytes());
        let reader = File::open("/home/midgard/dev/rink-runtime/tests/simple4.ink.json").unwrap();
        let inkObject = InkJSon::from_reader(reader).unwrap();
        assert_eq!(inkObject.ink_version, 17)
    }*/
}
