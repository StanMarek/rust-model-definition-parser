export type WrappedValue<T> = { v?: T; a?: boolean; attributes?: unknown };

export type Asset = {
    textField: WrappedValue<string>;
    numberField: WrappedValue<number>;
    arrayField: ArrayType[];
};

export type ArrayType = {
    field1: WrappedValue<string>;
    childField: ChildType;
};

export type ChildType = {
    field1: WrappedValue<string>;
    field2: WrappedValue<number>;
};
