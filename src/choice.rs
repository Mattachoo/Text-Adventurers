pub trait Choice {
    fn describe(&self) -> String;
}

pub trait ConstantChoice {
    fn describe_str(&self) -> &str;
}

impl<T> Choice for T
where
    T: ConstantChoice,
{
    fn describe(&self) -> String {
        String::from(self.describe_str())
    }
}
