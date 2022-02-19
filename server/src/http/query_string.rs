use std::collections::HashMap;

#[derive(Debug)]
pub struct QueryString<'buf> {
    data: HashMap<&'buf str, Value<'buf>>
}

impl<'buf> QueryString<'buf> {
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.data.get(key)
    }
}

#[derive(Debug)]
pub enum Value<'buf> {
    Single(&'buf str),
    Multiple(Vec<&'buf str>),
}

impl<'buf> From<&'buf str> for QueryString<'buf> {
    fn from(s: &'buf str) -> Self {
        let mut data = HashMap::new();

        for sub_str in s.split('&') {
            let (mut key, mut value) = (sub_str, "");

            // get key and value
            if let Some(i) = sub_str.find('=') {
                key = &sub_str[..i];
                value = &sub_str[i+1..];
            }

            data.entry(key)
                .and_modify(|existing: &mut Value| match existing {
                    Value::Single(prev_val) => {
                        *existing = Value::Multiple(vec![prev_val, value]);
                    },
                    Value::Multiple(vector) => vector.push(value),
                })
                .or_insert(Value::Single(value));
        }

        QueryString { data }
    }
}
