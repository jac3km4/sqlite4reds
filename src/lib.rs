use red4ext_rs::interop::*;
use red4ext_rs::prelude::*;
use sqlite::{Connection, Value};
use static_init::dynamic;

define_plugin! {
    name: "sqlite4reds",
    author: "jekky",
    version: 0:1:0,
    on_register: {
        register_function!("SQLite.AttachDb", attach_db);
        register_function!("SQLite.Query", run_query);
        register_function!("SQLite.Execute", run_stmt);
    }
}

#[dynamic]
static mut CONNECTION: Connection = sqlite::open("").unwrap();

fn attach_db(name: String) {
    CONNECTION
        .read()
        .execute(format!("ATTACH DATABASE {} as {}", name, name))
        .ok();
}

fn run_query(query: String, args: REDArray<Variant>) -> Ref<ffi::IScriptable> {
    match do_query(&query, args.as_slice()) {
        Ok(res) => {
            call!("SQLite.Success::New;array<Row>" (res) -> Ref<ffi::IScriptable>)
        }
        Err(err) => {
            let code = err.code.unwrap_or(0);
            let msg = err.message.unwrap_or_default();
            call!("SQLite.Error::New;Int64String" (code as i64, msg) -> Ref<ffi::IScriptable>)
        }
    }
}

fn run_stmt(stmt: String) -> Ref<ffi::IScriptable> {
    match CONNECTION.read().execute(stmt) {
        Ok(()) => {
            call!("SQLite.Success::NewEmpty;" () -> Ref<ffi::IScriptable>)
        }
        Err(err) => {
            let code = err.code.unwrap_or(0);
            let msg = err.message.unwrap_or_default();
            call!("SQLite.Error::New;Int64String" (code as i64, msg) -> Ref<ffi::IScriptable>)
        }
    }
}

fn do_query(query: &str, args: &[Variant]) -> Result<REDArray<Row>, sqlite::Error> {
    let con = CONNECTION.read();
    let mut cur = con.prepare(query)?.into_cursor();
    let args = args.iter().map(decode_value).collect::<Vec<_>>();
    cur.bind(&args)?;

    let mut rows = vec![];
    while let Some(row) = cur.next()? {
        let cols = REDArray::from_sized_iter(row.iter().map(encode_value));
        rows.push(Row { cols })
    }

    Ok(REDArray::from_sized_iter(rows.into_iter()))
}

fn decode_value(variant: &Variant) -> Value {
    if let Some(val) = variant.try_get::<i32>() {
        Value::Integer(val.into())
    } else if let Some(val) = variant.try_get::<i64>() {
        Value::Integer(val)
    } else if let Some(val) = variant.try_get::<u32>() {
        Value::Integer(val.into())
    } else if let Some(val) = variant.try_get::<f32>() {
        Value::Float(val.into())
    } else if let Some(val) = variant.try_get::<f64>() {
        Value::Float(val)
    } else if let Some(val) = variant.try_get::<String>() {
        Value::String(val)
    } else {
        unreachable!()
    }
}

fn encode_value(value: &Value) -> Variant {
    match value {
        Value::Integer(int) => Variant::new(*int),
        Value::Float(num) => Variant::new(*num),
        Value::String(str) => Variant::new(str.as_str()),
        _ => unreachable!(),
    }
}

#[derive(Debug, Default)]
#[repr(C)]
struct Row {
    cols: REDArray<Variant>,
}

impl IsoRED for Row {
    const NAME: &'static str = "SQLite.Row";
}
