use std::{collections::HashMap, path::PathBuf};

type IndexMap<T> = HashMap<usize, T>;

pub type Files = IndexMap<PathBuf>;
