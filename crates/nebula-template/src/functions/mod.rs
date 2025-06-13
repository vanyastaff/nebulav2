pub trait Function: Send + Sync {
    fn name(&self) -> &str;
    fn signature(&self) -> &FunctionSignature;
    fn execute(&self, args: Vec<Value>) -> Result<Value, FunctionError>;
}

#[derive(Debug, Clone)]
pub struct FunctionSignature {
    pub input_type: ValueType,
    pub parameters: Vec<Parameter>,
    pub return_type: ValueType,
}

#[derive(Debug, Clone)]
pub struct Parameter {
    pub name: String,
    pub value_type: ValueType,
    pub required: bool,
    pub default: Option<Value>,
}

pub struct FunctionRegistry {
    functions: HashMap<String, Box<dyn Function>>,
}

impl FunctionRegistry {
    pub fn with_builtins() -> Self;
    pub fn register<F: Function + 'static>(&mut self, func: F);
    pub fn get(&self, name: &str) -> Option<&dyn Function>;
}