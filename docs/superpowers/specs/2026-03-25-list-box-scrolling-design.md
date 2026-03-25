# List Box Scrolling Design (Revised)

Implement minimal scrolling for the `ListBox` in the application launcher. When the user navigates using the arrow keys or the selection changes, the `ScrolledWindow` should update its vertical position to ensure the selected item remains visible.

## Architecture

The application uses `relm4` and `gtk4`. The `ListBox` is contained within a `ScrolledWindow`. We will use GTK signal handlers to perform UI-side side effects (scrolling) without polluting the `App` model with widget references.

## Design Details

### 1. Widget Signal Handling
In the `view!` macro, we will enhance the `connect_row_selected` signal of the `ListBox`. This signal triggers whenever the selection changes, whether by keyboard navigation or programmatic selection.

### 2. Scrolling Logic
The logic will be encapsulated in a helper function to maintain isolation and testability.

**Helper Function: `ensure_row_visible(list_box: &ListBox, row: &ListBoxRow, adjustment: &Adjustment)`**
1.  Calculate row boundaries:
    -   `row_top = row.allocation().y()`
    -   `row_bottom = row_top + row.allocation().height()`
2.  Get current adjustment values:
    -   `view_top = adjustment.value()`
    -   `view_bottom = view_top + adjustment.page_size()`
3.  Perform minimal scroll:
    -   If `row_top < view_top`, set `adjustment.set_value(row_top)`.
    -   Else if `row_bottom > view_bottom`, set `adjustment.set_value(row_bottom - adjustment.page_size())`.

### 3. Integration in `view!`
```rust
#[local_ref]
list_box -> gtk4::ListBox {
    // ...
    connect_row_selected[sender, vadjustment_ref] => move |list_box, row| {
        if let Some(row) = row {
            ensure_row_visible(list_box, row, &vadjustment_ref);
            sender.input(Msg::RowSelected(Some(row.index())));
        } else {
            sender.input(Msg::RowSelected(None));
        }
    }
}
```
*Note: `vadjustment_ref` will be a reference to the `ScrolledWindow`'s vertical adjustment, captured by the closure.*

### 4. Edge Cases
-   **Search Results Reset:** When the search entry changes, the model resets the selection to index 0. This triggers `row-selected`, which will correctly scroll to the top.
-   **Header Height:** `row.allocation().y()` in GTK4 is relative to the `ListBox`. It includes the height of any headers preceding the row, which is exactly what we need.

## Testing Strategy

1.  **Unit Testing:** Implement a testable version of `ensure_row_visible` (or a pure function that calculates the new adjustment value given row/view bounds) and verify it returns correct values for various scenarios (row above, row below, row visible).
2.  **Manual Verification:**
    -   Verify scrolling works when navigating with arrow keys.
    -   Verify scrolling works when the search result changes (reset to top).
    -   Verify the list box doesn't jump unnecessarily when the selected item is already visible.
