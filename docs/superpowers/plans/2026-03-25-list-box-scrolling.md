# List Box Scrolling Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Implement minimal scrolling for the `ListBox` when navigating with keyboard arrows.

**Architecture:** Use GTK signal handlers (`row-selected`) to trigger a helper function that calculates if the selected row is out of the viewport and adjusts the `ScrolledWindow`'s vertical adjustment accordingly.

**Tech Stack:** Rust, GTK4, Relm4

---

### Task 1: Create Helper Function and Unit Tests

**Files:**
- Create: `src/scrolling.rs`
- Modify: `src/main.rs:1-1`

- [ ] **Step 1: Create `src/scrolling.rs` with the logic and tests**

```rust
use gtk4::prelude::*;

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
```

- [ ] **Step 2: Run tests to verify they pass**

Run: `cargo test scrolling`
Expected: 3 passed

- [ ] **Step 3: Add `mod scrolling;` to `src/main.rs`**

Modify `src/main.rs:1`:
```rust
mod app_info;
mod scrolling;
```

- [ ] **Step 4: Commit**

```bash
git add src/scrolling.rs src/main.rs
git commit -m "feat: add scrolling logic and tests"
```

### Task 2: Integrate Scrolling into the View

**Files:**
- Modify: `src/main.rs:144-157, 284-288`

- [ ] **Step 1: Initialize `vadjustment` in `App::init`**

Modify `src/main.rs:284-288`:
```rust
        // Select the first row by default
        if let Some(row) = list_box.row_at_index(0) {
            list_box.select_row(Some(&row));
        }

        let vadjustment = gtk4::Adjustment::new(0.0, 0.0, 0.0, 0.0, 0.0, 0.0);

        let widgets = view_output!();
```

- [ ] **Step 2: Apply `vadjustment` and implement scrolling in `view!` macro**

Modify `src/main.rs:144-157`:
```rust
                    #[wrap(Some)]
                    set_start_child = &gtk4::ScrolledWindow {
                        set_policy: (gtk4::PolicyType::Never, gtk4::PolicyType::Automatic),
                        set_vadjustment: Some(&vadjustment),
                        #[local_ref]
                        list_box -> gtk4::ListBox {
                            set_selection_mode: gtk4::SelectionMode::Single,
                            set_activate_on_single_click: false,
                            connect_row_activated[sender] => move |_, row| {
                                sender.input(Msg::AppClicked(row.index()));
                            },
                            connect_row_selected[sender, vadjustment] => move |_, row| {
                                if let Some(row) = row {
                                    let allocation = row.allocation();
                                    let new_value = crate::scrolling::get_new_adjustment_value(
                                        allocation.y() as f64,
                                        allocation.height() as f64,
                                        vadjustment.value(),
                                        vadjustment.page_size(),
                                    );
                                    vadjustment.set_value(new_value);
                                    sender.input(Msg::RowSelected(Some(row.index())));
                                } else {
                                    sender.input(Msg::RowSelected(None));
                                }
                            }
                        }
                    },
```

- [ ] **Step 3: Build and run the application to manually verify**

Run: `cargo run`
Expected: Scrolling works correctly (minimal scrolling) when navigating items with arrow keys.

- [ ] **Step 4: Commit**

```bash
git add src/main.rs
git commit -m "feat: integrate minimal scrolling into list box"
```
