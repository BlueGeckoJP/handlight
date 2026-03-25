pub fn get_new_adjustment_value(
    row_y: f64,
    row_height: f64,
    view_value: f64,
    view_page_size: f64,
) -> f64 {
    let row_top = row_y;
    let row_bottom = row_top + row_height;
    let view_bottom = view_value + view_page_size;

    if row_top < view_value {
        row_top
    } else if row_bottom > view_bottom {
        row_bottom - view_page_size
    } else {
        view_value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scroll_down() {
        // Row is below viewport (viewport: 0-100, row: 110-130)
        assert_eq!(get_new_adjustment_value(110.0, 20.0, 0.0, 100.0), 30.0);
    }

    #[test]
    fn test_scroll_up() {
        // Row is above viewport (viewport: 100-200, row: 50-70)
        assert_eq!(get_new_adjustment_value(50.0, 20.0, 100.0, 100.0), 50.0);
    }

    #[test]
    fn test_no_scroll() {
        // Row is inside viewport (viewport: 0-100, row: 10-30)
        assert_eq!(get_new_adjustment_value(10.0, 20.0, 0.0, 100.0), 0.0);
    }
}
