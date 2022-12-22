use std::cmp::Ordering;
use std::io;
// we can use nested paths to bring the same items into scope in one line.
use std::{cmp::Ordering, io};

use std::io;
use std::io::Write;
// To merge these two paths into one use statement, we can use self in the nested path
use std::io::{self, Write};

// If we want to bring all public items defined in a path into scope, we can specify that path followed by the * glob operator
use std::collections::*;
