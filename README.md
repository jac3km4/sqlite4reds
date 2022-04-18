# sqlite4reds
Simple sqlite binding for redscript

## usage
- import the library
  ```haskell
  import SQLite
  ```
- load your database
  ```swift
  AttachDb("mydb");
  ```
- create a table
  ```swift
  // you always need to refer to your db by name (mydb)
  let res = Execute("
    CREATE TABLE mydb.COMPANY(
      ID INT PRIMARY KEY     NOT NULL,
      NAME           TEXT    NOT NULL,
      AGE            INT     NOT NULL,
      ADDRESS        CHAR(50),
      SALARY         REAL
    )"
  );
  // Success(Array([]))
  ```
- insert a row
  ```swift
  let res = Execute("
    INSERT INTO mydb.COMPANY (ID,NAME,AGE,ADDRESS,SALARY)
    VALUES (1, 'Paul', 32, 'California', 20000.00)
  ");
  // Success(Array([]))
  ```
- query a table
  ```swift
  let res = Query("SELECT * FROM mydb.COMPANY where name = ?", ["Paul"]);
  if res.IsSuccess() {
    for row in (res as Success).GetRows() {
        let name = FromVariant<String>(row.columns[1]);
        let age = FromVariant<Int64>(row.columns[2]);
        ...
      }
  }
  ```
- handle errors
  ```swift
  // the Dump method logs the result which may be an error or a list of rows
  res.Dump();
  // each error has a message and an error code, and both can be inspected
  if !res.IsSuccess() {
    LogChannel(n"DEBUG", (res as Error).GetMessage());
  }
  ```

## requirements
- [RED4ext](https://github.com/jac3km4/redscript)
- [redscript](https://github.com/WopsS/RED4ext.SDK)

## build
```
cargo build --release
```
