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
        let cols = row.iter().map(|val| match val {
            Value::Integer(int) => {
                call!("SQLite.IntCol;Int64" (*int) -> Variant)
            }
            Value::Float(num) => {
                call!("SQLite.FloatCol;Double" (*num) -> Variant)
            }
            Value::String(str) => {
                call!("SQLite.StringCol;String" (str.as_str()) -> Variant)
            }
            _ => unreachable!(),
        });

        rows.push(Row {
            cols: REDArray::from_sized_iter(cols),
        })
    }

    Ok(REDArray::from_sized_iter(rows.into_iter()))
}

fn decode_value(variant: &Variant) -> Value {
    match rtti::get_type_name(variant.get_type()) {
        prims::INT32 => Value::Integer(unsafe { *(variant.get_data() as *const i32) as i64 }),
        prims::INT64 => Value::Integer(unsafe { *(variant.get_data() as *const i64) }),
        prims::UINT32 => Value::Integer(unsafe { *(variant.get_data() as *const i64) }),
        prims::FLOAT => Value::Float(unsafe { *(variant.get_data() as *const f32) as f64 }),
        prims::DOUBLE => Value::Float(unsafe { *(variant.get_data() as *const f64) }),
        prims::STRING => {
            let str = unsafe { *(variant.get_data() as *const REDString) };
            Value::String(FromRED::from_repr(str))
        }
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

mod prims {
    use red4ext_rs::interop::CName;

    pub const INT32: CName = CName::new("Int32");
    pub const INT64: CName = CName::new("Int64");
    pub const UINT32: CName = CName::new("Uint32");
    pub const FLOAT: CName = CName::new("Float");
    pub const DOUBLE: CName = CName::new("Double");
    pub const STRING: CName = CName::new("String");
}
