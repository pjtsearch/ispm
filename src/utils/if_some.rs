pub fn if_some<T,C>(option:Option<T>,cb:C) where C: FnOnce(T){
    if option.is_some() {
        cb(option.unwrap())
    }
}