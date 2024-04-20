use my_tools::MyToolsAddon;
use my_tools::MyToolsAddonCommand;
use my_tools::MyToolsError;
use my_tools::CommandStringResult;

/// Command to print "Hello, world!"
struct HelloWorldCommand {}

impl MyToolsAddonCommand for HelloWorldCommand {
    fn execute(&self) -> Result<CommandStringResult, MyToolsError> {
        Ok("Hello, world!".to_string())
    }
}

/// Command to print "Hello, {name}!"
struct HelloInputCommand {
    name: String,
}

impl MyToolsAddonCommand for HelloInputCommand {
    fn execute(&self) -> Result<CommandStringResult, MyToolsError> {
        Ok(format!("Hello, {}!", self.name))
    }
}


/// HelloWorldAddon structure
pub struct HelloWorldAddon; 

impl MyToolsAddon for HelloWorldAddon {
    fn get_keyword() -> &'static str {
        "test"
    }

    fn parse(args: &[String]) -> Result<Box<dyn MyToolsAddonCommand>, MyToolsError> {
        // Convert &[String] to &[&str]
        let args: Vec<&str> = args.iter().map(|s| s.as_str()).collect();

        // Parse the arguments and return the corresponding command
        match args[..] {
            [] => Ok(Box::new(HelloWorldCommand {})),
            [name] => Ok(Box::new(HelloInputCommand { name: name.to_string() })),
            _ => Err(MyToolsError::InvalidCommand(format!("{}", args.join(" "))))
        }
    }    

    fn list_commands() -> Vec<String> {
        vec![
            ".. hello : Print \"Hello, world!\"".to_string(),
            ".. hello {name} : \"Hello, {name}!\"".to_string(),
        ]
    }
}

#[test]
fn get_keyword() {
    let keywork = HelloWorldAddon::get_keyword();
    assert!(keywork == "test");
}

// HelloWorldAddon::parse tests
// "hello" -> HelloWorldCommand - Ok
#[test]
fn command_hello() {
    let args = vec![];
    let cmd = HelloWorldAddon::parse(&args).expect("Failed to parse command");
    assert_eq!(cmd.execute().unwrap(), String::from("Hello, world!"));
}

// "hello world123" -> HelloInputCommand -> Ok
#[test]
fn command_hello_input() {
    let args = vec!["world123".to_string()];
    let cmd = HelloWorldAddon::parse(&args).expect("Failed to parse command");
    assert_eq!(cmd.execute().unwrap(), String::from("Hello, world123!"));
}

// "hello world test" -> InvalidCommand -> Error
#[test]
fn parse_over_args() {
    let args = vec!["world".to_string(), "test".to_string()];
    let cmd = HelloWorldAddon::parse(&args);
    assert!(cmd.is_err());
}

