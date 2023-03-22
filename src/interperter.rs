use serde_json::{Value, value};

use crate::{statement::{Statement}, expression::Expression};

pub struct Interperter {
    pub context: Value,
    pub result: String
}

impl Interperter {
    pub fn interpret(&mut self, statements: Vec<Statement>) {
        for statement in statements {
            println!("{:?}", statement);
            self.result.push_str(&Interperter::to_string(self.execute(statement)));
        }
    }

    fn to_string(value: Value) -> String{
        if value.is_string() {
            value.as_str().unwrap().to_owned()
        } else if value.is_number() {
            value.as_f64().unwrap().to_string()
        } else if value.is_null() {
            "null".to_string()
        } else {
            todo!("{:?}", value);
        }
    }

    fn execute(&self, statement: Statement) -> Value {
        match statement {
            Statement::Expression(expression) => {
                match expression {
                    Expression::Variable(variable_expression) => {
                        return self.context[variable_expression.name].clone();
                    }
                    Expression::TemplateLiteral(template_literal_expression) => {
                        return serde_json::Value::String(template_literal_expression.value);
                    }
                    Expression::Call(call_expression) => {
                        let value = self.execute(Statement::Expression(*call_expression.callee));
                        if !value.is_object() {
                            panic!("{:?} is undefined", call_expression.name);
                        }
                        return value[call_expression.name].clone();
                    }
                }
            }
            Statement::For(for_statement) => {
                let array = self.execute(Statement::Expression(for_statement.array_variable));
                if !array.is_array() {
                    panic!("Not array");
                }
                for i in array.as_array() {
                    println!("{:?}", i);
                }
            }
        }
        Value::Null
    }
}

#[cfg(test)]
mod tests {
    use crate::{parser::Parser, interperter, tokenizer::{self, Tokenizer}};

    use super::*;

    #[test]
    fn it_works() {
        let mut parser = Parser::new();
        let mut tokenizer = Tokenizer::new(r#"
Yoo {{ for i in items }} yes {{ end }}
"#.as_bytes());
        let statements = parser.parse(&mut tokenizer);
        let value: Value = serde_json::from_str(r#"{"items": ["1", "2", "3"]}"#).unwrap();
        //println!("{}", value["name"]);
        let mut interperter = Interperter {context: value, result: String::new()};
        interperter.interpret(statements);
        println!("{}", interperter.result);
        assert!(false);
    }
}