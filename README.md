# sqlite4reds
Simple sqlite binding for redscript

## usage
- import the library
  ```haskell
  import SQLite.*
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
    )
  ");
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
        // nullables can be checked using IsDefined
        let has_address = IsDefined(row.columns[3]);
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

## SQLite types

<table>
  <tr>
    <th>SQLite type</th>
    <th>Redscript input types (in queries)</th>
    <th>Redscript output type (in result columns)</th>
  </tr>
  <tr>
    <td>INTEGER</td>
    <td>Int32, Int64, Uint32</td>
    <td>Int64</td>
  </tr>
  <tr>
    <td>REAL</td>
    <td>Float, Double</td>
    <td>Double</td>
  </tr>
  <tr>
    <td>TEXT</td>
    <td>String</td>
    <td>String</td>
  </tr>
  <tr>
    <td>BLOB</td>
    <td>Not supported</td>
    <td>Not supported</td>
  </tr>
</table>

## requirements
- [RED4ext](https://github.com/jac3km4/redscript)
- [redscript](https://github.com/WopsS/RED4ext.SDK)

## build
```
cargo build --release
```
