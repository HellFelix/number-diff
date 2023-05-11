use crate::Elementary::{self, *};

impl Elementary {
    pub fn test_from<'a>(value: &'a String) -> Vec<&'a str> {
        Self::split_function(value)
    }

    fn split_function<'a>(value: &'a String) -> Vec<&'a str> {
        let mut interp_slice: Vec<&str> = value.split("").collect();
        // remove the first and last element because they are just empty string slices
        interp_slice.remove(0);
        interp_slice.pop();

        println!("{}, {}", interp_slice.len(), value.len());

        let mut chunks: Vec<&str> = Vec::new();
        let mut open_parenthesis = -1;

        let mut cut_index = 0;
        for i in 0..interp_slice.len() {
            if interp_slice[i] == "(" {
                // this is for the first case of an opening parenthesis. Note that we cannot start
                // at 0 since that would match the case for closing an outer parenthesis
                if open_parenthesis == -1 {
                    open_parenthesis = 1;
                } else {
                    // for all other cases, however, the number of open parentheses just goes up by
                    // one
                    open_parenthesis += 1;
                }
            } else if interp_slice[i] == ")" {
                open_parenthesis -= 1
            }

            // check if outer parenthesis has been closed
            if open_parenthesis == 0 {
                chunks.push(&value[cut_index..=i]);

                // set new cut index
                cut_index = i + 1;

                // reset parenthesis
                open_parenthesis = -1;
            }

            // detect operations
            if open_parenthesis == -1
                && (interp_slice[i] == "+"
                    || interp_slice[i] == "-"
                    || interp_slice[i] == "*"
                    || interp_slice[i] == "/"
                    || interp_slice[i] == "^")
            {
                chunks.push(interp_slice[i]);
                cut_index = i + 1;
            }
        }

        chunks
    }

    fn to_elementary<'a>(strings: Vec<&'a str>) -> Self {
        let mut elements: Vec<Self> = Vec::new();
        // order of operations

        unimplemented!()
    }
}
