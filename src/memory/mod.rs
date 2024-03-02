use std::str::SplitWhitespace;

#[derive(Debug)]
pub struct FreeOutput {
    non_swap_row: NonSwapRow,
    swap_row: SwapRow,
}
impl FreeOutput {
    pub fn from_free_command(free_output: String) -> FreeOutput {
        let mut free_output_lines = free_output.lines();

        let header_row = free_output_lines.next().unwrap().split_whitespace();
        Self::validate_header_row(header_row);

        let mut non_swap_row_iter = free_output_lines.next().unwrap().split_whitespace();
        non_swap_row_iter.next();  // Mem:

        let total = Self::get_next_cell(&mut non_swap_row_iter);
        let used = Self::get_next_cell(&mut non_swap_row_iter);
        let free = Self::get_next_cell(&mut non_swap_row_iter);
        let shared = Self::get_next_cell(&mut non_swap_row_iter);
        let buffers = Self::get_next_cell(&mut non_swap_row_iter);
        let cache = Self::get_next_cell(&mut non_swap_row_iter);
        let available = Self::get_next_cell(&mut non_swap_row_iter);

        let non_swap_row = NonSwapRow {
            total,
            used,
            free,
            shared,
            buffers,
            cache,
            available
        };

        let mut swap_row_iter = free_output_lines.next().unwrap().split_whitespace();
        swap_row_iter.next();  // Swap:

        let swap_total = Self::get_next_cell(&mut swap_row_iter);
        let swap_used = Self::get_next_cell(&mut swap_row_iter);
        let swap_free = Self::get_next_cell(&mut swap_row_iter);

        let swap_row = SwapRow {
            swap_total,
            swap_used,
            swap_free
        };

        FreeOutput {
            non_swap_row,
            swap_row,
        }
    }

    fn validate_header_row(mut header_row: SplitWhitespace) {
        let total_header = header_row.next().unwrap();
        assert!(total_header.eq("total"));

        let used_header = header_row.next().unwrap();
        assert!(used_header.eq("used"));

        let free_header = header_row.next().unwrap();
        assert!(free_header.eq("free"));

        let shared_header = header_row.next().unwrap();
        assert!(shared_header.eq("shared"));

        let buffer_header = header_row.next().unwrap();
        assert!(buffer_header.eq("buffers"));

        let cache_header = header_row.next().unwrap();
        assert!(cache_header.eq("cache"));

        let available_header = header_row.next().unwrap();
        assert!(available_header.eq("available"));
    }

    fn get_next_cell(row: &mut SplitWhitespace) -> u64 {
        row.next().unwrap().parse::<u64>().unwrap()
    }

    pub fn print(&self) {
        &self.non_swap_row;
    }
}

#[derive(Debug)]
struct NonSwapRow {
    total: u64,
    used: u64,
    free: u64,
    shared: u64,
    buffers: u64,
    cache: u64,
    available: u64,
}


#[derive(Debug)]
struct SwapRow {
    swap_total: u64,
    swap_used: u64,
    swap_free: u64,
}