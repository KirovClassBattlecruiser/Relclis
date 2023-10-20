use std::collections::HashMap;
use std::hash::Hash;
use std::io;
use std::ops::{Add, AddAssign};
use std::process::exit;

struct CLI {
    commandMap: HashMap<String, fn(Vec<String>)>,
}

impl Add for CLI {
    type Output = CLI;

    fn add(self, other: CLI) -> CLI {
        let mut merged_cli = CLI::default();

        for (key, value) in self.commandMap.iter() {
            merged_cli.addCommand(key.clone(), value.clone());
        }

        for (key, value) in other.commandMap.iter() {
            merged_cli.addCommand(key.clone(), value.clone());
        }

        merged_cli
    }
}

impl AddAssign for CLI {
    fn add_assign(&mut self, other: CLI) {
        for (key, value) in other.commandMap {
            self.addCommand(key, value);
        }
    }
}

impl Default for CLI {
    fn default() -> Self {
        CLI {
            commandMap: HashMap::new(),
        }
    }
}

impl CLI {
    pub fn merge(&mut self, other: CLI, bis_functions: bool) -> &mut Self {
        for (key, value) in other.commandMap {
            // Check if we already have that command
            if (self.commandMap.get(&*key).is_some()) {
                if bis_functions {
                    self.addCommand(format!("{key}_bis"), value);
                }
            } else {
                self.addCommand(key, value);
            }
        }
        return self;
    }

    pub fn addCommand(&mut self, commandName: String, commandAction: fn(Vec<String>)) -> &mut Self {
        self.commandMap.insert(commandName, commandAction);
        return self;
    }

    pub fn addCommands(
        &mut self,
        commandNames: Vec<String>,
        commandAction: fn(Vec<String>),
    ) -> &mut Self {
        for commandName in commandNames {
            self.commandMap.insert(commandName, commandAction);
        }
        return self;
    }

    pub fn hasCommand(&self, commandName: &str) -> bool {
        return self.commandMap.get(commandName).is_some();
    }

    pub fn executeCommand(&self, commandName: String, args: Vec<String>) -> Result<(), String> {
        if let Some(command_fn) = self.commandMap.get(&commandName) {
            command_fn(args);
            Ok(())
        } else {
            Err(format!("No such function \"{}\"", commandName))
        }
    }

    pub fn executeCommands(
        &self,
        commandNames: Vec<String>,
        args: Vec<String>,
    ) -> Result<(), String> {
        for commandName in commandNames {
            self.executeCommand(commandName, args.clone())?;
        }
        Ok(())
    }

    pub fn args_from_input(input: &String, delimiter: &char) -> Vec<String> {
        let mut result = Vec::new();
        let mut back: String = String::new();
        for substr in input.chars() {
            if (substr == *delimiter) {
                result.push(back.clone());
                back.clear();
            } else {
                back.insert(1, substr);
            }
        }

        result.remove(0);
        return result;
    }

    pub fn question_from_input(input: &String, delimiter: &char) -> String {
        let mut result = Vec::new();
        let mut back: String = String::new();
        for substr in input.chars() {
            if (substr == *delimiter) {
                result.push(back.clone());
                back.clear();
            } else {
                back.insert(1, substr);
            }
        }

        //TODO: Less lazy function
        return result[0].clone();
    }

    pub fn run(&self, inputLoopQuestion: &str) {
        let mut input = String::new();

        loop {
            println!("{}", inputLoopQuestion);

            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");

            input.trim();

            let args = CLI::args_from_input(&input, &' ');
            let command = CLI::question_from_input(&input, &' ');

            let result = self.executeCommand(command, args);
            match result {
                Ok(()) => {}
                Err(String) => {
                    println!("Errror") /* TODO: Print the error*/
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_command1(args: Vec<String>) {
        println!("Test command 1 up and running. Arguments: ");
        for arg in args {
            println!("{arg}");
        }
    }

    fn test_command2(args: Vec<String>) {
        println!("Test command 2 up and running. Arguments: ");
        for arg in args {
            println!("{arg}");
        }
    }

    #[test]
    fn command_testing() {
        let mut cli = CLI::default();
        cli.addCommand("test1".to_string(), test_command1)
            .addCommands(
                vec!["test2".to_string(), "testtwo".to_string()],
                test_command2,
            );

        let args: Vec<String> = vec!["--test".to_string(), "IsTest=true".to_string()];
        assert_eq!(
            cli.executeCommand("test1".to_string(), args.clone()),
            Ok(())
        );
        assert_eq!(
            cli.executeCommand("unreal".to_string(), args.clone()),
            Err("No such function \"unreal\"".to_string())
        );
        assert_eq!(
            cli.executeCommand("test2".to_string(), args.clone()),
            Ok(())
        );
        assert_eq!(
            cli.executeCommand("testtwo".to_string(), args.clone()),
            Ok(())
        );

        let validCommands: Vec<String> = vec!["test1".to_string(), "test2".to_string()];
        let invalidCommands: Vec<String> = vec!["test500".to_string(), "joe biden".to_string()];
        assert_eq!(cli.executeCommands(validCommands, args.clone()), Ok(()));
        assert!(cli.executeCommands(invalidCommands, args.clone()).is_err());
    }

    #[test]
    fn merge_testing() {
        let mut cli1 = CLI::default();
        cli1.addCommand("test1".to_string(), test_command1)
            .addCommand("test2".to_string(), test_command2);
        let mut cli2 = CLI::default();
        cli2.addCommand("test2".to_string(), test_command2);

        cli1.merge(cli2, true);

        assert!(cli1.hasCommand("test2") == true);
        assert!(cli1.hasCommand("test2_bis") == true);

        cli2.run("Enter command: ");
    }

    #[test]
    fn operator_testing() {
        let mut cli1 = CLI::default();
        cli1.addCommand("test1".to_string(), test_command1)
            .addCommand("test2".to_string(), test_command2);
        let mut cli2 = CLI::default();
        cli2.addCommand("test2".to_string(), test_command2);

        cli1 += cli2;

        assert!(cli1.hasCommand("test2") == true);
        assert!(cli1.hasCommand("test2") == true);
        assert!(cli1.hasCommand("test2_bis") == false);
    }
}
