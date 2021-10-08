#[macro_export]
macro_rules! accessor {
    ((get=$getter:ident) : $type:ty ) => {
        fn $getter(&self) -> $type;
    };
    ((set=$setter:ident) : $type:ty ) => {
        fn $setter(&mut self, value: $type);
    };
    ((get=$getter:ident, set=$setter:ident) : $type:ty ) => {
        accessor!((get = $getter): $type);
        accessor!((set = $setter): $type);
    };
}

#[macro_export]
macro_rules! accessor_impl {
    ((get=$getter:ident) $name:ident : $type:ty ) => {
        fn $getter(&self) -> $type {
            self.$name
        }
    };
    ((set=$setter:ident) $name:ident : $type:ty ) => {
        fn $setter(&mut self, value: $type) {
            self.$name = value;
        }
    };
    ((get=$getter:ident, set=$setter:ident) $name:ident : $type:ty ) => {
        accessor_impl!((get=$getter) $name:$type);
        accessor_impl!((set=$setter) $name:$type);
    };
}
