pub fn required<T>(name:&str,target:std::option::Option<T>) -> T {
    target.expect(&format!("{} is required",name))
}