pub fn if_some<T,C>(option:Option<T>,cb:C) where C: FnOnce(T){
    if let Some(option) = option {
        cb(option)
    }
}