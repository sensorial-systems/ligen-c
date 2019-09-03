use ligen::ligen;
use ligen_core::{Object, Method, Type, Output, Inputs, Input, Files, File};

pub struct Generator {}

#[ligen]
impl Generator {
    pub fn new() -> Generator {
        Generator {}
    }
    pub fn generate(&self, object: &Object) -> Files {
        let path = format!("c/include/{object}.h", object = object.ty.identifier.name);
        let object = self.generate_object(&object);
        Files::from(vec![File::new(path, object)])
    }

    pub fn generate_ty(&self, ty : &Type) -> String {
        match ty.is_atomic {
            true => match ty.identifier.name.as_ref() {
                "u64" => "uint64_t",
                "u32" => "uint32_t",
                "u16" => "uint16_t",
                "u8" => "uint8_t",
                "i64" => "int64_t",
                "i32" => "int32_t",
                "i16" => "int16_t",
                "i8" => "int8_t",
                "f32" => "float",
                "f64" => "double",
                _ => "#ERROR_GENERATING_ATOMIC_TYPE"
            }.to_string(),
            false => String::from("void*")
        }
    }

    pub fn generate_output(&self, output : &Output) -> String {
        match &output.output {
            Option::Some(ty) => self.generate_ty(&ty),
            Option::None => String::from("void")
        }
    }

    pub fn generate_args(&self, inputs_: &Inputs) -> String {
        let mut args = String::new();
        for (i, input) in inputs_.inputs.iter().enumerate() {
            let input = &input.identifier.name;
            let comma = if i > 0 { ", " } else { "" };
            args.push_str(&format!("{comma}{input}", comma = comma, input = input));
        }
        args
    }

    pub fn generate_input(&self, input : &Input) -> String {
        let input_type = self.generate_ty(&input.ty);
        let input_name = &input.identifier.name;
        let mut input = format!(include_str!("input.h"), input_type = input_type, input_name = input_name);
        input.truncate(input.len() - 2);
        input
    }

    pub fn generate_inputs(&self, inputs_ : &Inputs) -> String {
        let mut inputs = String::new();
        for (i, input) in inputs_.inputs.iter().enumerate() {
            let input = self.generate_input(&input);
            let comma = if i > 0 { ", " } else { "" };
            inputs.push_str(&format!("{comma}{input}", comma = comma, input = input));
        }
        inputs
    }

    pub fn generate_method(&self, method : &Method) -> String {
        let return_type = self.generate_output(&method.output);
        let function_identifier = method.get_extern_name();
        let parameters = self.generate_inputs(&method.inputs);
        let parameters = if method.inputs.is_associated {
            parameters
        } else {
            if parameters.len() > 0 {
                format!("void* self, {}", parameters)
            } else {
                format!("void* self")
            }
        };
        format!(include_str!("method.h"), return_type = return_type, function_identifier = function_identifier, parameters = parameters)
    }

    pub fn generate_object(&self, object : &Object) -> String {
        let mut methods = String::new();
        for method in &object.methods {
            let method = self.generate_method(&method);
            methods.push_str(&format!("\n{}", method));
        };
        format!(include_str!("object.h"), object = object.ty.identifier.name, methods = methods)
    }
}
