use std::fs::File;

use neon::prelude::*;
use serde_derive::{Serialize, Deserialize};
use serde_valid::Validate;

#[derive(Serialize, Deserialize, Validate, Debug)]
struct TestYaml {
    name: String,
    #[validate(minimum = 1)]
    #[validate(maximum = 10)]
    num: i32,
}

impl TestYaml {
    fn to_object<'a>(&self, cx: &mut FunctionContext<'a>) -> JsResult<'a, JsObject> {
       let obj = cx.empty_object();

        let num = cx.number(self.num);
        obj.set(cx, "num", num)?;

        let name = cx.string(&self.name);
        obj.set(cx, "name", name)?;

        Ok(obj) 
    }
}

fn get_num_cpus(mut cx: FunctionContext) -> JsResult<JsNumber> {
    Ok(cx.number(num_cpus::get() as f64))
}

fn print_test_obj(mut cx: FunctionContext) -> JsResult<JsObject> {
    TestYaml {
        num: 32,
        name: "David".to_string(),
    }.to_object(&mut cx)
}

fn validate_obj(mut cx: FunctionContext) -> JsResult<JsObject> {
    let test_yml: TestYaml = serde_yaml::from_reader(File::open(&"test.yml".to_string()).unwrap()).unwrap();
    let file_errors_yml = test_yml.validate();

    println!("{:#?}", file_errors_yml);

    if !file_errors_yml.is_err() {
        println!("No errors.");
    //   println!("Writing result to file");
    //   serde_json::to_writer(File::create(&"test-result.json".to_string()).unwrap(), &test_json).unwrap();
    } else {
        println!("An error occured when validating: {}", file_errors_yml.unwrap_err());
    }

    test_yml.to_object(&mut cx)
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("get", get_num_cpus)?;
    cx.export_function("print_obj", print_test_obj)?;
    cx.export_function("validate_obj", validate_obj)?;

    Ok(())
}