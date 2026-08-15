use crate::TypeSize;

impl TypeSize for url::Url {
    fn extra_size(&self) -> usize {
        self.as_str().len()
    }
}
