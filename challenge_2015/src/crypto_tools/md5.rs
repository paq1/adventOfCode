pub trait MD5 {
    fn md5(&self) -> String;
}

impl MD5 for String {
    fn md5(&self) -> String {
        let res = md5::compute(self);
        format!("{:x}", res)
    }
}