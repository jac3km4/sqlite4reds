module SQLite

public native func AttachDb(name: String);

public native func Query(query: String, args: array<Variant>) -> ref<Result>;

public native func Execute(query: String) -> ref<Result>;

public abstract class Result {
    public func IsSuccess() -> Bool;
    
    public final func Dump() {
        if IsDefined(this as Success) {
            LogChannel(n"DEBUG", s"Success(\((this as Success).GetRows()))");
        }
        if IsDefined(this as Error) {
            LogChannel(n"DEBUG", s"Error(\((this as Error).GetMessage()))");
        }
    }
}

public final class Success extends Result {
    let rows: array<Row>;

    public func IsSuccess() -> Bool = true;
    public func GetRows() -> array<Row> = this.rows;
    
    static func New(rows: array<Row>) -> ref<Success> {
        let self = new Success();
        self.rows = rows;
        return self;
    }

    static func NewEmpty() -> ref<Success> {
        return new Success();
    }
}

public final class Error extends Result {
    let code: Int64;
    let message: String;

    public func IsSuccess() -> Bool = false;
    public func GetCode() -> Int64 = this.code;
    public func GetMessage() -> String = this.message;
    
    static func New(code: Int64, message: String) -> ref<Error> {
        let self = new Error();
        self.code = code;
        self.message = message;
        return self;
    }
}

public struct Row {
    public let columns: array<Variant>;
}

func IntCol(val: Int64) -> Variant = val;
func FloatCol(val: Double) -> Variant = val;
func StringCol(val: String) -> Variant = val;
