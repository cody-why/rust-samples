

pub trait ToStringVec {
    fn to_string_vec(&self) -> Vec<String>;
}

impl<T> ToStringVec for Vec<T> 
where 
    T: ToString
{
    fn to_string_vec(&self) -> Vec<String> {
        self.iter().map(|s| s.to_string()).collect()
    }
}


#[macro_export]
macro_rules! vec_string {
    ($($x:expr),*) => {{
        vec![$($x.to_string()),*]
    }};
}
